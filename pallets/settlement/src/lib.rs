// This file is part of the Polymesh distribution (https://github.com/PolymeshAssociation/Polymesh).
// Copyright (c) 2020 Polymesh Association

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.

// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

//! # Settlement Module
//!
//! Settlement module manages all kinds of transfers and settlements of assets
//!
//! ## Overview
//!
//! The settlement module provides functionality to settle onchain as well as offchain trades between multiple parties.
//! All trades are settled under venues. An appropriately permissioned external agent
//! can allow/block certain venues from settling trades that involve their tokens.
//! An atomic settlement is called an Instruction. An instruction can contain multiple legs. Legs are essentially simple one to one transfers.
//! When an instruction is settled, either all legs are executed successfully or none are. In other words, if one of the leg fails due to
//! compliance failure, all other legs will also fail.
//!
//! An instruction must be authorized by all the counter parties involved for it to be executed.
//! An instruction can be set to automatically execute in the next block when all authorizations are received or at a particular block number.
//!
//! Offchain settlements are represented via receipts. If a leg has a receipt attached to it, it will not be executed onchain.
//! All other legs will be executed onchain during settlement.
//!
//! ## Dispatchable Functions
//!
//! - `create_venue` - Registers a new venue.
//! - `add_instruction` - Adds a new instruction.
//! - `affirm_instruction` - Affirms an existing instruction.
//! - `withdraw_affirmation` - Withdraw an existing affirmation to the given instruction.
//! - `reject_instruction` - Rejects an existing instruction.
//! - `set_venue_filtering` - Enables or disabled venue filtering for a token.
//! - `allow_venues` - Allows additional venues to create instructions involving an asset.
//! - `disallow_venues` - Revokes permission given to venues for creating instructions involving a particular asset.

#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;
mod migrations;

use codec::{Decode, Encode};
use frame_support::dispatch::{
    DispatchError, DispatchErrorWithPostInfo, DispatchResult, DispatchResultWithPostInfo,
    PostDispatchInfo,
};
use frame_support::traits::schedule::{DispatchTime, Named};
use frame_support::traits::Get;
use frame_support::weights::Weight;
use frame_support::{
    decl_error, decl_module, decl_storage, ensure, BoundedBTreeSet, IterableStorageDoubleMap,
};
use frame_system::{ensure_root, RawOrigin};
use sp_runtime::traits::{One, Verify};
use sp_std::collections::btree_set::BTreeSet;
use sp_std::convert::TryFrom;
use sp_std::prelude::*;
use sp_std::vec;

use pallet_asset::MandatoryMediators;
use pallet_base::{ensure_string_limited, try_next_post};
use polymesh_common_utilities::constants::queue_priority::SETTLEMENT_INSTRUCTION_EXECUTION_PRIORITY;
use polymesh_common_utilities::traits::portfolio::PortfolioSubTrait;
pub use polymesh_common_utilities::traits::settlement::{Event, RawEvent, WeightInfo};
use polymesh_common_utilities::traits::{asset, compliance_manager, identity, nft, CommonConfig};
use polymesh_common_utilities::with_transaction;
use polymesh_common_utilities::SystematicIssuers::Settlement as SettlementDID;
use polymesh_primitives::asset::AssetId;
use polymesh_primitives::settlement::{
    AffirmationCount, AffirmationStatus, AssetCount, ExecuteInstructionInfo, FilteredLegs,
    Instruction, InstructionId, InstructionInfo, InstructionStatus, Leg, LegId, LegStatus,
    MediatorAffirmationStatus, Receipt, ReceiptDetails, SettlementType, Venue, VenueDetails,
    VenueId, VenueType,
};
use polymesh_primitives::{
    storage_migrate_on, storage_migration_ver, Balance, IdentityId, Memo, NFTs, PortfolioId,
    SecondaryKey, WeightMeter,
};

type Identity<T> = pallet_identity::Module<T>;
type System<T> = frame_system::Pallet<T>;
type Asset<T> = pallet_asset::Module<T>;
type ExternalAgents<T> = pallet_external_agents::Module<T>;
type Nft<T> = pallet_nft::Module<T>;
type EnsureValidInstructionResult<AccountId, Moment, BlockNumber> = Result<
    (
        IdentityId,
        Option<SecondaryKey<AccountId>>,
        Instruction<Moment, BlockNumber>,
    ),
    DispatchError,
>;

pub trait Config:
    asset::Config
    + CommonConfig
    + compliance_manager::Config
    + frame_system::Config
    + identity::Config
    + nft::Config
    + pallet_timestamp::Config
{
    /// The overarching event type.
    type RuntimeEvent: From<Event<Self>> + Into<<Self as frame_system::Config>::RuntimeEvent>;

    /// A call type used by the scheduler.
    type Proposal: From<Call<Self>> + Into<<Self as identity::Config>::Proposal>;

    /// Scheduler of settlement instructions.
    type Scheduler: Named<Self::BlockNumber, <Self as Config>::Proposal, Self::SchedulerOrigin>;

    /// Maximum number of fungible assets that can be in a single instruction.
    type MaxNumberOfFungibleAssets: Get<u32>;

    /// Weight information for extrinsic of the settlement pallet.
    type WeightInfo: WeightInfo;

    /// Maximum number of NFTs that can be transferred in a leg.
    type MaxNumberOfNFTsPerLeg: Get<u32>;

    /// Maximum number of NFTs that can be transferred in a instruction.
    type MaxNumberOfNFTs: Get<u32>;

    /// Maximum number of off-chain assets that can be transferred in a instruction.
    type MaxNumberOfOffChainAssets: Get<u32>;

    /// Maximum number of portfolios.
    type MaxNumberOfPortfolios: Get<u32>;

    /// Maximum number of venue signers.
    type MaxNumberOfVenueSigners: Get<u32>;

    /// Maximum number mediators in the instruction level (this does not include asset mediators).
    type MaxInstructionMediators: Get<u32>;
}

decl_error! {
    /// Errors for the Settlement module.
    pub enum Error for Module<T: Config> {
        /// Venue does not exist.
        InvalidVenue,
        /// Sender does not have required permissions.
        Unauthorized,
        /// Instruction has not been affirmed.
        InstructionNotAffirmed,
        /// Signer is not authorized by the venue.
        UnauthorizedSigner,
        /// Receipt already used.
        ReceiptAlreadyClaimed,
        /// Venue does not have required permissions.
        UnauthorizedVenue,
        /// Instruction has invalid dates
        InstructionDatesInvalid,
        /// Instruction's target settle block reached.
        InstructionSettleBlockPassed,
        /// Offchain signature is invalid.
        InvalidSignature,
        /// Sender and receiver are the same.
        SameSenderReceiver,
        /// The provided settlement block number is in the past and cannot be used by the scheduler.
        SettleOnPastBlock,
        /// The current instruction affirmation status does not support the requested action.
        UnexpectedAffirmationStatus,
        /// Scheduling of an instruction fails.
        FailedToSchedule,
        /// Instruction status is unknown
        UnknownInstruction,
        /// Signer is already added to venue.
        SignerAlreadyExists,
        /// Signer is not added to venue.
        SignerDoesNotExist,
        /// Instruction leg amount can't be zero.
        ZeroAmount,
        /// Instruction settlement block has not yet been reached.
        InstructionSettleBlockNotReached,
        /// The caller is not a party of this instruction.
        CallerIsNotAParty,
        /// The number of nfts being transferred in the instruction was exceeded.
        MaxNumberOfNFTsExceeded,
        /// The given number of nfts being transferred was underestimated.
        NumberOfTransferredNFTsUnderestimated,
        /// Off-chain receipts can only be used for off-chain leg type.
        ReceiptForInvalidLegType,
        /// The maximum weight limit for executing the function was exceeded.
        WeightLimitExceeded,
        /// The maximum number of fungible assets was exceeded.
        MaxNumberOfFungibleAssetsExceeded,
        /// The maximum number of off-chain assets was exceeded.
        MaxNumberOfOffChainAssetsExceeded,
        /// The given number of fungible transfers was underestimated.
        NumberOfFungibleTransfersUnderestimated,
        /// AssetId could not be found on chain.
        UnexpectedOFFChainAsset,
        /// Off-Chain assets cannot be locked.
        OffChainAssetCantBeLocked,
        /// The given number of off-chain transfers was underestimated.
        NumberOfOffChainTransfersUnderestimated,
        /// No leg with the given id was found
        LegNotFound,
        /// The input weight is less than the minimum required.
        InputWeightIsLessThanMinimum,
        /// The maximum number of receipts was exceeded.
        MaxNumberOfReceiptsExceeded,
        /// There are parties who have not affirmed the instruction.
        NotAllAffirmationsHaveBeenReceived,
        /// Only [`InstructionStatus::Pending`] or [`InstructionStatus::Failed`] instructions can be executed.
        InvalidInstructionStatusForExecution,
        /// The instruction failed to release asset locks or transfer the assets.
        FailedToReleaseLockOrTransferAssets,
        /// No duplicate uid are allowed for different receipts.
        DuplicateReceiptUid,
        /// The instruction id in all receipts must match the extrinsic parameter.
        ReceiptInstructionIdMissmatch,
        /// Multiple receipts for the same leg are not allowed.
        MultipleReceiptsForOneLeg,
        /// An invalid has been reached.
        UnexpectedLegStatus,
        /// The maximum number of venue signers was exceeded.
        NumberOfVenueSignersExceeded,
        /// The caller is not a mediator in the instruction.
        CallerIsNotAMediator,
        /// The mediator's expiry date must be in the future.
        InvalidExpiryDate,
        /// The expiry date for the mediator's affirmation has passed.
        MediatorAffirmationExpired,
        /// Offchain assets must have a venue.
        OffChainAssetsMustHaveAVenue,
    }
}

storage_migration_ver!(3);

decl_storage! {
    trait Store for Module<T: Config> as Settlement {
        /// Info about a venue. venue_id -> venue
        pub VenueInfo get(fn venue_info): map hasher(twox_64_concat) VenueId => Option<Venue>;

        /// Free-form text about a venue. venue_id -> `VenueDetails`
        /// Only needed for the UI.
        pub Details get(fn details): map hasher(twox_64_concat) VenueId => VenueDetails;

        /// Instructions under a venue.
        /// Only needed for the UI.
        ///
        /// venue_id -> instruction_id -> ()
        pub VenueInstructions get(fn venue_instructions):
            double_map hasher(twox_64_concat) VenueId, hasher(twox_64_concat) InstructionId => ();

        /// Signers allowed by the venue. (venue_id, signer) -> bool
        VenueSigners get(fn venue_signers):
            double_map hasher(twox_64_concat) VenueId, hasher(twox_64_concat) T::AccountId => bool;
        /// Venues create by an identity.
        /// Only needed for the UI.
        ///
        /// identity -> venue_id -> ()
        pub UserVenues get(fn user_venues):
            double_map hasher(twox_64_concat) IdentityId, hasher(twox_64_concat) VenueId => ();
        /// Details about an instruction. instruction_id -> instruction_details
        pub InstructionDetails get(fn instruction_details):
            map hasher(twox_64_concat) InstructionId => Instruction<T::Moment, T::BlockNumber>;
        /// Status of a leg under an instruction. (instruction_id, leg_id) -> LegStatus
        pub InstructionLegStatus get(fn instruction_leg_status):
            double_map hasher(twox_64_concat) InstructionId, hasher(twox_64_concat) LegId => LegStatus<T::AccountId>;
        /// Number of affirmations pending before instruction is executed. instruction_id -> affirm_pending
        pub InstructionAffirmsPending get(fn instruction_affirms_pending): map hasher(twox_64_concat) InstructionId => u64;
        /// Tracks affirmations received for an instruction. (instruction_id, counter_party) -> AffirmationStatus
        pub AffirmsReceived get(fn affirms_received):
            double_map hasher(twox_64_concat) InstructionId, hasher(twox_64_concat) PortfolioId => AffirmationStatus;
        /// Helps a user track their pending instructions and affirmations (only needed for UI).
        /// (counter_party, instruction_id) -> AffirmationStatus
        pub UserAffirmations get(fn user_affirmations):
            double_map hasher(twox_64_concat) PortfolioId, hasher(twox_64_concat) InstructionId => AffirmationStatus;
        /// Tracks redemption of receipts. (signer, receipt_uid) -> receipt_used
        ReceiptsUsed get(fn receipts_used): double_map hasher(twox_64_concat) T::AccountId, hasher(blake2_128_concat) u64 => bool;
        /// Tracks if a token has enabled filtering venues that can create instructions involving their token. AssetId -> filtering_enabled
        VenueFiltering get(fn venue_filtering): map hasher(blake2_128_concat) AssetId => bool;
        /// Venues that are allowed to create instructions involving a particular asset. Only used if filtering is enabled.
        /// ([`AssetId`], venue_id) -> allowed
        VenueAllowList get(fn venue_allow_list): double_map hasher(blake2_128_concat) AssetId, hasher(twox_64_concat) VenueId => bool;
        /// Number of venues in the system (It's one more than the actual number)
        VenueCounter get(fn venue_counter) build(|_| VenueId(1u64)): VenueId;
        /// Number of instructions in the system (It's one more than the actual number)
        InstructionCounter get(fn instruction_counter) build(|_| InstructionId(1u64)): InstructionId;
        /// Instruction memo
        pub InstructionMemos get(fn memo): map hasher(twox_64_concat) InstructionId => Option<Memo>;
        /// Instruction statuses. instruction_id -> InstructionStatus
        pub InstructionStatuses get(fn instruction_status):
            map hasher(twox_64_concat) InstructionId => InstructionStatus<T::BlockNumber>;
        /// Legs under an instruction. (instruction_id, leg_id) -> Leg
        pub InstructionLegs get(fn instruction_legs):
            double_map hasher(twox_64_concat) InstructionId, hasher(twox_64_concat) LegId => Option<Leg>;
        /// Tracks the affirmation status for offchain legs in a instruction. [`(InstructionId, LegId)`] -> [`AffirmationStatus`]
        pub OffChainAffirmations get(fn offchain_affirmations):
            double_map hasher(twox_64_concat) InstructionId, hasher(twox_64_concat) LegId => AffirmationStatus;
        /// Tracks the number of signers each venue has.
        pub NumberOfVenueSigners get(fn number_of_venue_signers): map hasher(twox_64_concat) VenueId => u32;
        /// The status for the mediators affirmation.
        pub InstructionMediatorsAffirmations get(fn venue_mediators_affirmations):
            double_map hasher(twox_64_concat) InstructionId, hasher(identity) IdentityId => MediatorAffirmationStatus<T::Moment>;
        /// Storage version.
        StorageVersion get(fn storage_version) build(|_| Version::new(3)): Version;
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: <T as frame_system::Config>::RuntimeOrigin {
        type Error = Error<T>;

        const MaxNumberOfOffChainAssets: u32 = T::MaxNumberOfOffChainAssets::get();
        const MaxNumberOfFungibleAssets: u32 = T::MaxNumberOfFungibleAssets::get();
        const MaxNumberOfNFTsPerLeg: u32 = T::MaxNumberOfNFTsPerLeg::get();
        const MaxNumberOfNFTs: u32 = T::MaxNumberOfNFTs::get();
        const MaxNumberOfPortfolios: u32 = T::MaxNumberOfPortfolios::get();
        const MaxNumberOfVenueSigners: u32 = T::MaxNumberOfVenueSigners::get();

        fn deposit_event() = default;

        fn on_runtime_upgrade() -> Weight {
            storage_migrate_on!(StorageVersion, 3, {
                migrations::migrate_to_v3::<T>();
            });
            Weight::zero()
        }

        /// Registers a new venue.
        ///
        /// * `details` - Extra details about a venue
        /// * `signers` - Array of signers that are allowed to sign receipts for this venue
        /// * `typ` - Type of venue being created
        #[weight = <T as Config>::WeightInfo::create_venue(details.len() as u32, signers.len() as u32)]
        pub fn create_venue(origin, details: VenueDetails, signers: Vec<T::AccountId>, typ: VenueType) {
            // Ensure permissions and details limit.
            let did = Identity::<T>::ensure_perms(origin)?;
            ensure_string_limited::<T>(&details)?;

            ensure!(
                signers.len() <= T::MaxNumberOfVenueSigners::get() as usize,
                Error::<T>::NumberOfVenueSignersExceeded
            );

            // Advance venue counter.
            // NB: Venue counter starts with 1.
            let id = VenueCounter::try_mutate(try_next_post::<T, _>)?;

            // Other commits to storage + emit event.
            let venue = Venue { creator: did, venue_type: typ };
            VenueInfo::insert(id, venue);
            Details::insert(id, details.clone());
            NumberOfVenueSigners::insert(id, signers.len() as u32);
            for signer in signers {
                <VenueSigners<T>>::insert(id, signer, true);
            }
            UserVenues::insert(did, id, ());
            Self::deposit_event(RawEvent::VenueCreated(did, id, details, typ));
        }

        /// Edit a venue's details.
        ///
        /// * `id` specifies the ID of the venue to edit.
        /// * `details` specifies the updated venue details.
        #[weight = <T as Config>::WeightInfo::update_venue_details(details.len() as u32)]
        pub fn update_venue_details(origin, id: VenueId, details: VenueDetails) -> DispatchResult {
            ensure_string_limited::<T>(&details)?;
            let did = Identity::<T>::ensure_perms(origin)?;
            Self::venue_for_management(id, did)?;

            // Commit to storage.
            Details::insert(id, details.clone());
            Self::deposit_event(RawEvent::VenueDetailsUpdated(did, id, details));
            Ok(())
        }

        /// Edit a venue's type.
        ///
        /// * `id` specifies the ID of the venue to edit.
        /// * `type` specifies the new type of the venue.
        #[weight = <T as Config>::WeightInfo::update_venue_type()]
        pub fn update_venue_type(origin, id: VenueId, typ: VenueType) -> DispatchResult {
            let did = Identity::<T>::ensure_perms(origin)?;

            let mut venue = Self::venue_for_management(id, did)?;
            venue.venue_type = typ;
            VenueInfo::insert(id, venue);

            Self::deposit_event(RawEvent::VenueTypeUpdated(did, id, typ));
            Ok(())
        }

        /// Affirms an instruction using receipts for offchain transfers.
        ///
        /// # Arguments
        /// * `id` - the [`InstructionId`] of the instruction being affirmed.
        /// * `receipt_details` - a vector of [`ReceiptDetails`], which contain the details about the offchain transfer.
        /// * `portfolios` - a vector of [`PortfolioId`] under the caller's control and intended for affirmation.
        ///
        /// # Permissions
        /// * Portfolio
        #[weight = <T as Config>::WeightInfo::affirm_with_receipts_input(None, portfolios.len() as u32)]
        pub fn affirm_with_receipts(
            origin,
            id: InstructionId,
            receipt_details: Vec<ReceiptDetails<T::AccountId, T::OffChainSignature>>,
            portfolios: BoundedBTreeSet<PortfolioId, T::MaxNumberOfPortfolios>,
        ) -> DispatchResultWithPostInfo {
            Self::affirm_with_receipts_and_maybe_schedule_instruction(
                origin,
                id,
                receipt_details,
                portfolios.into_inner(),
                None
            )
        }

        /// Enables or disabled venue filtering for a token.
        ///
        /// # Arguments
        /// * `asset_id` - AssetId of the token in question.
        /// * `enabled` - Boolean that decides if the filtering should be enabled.
        ///
        /// # Permissions
        /// * Asset
        #[weight = <T as Config>::WeightInfo::set_venue_filtering()]
        pub fn set_venue_filtering(origin, asset_id: AssetId, enabled: bool) {
            let did = <ExternalAgents<T>>::ensure_perms(origin, asset_id)?;
            if enabled {
                VenueFiltering::insert(asset_id, enabled);
            } else {
                VenueFiltering::remove(asset_id);
            }
            Self::deposit_event(RawEvent::VenueFiltering(did, asset_id, enabled));
        }

        /// Allows additional venues to create instructions involving an asset.
        ///
        /// * `asset_id` - AssetId of the token in question.
        /// * `venues` - Array of venues that are allowed to create instructions for the token in question.
        ///
        /// # Permissions
        /// * Asset
        #[weight = <T as Config>::WeightInfo::allow_venues(venues.len() as u32)]
        pub fn allow_venues(origin, asset_id: AssetId, venues: Vec<VenueId>) {
            let did = <ExternalAgents<T>>::ensure_perms(origin, asset_id)?;
            for venue in &venues {
                VenueAllowList::insert(&asset_id, venue, true);
            }
            Self::deposit_event(RawEvent::VenuesAllowed(did, asset_id, venues));
        }

        /// Revokes permission given to venues for creating instructions involving a particular asset.
        ///
        /// * `asset_id` - AssetId of the token in question.
        /// * `venues` - Array of venues that are no longer allowed to create instructions for the token in question.
        ///
        /// # Permissions
        /// * Asset
        #[weight = <T as Config>::WeightInfo::disallow_venues(venues.len() as u32)]
        pub fn disallow_venues(origin, asset_id: AssetId, venues: Vec<VenueId>) {
            let did = <ExternalAgents<T>>::ensure_perms(origin, asset_id)?;
            for venue in &venues {
                VenueAllowList::remove(&asset_id, venue);
            }
            Self::deposit_event(RawEvent::VenuesBlocked(did, asset_id, venues));
        }

        /// Edit a venue's signers.
        /// * `id` specifies the ID of the venue to edit.
        /// * `signers` specifies the signers to add/remove.
        /// * `add_signers` specifies the update type add/remove of venue where add is true and remove is false.
        #[weight = <T as Config>::WeightInfo::update_venue_signers(signers.len() as u32)]
        pub fn update_venue_signers(origin, id: VenueId, signers: Vec<T::AccountId>, add_signers: bool) {
            let did = Identity::<T>::ensure_perms(origin)?;

            Self::base_update_venue_signers(did, id, signers, add_signers)?;
        }

        /// Manually executes an instruction.
        ///
        /// # Arguments
        /// * `id`: The [`InstructionId`] of the instruction to be executed.
        /// * `portfolio`:  One of the caller's [`PortfolioId`] which is also a counter patry in the instruction.
        /// If None, the caller must be the venue creator or a counter party in a [`Leg::OffChain`].
        /// * `fungible_transfers`: The number of fungible legs in the instruction.
        /// * `nfts_transfers`: The number of nfts being transferred in the instruction.
        /// * `offchain_transfers`: The number of offchain legs in the instruction.
        /// * `weight_limit`: An optional maximum [`Weight`] value to be charged for executing the instruction.
        /// If the `weight_limit` is less than the required amount, the instruction will fail execution.
        ///
        /// Note: calling the rpc method `get_execute_instruction_info` returns an instance of [`ExecuteInstructionInfo`], which contains the count parameters.
        #[weight = <T as Config>::WeightInfo::execute_manual_weight_limit(weight_limit, fungible_transfers, nfts_transfers, offchain_transfers)]
        pub fn execute_manual_instruction(
            origin,
            id: InstructionId,
            portfolio: Option<PortfolioId>,
            fungible_transfers: u32,
            nfts_transfers: u32,
            offchain_transfers: u32,
            weight_limit: Option<Weight>
        ) -> DispatchResultWithPostInfo {
            let mut weight_meter = Self::ensure_valid_weight_meter(
                Self::execute_manual_instruction_minimum_weight(),
                weight_limit.unwrap_or(Self::execute_manual_instruction_weight_limit(
                    fungible_transfers,
                    nfts_transfers,
                    offchain_transfers,
                )),
            )?;
            let input_cost = AssetCount::new(fungible_transfers, nfts_transfers, offchain_transfers);
            Self::base_execute_manual_instruction(origin, id, portfolio, &input_cost, &mut weight_meter)
                .map_err(|e| DispatchErrorWithPostInfo {
                    post_info: Some(weight_meter.consumed()).into(),
                    error: e.error,
                })
        }

        /// Adds a new instruction.
        ///
        /// # Arguments
        /// * `venue_id`: The optional [`VenueId`] of the venue this instruction belongs to.
        /// * `settlement_type`: The [`SettlementType`] specifying when the instruction should be settled.
        /// * `trade_date`: Optional date from which people can interact with this instruction.
        /// * `value_date`: Optional date after which the instruction should be settled (not enforced).
        /// * `legs`: A vector of all [`Leg`] included in this instruction.
        /// * `memo`: An optional [`Memo`] field for this instruction.
        #[weight = <T as Config>::WeightInfo::add_instruction_legs(legs)]
        pub fn add_instruction(
            origin,
            venue_id: Option<VenueId>,
            settlement_type: SettlementType<T::BlockNumber>,
            trade_date: Option<T::Moment>,
            value_date: Option<T::Moment>,
            legs: Vec<Leg>,
            instruction_memo: Option<Memo>,
        ) {
            let did = Identity::<T>::ensure_perms(origin)?;
            Self::base_add_instruction(
                did,
                venue_id,
                settlement_type,
                trade_date,
                value_date,
                legs,
                instruction_memo,
                None
            )?;
        }

        /// Adds and affirms a new instruction.
        ///
        /// # Arguments
        /// * `venue_id`: The [`VenueId`] of the venue this instruction belongs to.
        /// * `settlement_type`: The [`SettlementType`] specifying when the instruction should be settled.
        /// * `trade_date`: Optional date from which people can interact with this instruction.
        /// * `value_date`: Optional date after which the instruction should be settled (not enforced).
        /// * `legs`: A vector of all [`Leg`] included in this instruction.
        /// * `portfolios`: A vector of [`PortfolioId`] under the caller's control and intended for affirmation.
        /// * `memo`: An optional [`Memo`] field for this instruction.
        ///
        /// # Permissions
        /// * Portfolio
        #[weight = <T as Config>::WeightInfo::add_and_affirm_instruction_legs(legs, portfolios.len() as u32)]
        pub fn add_and_affirm_instruction(
            origin,
            venue_id: Option<VenueId>,
            settlement_type: SettlementType<T::BlockNumber>,
            trade_date: Option<T::Moment>,
            value_date: Option<T::Moment>,
            legs: Vec<Leg>,
            portfolios: BoundedBTreeSet<PortfolioId, T::MaxNumberOfPortfolios>,
            instruction_memo: Option<Memo>,
        ) {
            let did = Identity::<T>::ensure_perms(origin.clone())?;
            let instruction_id = Self::base_add_instruction(
                did,
                venue_id,
                settlement_type,
                trade_date,
                value_date,
                legs,
                instruction_memo,
                None
            )?;
            Self::affirm_and_maybe_schedule_instruction(
                origin,
                instruction_id,
                portfolios.into_inner(),
                None
            )
            .map_err(|e| e.error)?;
        }

        /// Provide affirmation to an existing instruction.
        ///
        /// # Arguments
        /// * `id` - the [`InstructionId`] of the instruction being affirmed.
        /// * `portfolios` - a vector of [`PortfolioId`] under the caller's control and intended for affirmation.
        ///
        /// # Permissions
        /// * Portfolio
        #[weight = <T as Config>::WeightInfo::affirm_instruction_input(None, portfolios.len() as u32)]
        pub fn affirm_instruction(origin, id: InstructionId, portfolios: BoundedBTreeSet<PortfolioId, T::MaxNumberOfPortfolios>) -> DispatchResultWithPostInfo {
            Self::affirm_and_maybe_schedule_instruction(
                origin,
                id,
                portfolios.into_inner(),
                None
            )
        }

        /// Withdraw an affirmation for a given instruction.
        ///
        /// # Arguments
        /// * `id` - the [`InstructionId`] of the instruction getting an affirmation withdrawn.
        /// * `portfolios` - a vector of [`PortfolioId`] under the caller's control and intended for affirmation withdrawal.
        ///
        /// # Permissions
        /// * Portfolio
        #[weight = <T as Config>::WeightInfo::withdraw_affirmation_input(None, portfolios.len() as u32)]
        pub fn withdraw_affirmation(origin, id: InstructionId, portfolios: BoundedBTreeSet<PortfolioId, T::MaxNumberOfPortfolios>) -> DispatchResultWithPostInfo {
            Self::base_withdraw_affirmation(origin, id, portfolios.into_inner(), None)
        }

        /// Rejects an existing instruction.
        ///
        /// # Arguments
        /// * `id` - the [`InstructionId`] of the instruction being rejected.
        /// * `portfolio` - the [`PortfolioId`] that belongs to the instruction and is rejecting it.
        ///
        /// # Permissions
        /// * Portfolio
        #[weight = <T as Config>::WeightInfo::reject_instruction_input(None, false)]
        pub fn reject_instruction(origin, id: InstructionId, portfolio: PortfolioId) -> DispatchResultWithPostInfo {
            Self::base_reject_instruction(origin, id, Some(portfolio), None)
        }

        /// Root callable extrinsic, used as an internal call to execute a scheduled settlement instruction.
        #[weight = (*weight_limit).max(<T as Config>::WeightInfo::execute_scheduled_instruction(0, 0, 0))]
        fn execute_scheduled_instruction(
            origin,
            id: InstructionId,
            weight_limit: Weight
        ) -> DispatchResultWithPostInfo {
            Self::ensure_root_origin(origin)?;
            let mut weight_meter = Self::ensure_valid_weight_meter(
                Self::execute_scheduled_instruction_minimum_weight(),
                weight_limit,
            )?;
            Ok(Self::base_execute_scheduled_instruction(id, &mut weight_meter))
        }

        /// Affirms an instruction using receipts for offchain transfers.
        ///
        /// # Arguments
        /// * `id` - the [`InstructionId`] of the instruction being affirmed.
        /// * `receipt_details` - a vector of [`ReceiptDetails`], which contain the details about the offchain transfer.
        /// * `portfolios` - a vector of [`PortfolioId`] under the caller's control and intended for affirmation.
        /// * `number_of_assets` - an optional [`AffirmationCount`] that will be used for a precise fee estimation before executing the extrinsic.
        ///
        /// Note: calling the rpc method `get_affirmation_count` returns an instance of [`AffirmationCount`].
        ///
        /// # Permissions
        /// * Portfolio
        #[weight = <T as Config>::WeightInfo::affirm_with_receipts_input(*number_of_assets, portfolios.len() as u32)]
        pub fn affirm_with_receipts_with_count(
            origin,
            id: InstructionId,
            receipt_details: Vec<ReceiptDetails<T::AccountId, T::OffChainSignature>>,
            portfolios: BoundedBTreeSet<PortfolioId, T::MaxNumberOfPortfolios>,
            number_of_assets: Option<AffirmationCount>
        ) {
            Self::affirm_with_receipts_and_maybe_schedule_instruction(
                origin,
                id,
                receipt_details,
                portfolios.into_inner(),
                number_of_assets
            )
            .map_err(|e| e.error)?;
        }

        /// Provide affirmation to an existing instruction.
        ///
        /// # Arguments
        /// * `id` - the [`InstructionId`] of the instruction being affirmed.
        /// * `portfolios` - a vector of [`PortfolioId`] under the caller's control and intended for affirmation.
        /// * `number_of_assets` - an optional [`AffirmationCount`] that will be used for a precise fee estimation before executing the extrinsic.
        ///
        /// Note: calling the rpc method `get_affirmation_count` returns an instance of [`AffirmationCount`].
        ///
        /// # Permissions
        /// * Portfolio
        #[weight = <T as Config>::WeightInfo::affirm_instruction_input(*number_of_assets, portfolios.len() as u32)]
        pub fn affirm_instruction_with_count(
            origin,
            id: InstructionId,
            portfolios: BoundedBTreeSet<PortfolioId, T::MaxNumberOfPortfolios>,
            number_of_assets: Option<AffirmationCount>
        ) {
            Self::affirm_and_maybe_schedule_instruction(
                origin,
                id,
                portfolios.into_inner(),
                number_of_assets
            )
            .map_err(|e| e.error)?;
        }

        /// Rejects an existing instruction.
        ///
        /// # Arguments
        /// * `id` - the [`InstructionId`] of the instruction being rejected.
        /// * `portfolio` - the [`PortfolioId`] that belongs to the instruction and is rejecting it.
        /// * `number_of_assets` - an optional [`AssetCount`] that will be used for a precise fee estimation before executing the extrinsic.
        ///
        /// Note: calling the rpc method `get_execute_instruction_info` returns an instance of [`ExecuteInstructionInfo`], which contain the asset count.
        ///
        /// # Permissions
        /// * Portfolio
        #[weight = <T as Config>::WeightInfo::reject_instruction_input(*number_of_assets, false)]
        pub fn reject_instruction_with_count(
            origin,
            id: InstructionId,
            portfolio: PortfolioId,
            number_of_assets: Option<AssetCount>
        ) {
            Self::base_reject_instruction(origin, id, Some(portfolio), number_of_assets)
                .map_err(|e| e.error)?;
        }

        /// Withdraw an affirmation for a given instruction.
        ///
        /// # Arguments
        /// * `id` - the [`InstructionId`] of the instruction getting an affirmation withdrawn.
        /// * `portfolios` - a vector of [`PortfolioId`] under the caller's control and intended for affirmation withdrawal.
        /// * `number_of_assets` - an optional [`AffirmationCount`] that will be used for a precise fee estimation before executing the extrinsic.
        ///
        /// Note: calling the rpc method `get_affirmation_count` returns an instance of [`AffirmationCount`].
        ///
        /// # Permissions
        /// * Portfolio
        #[weight = <T as Config>::WeightInfo::withdraw_affirmation_input(*number_of_assets, portfolios.len() as u32)]
        pub fn withdraw_affirmation_with_count(
            origin,
            id: InstructionId,
            portfolios: BoundedBTreeSet<PortfolioId, T::MaxNumberOfPortfolios>,
            number_of_assets: Option<AffirmationCount>
        ) {
            Self::base_withdraw_affirmation(origin, id, portfolios.into_inner(), number_of_assets)
                .map_err(|e| e.error)?;
        }

        /// Adds a new instruction with mediators.
        ///
        /// # Arguments
        /// * `venue_id`: The [`VenueId`] of the venue this instruction belongs to.
        /// * `settlement_type`: The [`SettlementType`] specifying when the instruction should be settled.
        /// * `trade_date`: Optional date from which people can interact with this instruction.
        /// * `value_date`: Optional date after which the instruction should be settled (not enforced).
        /// * `legs`: A vector of all [`Leg`] included in this instruction.
        /// * `instruction_memo`: An optional [`Memo`] field for this instruction.
        /// * `mediators`: A set of [`IdentityId`] of all the mandatory mediators for the instruction.
        #[weight = <T as Config>::WeightInfo::add_instruction_with_mediators_legs(legs, mediators.len() as u32)]
        pub fn add_instruction_with_mediators(
            origin,
            venue_id: Option<VenueId>,
            settlement_type: SettlementType<T::BlockNumber>,
            trade_date: Option<T::Moment>,
            value_date: Option<T::Moment>,
            legs: Vec<Leg>,
            instruction_memo: Option<Memo>,
            mediators: BoundedBTreeSet<IdentityId, T::MaxInstructionMediators>,
        ) {
            let did = Identity::<T>::ensure_perms(origin)?;
            Self::base_add_instruction(
                did,
                venue_id,
                settlement_type,
                trade_date,
                value_date,
                legs,
                instruction_memo,
                Some(mediators)
            )?;
        }

        /// Adds and affirms a new instruction with mediators.
        ///
        /// # Arguments
        /// * `venue_id`: The [`VenueId`] of the venue this instruction belongs to.
        /// * `settlement_type`: The [`SettlementType`] specifying when the instruction should be settled.
        /// * `trade_date`: Optional date from which people can interact with this instruction.
        /// * `value_date`: Optional date after which the instruction should be settled (not enforced).
        /// * `legs`: A vector of all [`Leg`] included in this instruction.
        /// * `portfolios`: A vector of [`PortfolioId`] under the caller's control and intended for affirmation.
        /// * `instruction_memo`: An optional [`Memo`] field for this instruction.
        /// * `mediators`: A set of [`IdentityId`] of all the mandatory mediators for the instruction.
        ///
        /// # Permissions
        /// * Portfolio
        #[weight = <T as Config>::WeightInfo::add_and_affirm_with_mediators_legs(legs, portfolios.len() as u32, mediators.len() as u32)]
        pub fn add_and_affirm_with_mediators(
            origin,
            venue_id: Option<VenueId>,
            settlement_type: SettlementType<T::BlockNumber>,
            trade_date: Option<T::Moment>,
            value_date: Option<T::Moment>,
            legs: Vec<Leg>,
            portfolios: BoundedBTreeSet<PortfolioId, T::MaxNumberOfPortfolios>,
            instruction_memo: Option<Memo>,
            mediators: BoundedBTreeSet<IdentityId, T::MaxInstructionMediators>,
        ) {
            let did = Identity::<T>::ensure_perms(origin.clone())?;
            let instruction_id = Self::base_add_instruction(
                did,
                venue_id,
                settlement_type,
                trade_date,
                value_date,
                legs,
                instruction_memo,
                Some(mediators)
            )?;
            Self::affirm_and_maybe_schedule_instruction(
                origin,
                instruction_id,
                portfolios.into_inner(),
                None
            )
            .map_err(|e| e.error)?;
        }

        /// Affirms the instruction as a mediator - should only be called by mediators, otherwise it will fail.
        ///
        /// # Arguments
        /// * `origin`: The secondary key of the sender.
        /// * `instruction_id`: The [`InstructionId`] that will be affirmed by the mediator.
        /// * `expiry`: An Optional value for defining when the affirmation will expire (None means it will always be valid).
        #[weight = <T as Config>::WeightInfo::affirm_instruction_as_mediator()]
        pub fn affirm_instruction_as_mediator(
            origin,
            instruction_id: InstructionId,
            expiry: Option<T::Moment>
        ) {
            Self::base_affirm_instruction_as_mediator(origin, instruction_id, expiry)?;
        }

        /// Removes the mediator's affirmation for the instruction - should only be called by mediators, otherwise it will fail.
        ///
        /// # Arguments
        /// * `origin`: The secondary key of the sender.
        /// * `instruction_id`: The [`InstructionId`] that will have the affirmation removed.
        #[weight = <T as Config>::WeightInfo::withdraw_affirmation_as_mediator()]
        pub fn withdraw_affirmation_as_mediator(origin, instruction_id: InstructionId) {
            Self::base_withdraw_affirmation_as_mediator(origin, instruction_id)?;
        }

        /// Rejects an existing instruction - should only be called by mediators, otherwise it will fail.
        ///
        /// # Arguments
        /// * `instruction_id` - the [`InstructionId`] of the instruction being rejected.
        /// * `number_of_assets` - an optional [`AssetCount`] that will be used for a precise fee estimation before executing the extrinsic.
        ///
        /// Note: calling the rpc method `get_execute_instruction_info` returns an instance of [`ExecuteInstructionInfo`], which contain the asset count.
        #[weight = <T as Config>::WeightInfo::reject_instruction_input(*number_of_assets, true)]
        pub fn reject_instruction_as_mediator(
            origin,
            instruction_id: InstructionId,
            number_of_assets: Option<AssetCount>
        ) -> DispatchResultWithPostInfo {
            Self::base_reject_instruction(origin, instruction_id, None, number_of_assets)
        }
    }
}

impl<T: Config> Module<T> {
    fn lock_via_leg(leg: &Leg) -> DispatchResult {
        match leg {
            Leg::Fungible {
                sender,
                asset_id,
                amount,
                ..
            } => T::Portfolio::lock_tokens(&sender, &asset_id, *amount),
            Leg::NonFungible { sender, nfts, .. } => with_transaction(|| {
                for nft_id in nfts.ids() {
                    T::Portfolio::lock_nft(&sender, nfts.asset_id(), &nft_id)?;
                }
                Ok(())
            }),
            Leg::OffChain { .. } => Err(Error::<T>::OffChainAssetCantBeLocked.into()),
        }
    }

    fn unlock_via_leg(leg: &Leg) -> DispatchResult {
        match leg {
            Leg::Fungible {
                sender,
                asset_id,
                amount,
                ..
            } => T::Portfolio::unlock_tokens(&sender, &asset_id, *amount),
            Leg::NonFungible { sender, nfts, .. } => with_transaction(|| {
                for nft_id in nfts.ids() {
                    T::Portfolio::unlock_nft(&sender, nfts.asset_id(), &nft_id)?;
                }
                Ok(())
            }),
            Leg::OffChain { .. } => Err(Error::<T>::OffChainAssetCantBeLocked.into()),
        }
    }

    /// Ensure origin call permission and the given instruction validity.
    fn ensure_origin_perm_and_instruction_validity(
        origin: <T as frame_system::Config>::RuntimeOrigin,
        id: InstructionId,
        is_execute: bool,
    ) -> EnsureValidInstructionResult<T::AccountId, T::Moment, T::BlockNumber> {
        let origin_data = Identity::<T>::ensure_origin_call_permissions(origin)?;
        Ok((
            origin_data.primary_did,
            origin_data.secondary_key,
            Self::ensure_instruction_validity(id, is_execute)?,
        ))
    }

    // Extract `Venue` with `id`, assuming it was created by `did`, or error.
    fn venue_for_management(id: VenueId, did: IdentityId) -> Result<Venue, DispatchError> {
        // Ensure venue exists & that DID created it.
        let venue = Self::venue_info(id).ok_or(Error::<T>::InvalidVenue)?;
        ensure!(venue.creator == did, Error::<T>::Unauthorized);
        Ok(venue)
    }

    pub fn base_add_instruction(
        did: IdentityId,
        venue_id: Option<VenueId>,
        settlement_type: SettlementType<T::BlockNumber>,
        trade_date: Option<T::Moment>,
        value_date: Option<T::Moment>,
        legs: Vec<Leg>,
        memo: Option<Memo>,
        mediators: Option<BoundedBTreeSet<IdentityId, T::MaxInstructionMediators>>,
    ) -> Result<InstructionId, DispatchError> {
        // Verifies if the block number is in the future so that `T::Scheduler::schedule_named` doesn't fail.
        if let SettlementType::SettleOnBlock(block_number) = &settlement_type {
            ensure!(
                *block_number > System::<T>::block_number(),
                Error::<T>::SettleOnPastBlock
            );
        }

        // Ensure that instruction dates are valid.
        if let (Some(trade_date), Some(value_date)) = (trade_date, value_date) {
            ensure!(
                value_date >= trade_date,
                Error::<T>::InstructionDatesInvalid
            );
        }

        // Ensure venue exists & sender is its creator.
        if let Some(venue_id) = venue_id {
            Self::venue_for_management(venue_id, did)?;
        }

        // Verifies if all legs are valid.
        let mut instruction_info = Self::ensure_valid_legs(&legs, &venue_id)?;

        // Adds the instruction mediators
        if let Some(mediators) = mediators {
            instruction_info.extend_mediators(mediators.into())
        }

        // Advance and get next `instruction_id`.
        let instruction_id = InstructionCounter::try_mutate(try_next_post::<T, _>)?;

        // All checks have been made - Write data to storage.
        InstructionStatuses::<T>::insert(instruction_id, InstructionStatus::Pending);

        for portfolio_id in instruction_info.portfolios_pending_approval() {
            UserAffirmations::insert(portfolio_id, instruction_id, AffirmationStatus::Pending);
        }

        for mediator_id in instruction_info.mediators() {
            InstructionMediatorsAffirmations::<T>::insert(
                instruction_id,
                mediator_id,
                MediatorAffirmationStatus::Pending,
            );
        }
        InstructionAffirmsPending::insert(
            instruction_id,
            instruction_info.number_of_pending_affirmations(),
        );

        legs.iter().enumerate().for_each(|(index, leg)| {
            let leg_id = LegId(index as u64);
            InstructionLegs::insert(instruction_id, leg_id, leg.clone());
            if leg.is_off_chain() {
                OffChainAffirmations::insert(instruction_id, leg_id, AffirmationStatus::Pending);
            }
        });

        <InstructionDetails<T>>::insert(
            instruction_id,
            Instruction {
                instruction_id,
                venue_id,
                settlement_type,
                created_at: Some(<pallet_timestamp::Pallet<T>>::get()),
                trade_date,
                value_date,
            },
        );
        if let Some(ref memo) = memo {
            InstructionMemos::insert(instruction_id, &memo);
        }
        if let Some(venue_id) = venue_id {
            VenueInstructions::insert(venue_id, instruction_id, ());
        }

        Self::deposit_event(RawEvent::InstructionCreated(
            did,
            venue_id,
            instruction_id,
            settlement_type,
            trade_date,
            value_date,
            legs,
            memo,
        ));

        for portfolio_id in instruction_info.portfolios_pre_approved_difference() {
            UserAffirmations::insert(portfolio_id, instruction_id, AffirmationStatus::Affirmed);
            AffirmsReceived::insert(instruction_id, portfolio_id, AffirmationStatus::Affirmed);
            Self::deposit_event(RawEvent::InstructionAutomaticallyAffirmed(
                did,
                *portfolio_id,
                instruction_id,
            ));
        }

        if !instruction_info.mediators().is_empty() {
            Self::deposit_event(RawEvent::InstructionMediators(
                instruction_id,
                instruction_info.mediators().clone(),
            ));
        }

        if let SettlementType::SettleOnBlock(block_number) = settlement_type {
            let weight_limit = Self::execute_scheduled_instruction_weight_limit(
                instruction_info.fungible_transfers(),
                instruction_info.nfts_transferred(),
                instruction_info.off_chain(),
            );
            Self::schedule_instruction(instruction_id, block_number, weight_limit);
        }

        Ok(instruction_id)
    }

    /// Returns [`InstructionInfo`] if all legs are valid, otherwise returns an error.
    /// See also: [`Module::ensure_valid_fungible_leg`], [`Module::ensure_valid_nft_leg`] and [`Module::ensure_valid_off_chain_leg`].
    fn ensure_valid_legs(
        legs: &[Leg],
        venue_id: &Option<VenueId>,
    ) -> Result<InstructionInfo, DispatchError> {
        // Tracks the number of fungible, non-fungible and offchain assets across the legs
        let mut instruction_asset_count = AssetCount::default();
        // Tracks all portfolios that have not been pre-affirmed
        let mut portfolios_pending_approval = BTreeSet::new();
        // Tracks all portfolios that have pre-approved the transfer.
        let mut portfolios_pre_approved = BTreeSet::new();
        // Tracks all mediators that have to affirm the instruction.
        let mut mediators = BTreeSet::new();
        // Tracks all tickers that have been checked for filtering
        let mut tickers = BTreeSet::new();

        // Validates all legs and checks if they have been pre-affirmed
        for leg in legs {
            Self::ensure_valid_leg(leg, venue_id, &mut tickers, &mut instruction_asset_count)?;

            let (asset_id, sender, receiver) = {
                match leg {
                    Leg::Fungible {
                        sender,
                        receiver,
                        asset_id,
                        ..
                    } => (asset_id, sender, receiver),
                    Leg::NonFungible {
                        sender,
                        receiver,
                        nfts,
                    } => (nfts.asset_id(), sender, receiver),
                    Leg::OffChain { .. } => continue,
                }
            };
            Identity::<T>::ensure_id_record_exists(sender.did)?;
            Identity::<T>::ensure_id_record_exists(receiver.did)?;
            T::Portfolio::ensure_portfolio_validity(sender)?;
            T::Portfolio::ensure_portfolio_validity(receiver)?;

            portfolios_pending_approval.insert(*sender);
            if T::Portfolio::skip_portfolio_affirmation(receiver, asset_id) {
                portfolios_pre_approved.insert(*receiver);
            } else {
                portfolios_pending_approval.insert(*receiver);
            }

            let asset_mediators = MandatoryMediators::<T>::get(asset_id);
            mediators.extend(asset_mediators.iter());
        }
        // The maximum number of each asset type in one instruction is checked here
        Self::ensure_within_instruction_max(&instruction_asset_count)?;

        Ok(InstructionInfo::new(
            instruction_asset_count,
            portfolios_pending_approval,
            portfolios_pre_approved,
            mediators,
        ))
    }

    fn unsafe_withdraw_instruction_affirmation(
        did: IdentityId,
        id: InstructionId,
        portfolios: BTreeSet<PortfolioId>,
        secondary_key: Option<&SecondaryKey<T::AccountId>>,
        affirmation_count: Option<AffirmationCount>,
    ) -> Result<FilteredLegs, DispatchError> {
        // checks custodianship of portfolios and affirmation status
        Self::ensure_portfolios_and_affirmation_status(
            id,
            &portfolios,
            did,
            secondary_key,
            &[AffirmationStatus::Affirmed],
        )?;
        // Unlock tokens that were previously locked during the affirmation
        let filtered_legs = Self::filtered_legs(id, &portfolios);
        // If the fee was estimated in advance, the input values must be at least equal to the actual values
        if let Some(affirmation_count) = affirmation_count {
            Self::ensure_valid_affirmation_count(&filtered_legs, &affirmation_count)?;
        }
        for (leg_id, leg) in filtered_legs.sender_subset() {
            match Self::instruction_leg_status(id, leg_id) {
                LegStatus::ExecutionToBeSkipped(_, _) => {
                    return Err(Error::<T>::UnexpectedLegStatus.into())
                }
                LegStatus::ExecutionPending => {
                    Self::unlock_via_leg(&leg)?;
                }
                LegStatus::PendingTokenLock => {
                    return Err(Error::<T>::InstructionNotAffirmed.into());
                }
            };
            <InstructionLegStatus<T>>::insert(id, leg_id, LegStatus::PendingTokenLock);
        }

        // Updates storage.
        for portfolio in &portfolios {
            UserAffirmations::insert(portfolio, id, AffirmationStatus::Pending);
            AffirmsReceived::remove(id, portfolio);
            Self::deposit_event(RawEvent::AffirmationWithdrawn(did, *portfolio, id));
        }

        InstructionAffirmsPending::mutate(id, |affirms_pending| {
            *affirms_pending += u64::try_from(portfolios.len()).unwrap_or_default()
        });
        Ok(filtered_legs)
    }

    fn ensure_instruction_validity(
        id: InstructionId,
        is_execute: bool,
    ) -> Result<Instruction<T::Moment, T::BlockNumber>, DispatchError> {
        let details = Self::instruction_details(id);
        ensure!(
            Self::instruction_status(id) != InstructionStatus::Unknown,
            Error::<T>::UnknownInstruction
        );

        match (details.settlement_type, is_execute) {
            // is_execute is true for execution
            (SettlementType::SettleOnBlock(block_number), true) => {
                // Ensures block number is less than or equal to current block number.
                ensure!(
                    block_number <= System::<T>::block_number(),
                    Error::<T>::InstructionSettleBlockNotReached
                );
            }
            // is_execute is false for affirmation
            (SettlementType::SettleOnBlock(block_number), false) => {
                // Ensures block number is greater than current block number.
                ensure!(
                    block_number > System::<T>::block_number(),
                    Error::<T>::InstructionSettleBlockPassed
                );
            }
            (SettlementType::SettleManual(block_number), true) => {
                // Ensures block number is less than  or equal to current block number.
                ensure!(
                    block_number <= System::<T>::block_number(),
                    Error::<T>::InstructionSettleBlockNotReached
                );
            }
            (_, _) => {}
        }

        Ok(details)
    }

    /// Executes the instruction of the given `id`. If the execution succeeds, the instruction gets pruned,
    /// otherwise the instruction status is set to failed.
    fn execute_instruction_retryable(
        id: InstructionId,
        caller_did: IdentityId,
        weight_meter: &mut WeightMeter,
    ) -> DispatchResult {
        if let Err(e) = Self::execute_instruction(id, caller_did, weight_meter) {
            InstructionStatuses::<T>::insert(id, InstructionStatus::Failed);
            return Err(e);
        }
        Ok(())
    }

    fn execute_instruction(
        instruction_id: InstructionId,
        caller_did: IdentityId,
        weight_meter: &mut WeightMeter,
    ) -> DispatchResult {
        let mut failed_leg_id = None;
        let tx_result = with_transaction(|| {
            // Ensures the number of pending affirmations is zero
            let n_pending_affirmations = InstructionAffirmsPending::take(instruction_id);
            ensure!(
                n_pending_affirmations == 0,
                Error::<T>::NotAllAffirmationsHaveBeenReceived
            );
            // Ensures the instruction is pending or has failed at least one time
            let instruction_status = InstructionStatuses::<T>::get(instruction_id);
            ensure!(
                instruction_status == InstructionStatus::Pending
                    || instruction_status == InstructionStatus::Failed,
                Error::<T>::InvalidInstructionStatusForExecution
            );
            // Ensures all mediator's affirmations are still valid
            Self::ensure_non_expired_affirmations(&instruction_id)?;

            // The order of execution of the legs matter in some edge cases around compliance.
            let mut instruction_legs: Vec<(LegId, Leg)> =
                InstructionLegs::drain_prefix(&instruction_id).collect();
            instruction_legs.sort_by_key(|leg_id_leg| leg_id_leg.0);

            // Ensures all affirmations have been received
            Self::ensure_no_missing_affirmation(&instruction_id, &instruction_legs)?;

            let instruction_asset_count = AssetCount::from_legs(&instruction_legs);
            weight_meter
                .check_accrue(<T as Config>::WeightInfo::execute_instruction_paused(
                    instruction_asset_count.fungible(),
                    instruction_asset_count.non_fungible(),
                    instruction_asset_count.off_chain(),
                ))
                .map_err(|_| Error::<T>::WeightLimitExceeded)?;

            // Ensures the venue is allowed for all tickers in the instruction
            let instruction_details = InstructionDetails::<T>::take(instruction_id);
            Self::ensure_allowed_venue(&instruction_legs, instruction_details.venue_id)?;

            // Attempts to release the locks
            Self::release_locks(instruction_id, &instruction_legs)?;

            // Transfer all fungible an non fungible assets
            let instruction_memo = InstructionMemos::get(&instruction_id);
            match Self::transfer_pending_legs(
                instruction_id,
                &instruction_legs,
                instruction_memo,
                caller_did,
                weight_meter,
            ) {
                Ok(_) => {
                    // Remove remaning storage
                    if let Some(venue_id) = instruction_details.venue_id {
                        VenueInstructions::remove(venue_id, instruction_id);
                    }
                    let _ = InstructionLegStatus::<T>::clear_prefix(
                        instruction_id,
                        instruction_legs.len() as u32,
                        None,
                    );
                    // Change instruction status
                    InstructionStatuses::<T>::insert(
                        instruction_id,
                        InstructionStatus::Success(System::<T>::block_number()),
                    );
                    Self::deposit_event(RawEvent::InstructionExecuted(caller_did, instruction_id));
                    Ok(())
                }
                Err(leg_id) => {
                    failed_leg_id = Some(leg_id);
                    Err(Error::<T>::FailedToReleaseLockOrTransferAssets.into())
                }
            }
        });

        // Since with_transaction reverts events as well, the events have to be emitted here
        if let Some(failed_leg_id) = failed_leg_id {
            Self::deposit_event(RawEvent::LegFailedExecution(
                caller_did,
                instruction_id,
                failed_leg_id,
            ));
        }

        tx_result
    }

    /// Returns `Ok` if all mediator's affirmation are still valid. Otherwise, returns an error.
    /// This call also removes all elements from the `InstructionMediatorsAffirmations` storage.
    fn ensure_non_expired_affirmations(instruction_id: &InstructionId) -> DispatchResult {
        let current_timestamp = <pallet_timestamp::Pallet<T>>::get();
        for (_, mediator_affirmation) in
            InstructionMediatorsAffirmations::<T>::drain_prefix(instruction_id)
        {
            match mediator_affirmation {
                MediatorAffirmationStatus::Affirmed { expiry, .. } => {
                    if let Some(expiry) = expiry {
                        ensure!(
                            expiry > current_timestamp,
                            Error::<T>::MediatorAffirmationExpired
                        );
                    }
                }
                MediatorAffirmationStatus::Unknown | MediatorAffirmationStatus::Pending => {
                    return Err(Error::<T>::NotAllAffirmationsHaveBeenReceived.into())
                }
            }
        }
        Ok(())
    }

    /// Returns `Ok` if all affirmations have been received. Otherwise, returns an error.
    /// This call also removes all elements from `UserAffirmations`, `AffirmsReceived` and `OffChainAffirmations` storage.
    fn ensure_no_missing_affirmation(
        instruction_id: &InstructionId,
        instruction_legs: &[(LegId, Leg)],
    ) -> DispatchResult {
        let mut unique_portfolios = BTreeSet::new();

        for (leg_id, leg) in instruction_legs {
            match leg {
                Leg::Fungible {
                    sender, receiver, ..
                }
                | Leg::NonFungible {
                    sender, receiver, ..
                } => {
                    if unique_portfolios.insert(sender) {
                        let sdr_affirmation_status = UserAffirmations::take(sender, instruction_id);
                        ensure!(
                            sdr_affirmation_status == AffirmationStatus::Affirmed,
                            Error::<T>::NotAllAffirmationsHaveBeenReceived
                        );
                        let sdr_affirmation_status = AffirmsReceived::take(instruction_id, sender);
                        ensure!(
                            sdr_affirmation_status == AffirmationStatus::Affirmed,
                            Error::<T>::NotAllAffirmationsHaveBeenReceived
                        );
                    }
                    if unique_portfolios.insert(receiver) {
                        let rcv_affirmation_status =
                            UserAffirmations::take(receiver, instruction_id);
                        ensure!(
                            rcv_affirmation_status == AffirmationStatus::Affirmed,
                            Error::<T>::NotAllAffirmationsHaveBeenReceived
                        );
                        let rcv_affirmation_status =
                            AffirmsReceived::take(instruction_id, receiver);
                        ensure!(
                            rcv_affirmation_status == AffirmationStatus::Affirmed,
                            Error::<T>::NotAllAffirmationsHaveBeenReceived
                        );
                    }
                }
                Leg::OffChain { .. } => {
                    ensure!(
                        OffChainAffirmations::take(instruction_id, leg_id)
                            == AffirmationStatus::Affirmed,
                        Error::<T>::NotAllAffirmationsHaveBeenReceived,
                    );
                }
            }
        }

        Ok(())
    }

    fn transfer_pending_legs(
        instruction_id: InstructionId,
        instruction_legs: &[(LegId, Leg)],
        instruction_memo: Option<Memo>,
        caller_did: IdentityId,
        weight_meter: &mut WeightMeter,
    ) -> Result<(), LegId> {
        for (leg_id, leg) in instruction_legs {
            if Self::instruction_leg_status(instruction_id, leg_id) == LegStatus::ExecutionPending {
                match leg {
                    Leg::Fungible {
                        sender,
                        receiver,
                        asset_id,
                        amount,
                    } => {
                        if <Asset<T>>::base_transfer(
                            *sender,
                            *receiver,
                            *asset_id,
                            *amount,
                            Some(instruction_id),
                            instruction_memo.clone(),
                            caller_did,
                            weight_meter,
                        )
                        .is_err()
                        {
                            return Err(*leg_id);
                        }
                    }
                    Leg::NonFungible {
                        sender,
                        receiver,
                        nfts,
                    } => {
                        if <Nft<T>>::base_nft_transfer(
                            *sender,
                            *receiver,
                            nfts.clone(),
                            instruction_id,
                            instruction_memo.clone(),
                            caller_did,
                            weight_meter,
                        )
                        .is_err()
                        {
                            return Err(*leg_id);
                        }
                    }
                    Leg::OffChain { .. } => {}
                }
            }
        }
        Ok(())
    }

    /// Clears the storage for a rejected instruction and updates the instruction status to
    /// [`InstructionStatus::Rejected`].
    fn prune_rejected_instruction(instruction_id: InstructionId) {
        let instruction_details = InstructionDetails::<T>::take(&instruction_id);
        if let Some(venue_id) = instruction_details.venue_id {
            VenueInstructions::remove(venue_id, instruction_id);
        }
        InstructionAffirmsPending::remove(instruction_id);
        let _ = InstructionMediatorsAffirmations::<T>::clear_prefix(
            instruction_id,
            T::MaxInstructionMediators::get(),
            None,
        );
        // We need all portfolios to clear the UserAffirmations storage
        let instruction_legs =
            InstructionLegs::drain_prefix(&instruction_id).collect::<Vec<(LegId, Leg)>>();
        let _ = InstructionLegStatus::<T>::clear_prefix(
            &instruction_id,
            instruction_legs.len() as u32,
            None,
        );
        for (leg_id, leg) in instruction_legs {
            match leg {
                Leg::Fungible {
                    sender, receiver, ..
                }
                | Leg::NonFungible {
                    sender, receiver, ..
                } => {
                    UserAffirmations::remove(sender, instruction_id);
                    UserAffirmations::remove(receiver, instruction_id);
                    AffirmsReceived::remove(instruction_id, sender);
                    AffirmsReceived::remove(instruction_id, receiver);
                }
                Leg::OffChain { .. } => {
                    OffChainAffirmations::remove(instruction_id, leg_id);
                }
            }
        }
        // Update the intruction Status to InstructionStatus::Rejected
        InstructionStatuses::<T>::insert(
            instruction_id,
            InstructionStatus::Rejected(System::<T>::block_number()),
        );
    }

    pub fn unsafe_affirm_instruction(
        did: IdentityId,
        id: InstructionId,
        portfolios: BTreeSet<PortfolioId>,
        secondary_key: Option<&SecondaryKey<T::AccountId>>,
        affirmation_count: Option<AffirmationCount>,
    ) -> Result<FilteredLegs, DispatchError> {
        // Checks portfolio's custodian and if it is a counter party with a pending affirmation.
        Self::ensure_portfolios_and_affirmation_status(
            id,
            &portfolios,
            did,
            secondary_key,
            &[AffirmationStatus::Pending],
        )?;

        let filtered_legs = Self::filtered_legs(id, &portfolios);
        // If the fee was estimated in advance, the input values must be at least equal to the actual values
        if let Some(affirmation_count) = affirmation_count {
            Self::ensure_valid_affirmation_count(&filtered_legs, &affirmation_count)?
        }
        for (leg_id, leg) in filtered_legs.sender_subset() {
            Self::lock_via_leg(&leg)?;
            <InstructionLegStatus<T>>::insert(id, leg_id, LegStatus::ExecutionPending);
        }

        let affirms_pending = Self::instruction_affirms_pending(id);

        // Updates storage
        for portfolio in &portfolios {
            UserAffirmations::insert(portfolio, id, AffirmationStatus::Affirmed);
            AffirmsReceived::insert(id, portfolio, AffirmationStatus::Affirmed);
            Self::deposit_event(RawEvent::InstructionAffirmed(did, *portfolio, id));
        }
        InstructionAffirmsPending::insert(
            id,
            affirms_pending.saturating_sub(u64::try_from(portfolios.len()).unwrap_or_default()),
        );
        Ok(filtered_legs)
    }

    fn release_locks(id: InstructionId, instruction_legs: &[(LegId, Leg)]) -> DispatchResult {
        for (leg_id, leg) in instruction_legs {
            if let LegStatus::ExecutionPending = Self::instruction_leg_status(id, leg_id) {
                Self::unlock_via_leg(&leg)?;
            }
        }
        Ok(())
    }

    /// Schedule a given instruction to be executed on the next block only if the
    /// settlement type is `SettleOnAffirmation` and no. of affirms pending is 0.
    fn maybe_schedule_instruction(affirms_pending: u64, id: InstructionId, weight_limit: Weight) {
        if affirms_pending == 0
            && Self::instruction_details(id).settlement_type == SettlementType::SettleOnAffirmation
        {
            // Schedule instruction to be executed in the next block.
            let execution_at = System::<T>::block_number() + One::one();
            Self::schedule_instruction(id, execution_at, weight_limit);
        }
    }

    /// Schedule execution of given instruction at given block number.
    ///
    /// NB - It is expected to execute the given instruction into the given block number but
    /// it is not a guaranteed behavior, Scheduler may have other high priority task scheduled
    /// for the given block so there are chances where the instruction execution block no. may drift.
    pub(crate) fn schedule_instruction(
        id: InstructionId,
        execution_at: T::BlockNumber,
        weight_limit: Weight,
    ) {
        let call = Call::<T>::execute_scheduled_instruction { id, weight_limit }.into();
        if let Err(_) = T::Scheduler::schedule_named(
            id.execution_name(),
            DispatchTime::At(execution_at),
            None,
            SETTLEMENT_INSTRUCTION_EXECUTION_PRIORITY,
            RawOrigin::Root.into(),
            call,
        ) {
            Self::deposit_event(RawEvent::SchedulingFailed(
                id,
                Error::<T>::FailedToSchedule.into(),
            ));
        }
    }

    /// Affirms all legs from the instruction of the given `instruction_id`, where `portfolios` are a counter party.
    /// If the portfolio is the sender, the asset is also locked.
    pub fn base_affirm_with_receipts(
        origin: <T as frame_system::Config>::RuntimeOrigin,
        instruction_id: InstructionId,
        receipts_details: Vec<ReceiptDetails<T::AccountId, T::OffChainSignature>>,
        portfolios: BTreeSet<PortfolioId>,
        affirmation_count: Option<AffirmationCount>,
    ) -> Result<FilteredLegs, DispatchError> {
        ensure!(
            receipts_details.len() <= T::MaxNumberOfOffChainAssets::get() as usize,
            Error::<T>::MaxNumberOfReceiptsExceeded
        );

        let (did, secondary_key, instruction_details) =
            Self::ensure_origin_perm_and_instruction_validity(origin, instruction_id, false)?;

        // Verify portfolio custodianship and check if it is a counter party with a pending affirmation.
        Self::ensure_portfolios_and_affirmation_status(
            instruction_id,
            &portfolios,
            did,
            secondary_key.as_ref(),
            &[AffirmationStatus::Pending],
        )?;

        Self::ensure_valid_receipts_details(
            instruction_details.venue_id,
            instruction_id,
            &receipts_details,
        )?;

        // Lock tokens for all legs that are not of type [`Leg::OffChain`]
        let filtered_legs = Self::filtered_legs(instruction_id, &portfolios);
        // If the fee was estimated in advance, the input values must be at least equal to the actual values
        if let Some(affirmation_count) = affirmation_count {
            Self::ensure_valid_affirmation_count(&filtered_legs, &affirmation_count)?
        }
        for (leg_id, leg) in filtered_legs.sender_subset() {
            Self::lock_via_leg(&leg)?;
            <InstructionLegStatus<T>>::insert(instruction_id, leg_id, LegStatus::ExecutionPending);
        }

        // Casting is safe since `Self::ensure_portfolios_and_affirmation_status` is called
        let affirms_pending = InstructionAffirmsPending::get(instruction_id)
            .saturating_sub(portfolios.len() as u64)
            .saturating_sub(receipts_details.len() as u64);
        InstructionAffirmsPending::insert(instruction_id, affirms_pending);

        // Update storage
        for receipt_detail in receipts_details {
            <InstructionLegStatus<T>>::insert(
                instruction_id,
                receipt_detail.leg_id(),
                LegStatus::ExecutionToBeSkipped(
                    receipt_detail.signer().clone(),
                    receipt_detail.uid(),
                ),
            );
            OffChainAffirmations::insert(
                instruction_id,
                receipt_detail.leg_id(),
                AffirmationStatus::Affirmed,
            );
            <ReceiptsUsed<T>>::insert(receipt_detail.signer(), receipt_detail.uid(), true);
            Self::deposit_event(RawEvent::ReceiptClaimed(
                did,
                instruction_id,
                receipt_detail.leg_id(),
                receipt_detail.uid(),
                receipt_detail.signer().clone(),
                receipt_detail.metadata().clone(),
            ));
        }

        for portfolio in portfolios {
            UserAffirmations::insert(portfolio, instruction_id, AffirmationStatus::Affirmed);
            AffirmsReceived::insert(instruction_id, portfolio, AffirmationStatus::Affirmed);
            Self::deposit_event(RawEvent::InstructionAffirmed(
                did,
                portfolio,
                instruction_id,
            ));
        }

        Ok(filtered_legs)
    }

    pub fn base_affirm_instruction(
        origin: <T as frame_system::Config>::RuntimeOrigin,
        id: InstructionId,
        portfolios: BTreeSet<PortfolioId>,
        affirmation_count: Option<AffirmationCount>,
    ) -> Result<FilteredLegs, DispatchError> {
        let (did, sk, _) = Self::ensure_origin_perm_and_instruction_validity(origin, id, false)?;
        // Provide affirmation to the instruction
        Self::unsafe_affirm_instruction(did, id, portfolios, sk.as_ref(), affirmation_count)
    }

    /// Affirms all legs from the instruction of the given `id`, where `portfolios` are a counter party.
    /// If the portfolio is the sender, the asset is also locked. If all affirmation have been received and
    /// the settlement type is [`SettlementType::SettleOnAffirmation`] the instruction will be scheduled for
    /// the next block.
    pub fn affirm_with_receipts_and_maybe_schedule_instruction(
        origin: <T as frame_system::Config>::RuntimeOrigin,
        id: InstructionId,
        receipt_details: Vec<ReceiptDetails<T::AccountId, T::OffChainSignature>>,
        portfolios: BTreeSet<PortfolioId>,
        affirmation_count: Option<AffirmationCount>,
    ) -> DispatchResultWithPostInfo {
        let filtered_legs = Self::base_affirm_with_receipts(
            origin,
            id,
            receipt_details,
            portfolios,
            affirmation_count,
        )?;
        let instruction_asset_count = filtered_legs.unfiltered_asset_count();
        let weight_limit = Self::execute_scheduled_instruction_weight_limit(
            instruction_asset_count.fungible(),
            instruction_asset_count.non_fungible(),
            instruction_asset_count.off_chain(),
        );
        // Schedule instruction to be executed in the next block (expected) if conditions are met.
        Self::maybe_schedule_instruction(Self::instruction_affirms_pending(id), id, weight_limit);
        Ok(PostDispatchInfo::from(Some(
            Self::affirm_with_receipts_actual_weight(
                filtered_legs.sender_asset_count().clone(),
                filtered_legs.receiver_asset_count().clone(),
                filtered_legs.unfiltered_asset_count().off_chain(),
            ),
        )))
    }

    /// Affirms all legs from the instruction of the given `id`, where `portfolios` are a counter party.
    /// If the portfolio is the sender, the asset is also locked. If all affirmation have been received and
    /// the settlement type is [`SettlementType::SettleOnAffirmation`] the instruction will be scheduled for
    /// the next block.
    pub fn affirm_and_maybe_schedule_instruction(
        origin: <T as frame_system::Config>::RuntimeOrigin,
        id: InstructionId,
        portfolios: BTreeSet<PortfolioId>,
        affirmation_count: Option<AffirmationCount>,
    ) -> DispatchResultWithPostInfo {
        let filtered_legs =
            Self::base_affirm_instruction(origin, id, portfolios, affirmation_count)?;
        let instruction_asset_count = filtered_legs.unfiltered_asset_count();
        let weight_limit = Self::execute_scheduled_instruction_weight_limit(
            instruction_asset_count.fungible(),
            instruction_asset_count.non_fungible(),
            instruction_asset_count.off_chain(),
        );
        // Schedule the instruction if conditions are met
        Self::maybe_schedule_instruction(Self::instruction_affirms_pending(id), id, weight_limit);
        Ok(PostDispatchInfo::from(Some(
            Self::affirm_instruction_actual_weight(
                filtered_legs.sender_asset_count().clone(),
                filtered_legs.receiver_asset_count().clone(),
            ),
        )))
    }

    /// Affirm with or without receipts, executing the instruction when all affirmations have been received.
    ///
    /// NB - Use this function only in the STO pallet to support DVP settlements.
    pub fn affirm_and_execute_instruction(
        origin: <T as frame_system::Config>::RuntimeOrigin,
        id: InstructionId,
        receipt: Option<ReceiptDetails<T::AccountId, T::OffChainSignature>>,
        portfolios: BTreeSet<PortfolioId>,
        caller_did: IdentityId,
        weight_meter: &mut WeightMeter,
    ) -> DispatchResult {
        match receipt {
            Some(receipt) => {
                Self::base_affirm_with_receipts(origin, id, vec![receipt], portfolios, None)?
            }
            None => Self::base_affirm_instruction(origin, id, portfolios, None)?,
        };
        Self::execute_settle_on_affirmation_instruction(
            id,
            Self::instruction_affirms_pending(id),
            Self::instruction_details(id).settlement_type,
            caller_did,
            weight_meter,
        )?;
        Ok(())
    }

    fn execute_settle_on_affirmation_instruction(
        id: InstructionId,
        affirms_pending: u64,
        settlement_type: SettlementType<T::BlockNumber>,
        caller_did: IdentityId,
        weight_meter: &mut WeightMeter,
    ) -> DispatchResult {
        // We assume `settlement_type == SettleOnAffirmation`,
        // to be defensive, however, this is checked before instruction execution.
        if settlement_type == SettlementType::SettleOnAffirmation && affirms_pending == 0 {
            // We use execute_instruction here directly
            // and not the execute_instruction_retryable variant
            // because direct settlement is not retryable.
            Self::execute_instruction(id, caller_did, weight_meter)?;
        }
        Ok(())
    }

    fn ensure_portfolios_and_affirmation_status(
        id: InstructionId,
        portfolios: &BTreeSet<PortfolioId>,
        custodian: IdentityId,
        secondary_key: Option<&SecondaryKey<T::AccountId>>,
        expected_statuses: &[AffirmationStatus],
    ) -> DispatchResult {
        for portfolio in portfolios {
            T::Portfolio::ensure_portfolio_custody_and_permission(
                *portfolio,
                custodian,
                secondary_key,
            )?;
            let user_affirmation = Self::user_affirmations(portfolio, id);
            ensure!(
                expected_statuses.contains(&user_affirmation),
                Error::<T>::UnexpectedAffirmationStatus
            );
        }
        Ok(())
    }

    /// Returns [`FilteredLegs`] where the orginal set is all legs in the instruction of the given
    /// `id` and the subset of legs are all legs where the sender is in the given `portfolio`.
    fn filtered_legs(id: InstructionId, portfolio: &BTreeSet<PortfolioId>) -> FilteredLegs {
        let instruction_legs: Vec<(LegId, Leg)> = InstructionLegs::iter_prefix(&id).collect();
        FilteredLegs::filter_sender(instruction_legs, portfolio)
    }

    fn get_instruction_asset_count(id: &InstructionId) -> AssetCount {
        // Get the weight limit for the instruction
        let legs: Vec<(LegId, Leg)> = InstructionLegs::iter_prefix(id).collect();
        AssetCount::from_legs(&legs)
    }

    fn base_update_venue_signers(
        did: IdentityId,
        id: VenueId,
        signers: Vec<T::AccountId>,
        add_signers: bool,
    ) -> DispatchResult {
        // Ensure venue exists & sender is its creator.
        Self::venue_for_management(id, did)?;

        if add_signers {
            let current_number_of_signers = NumberOfVenueSigners::get(id);
            ensure!(
                (current_number_of_signers as usize).saturating_add(signers.len())
                    <= T::MaxNumberOfVenueSigners::get() as usize,
                Error::<T>::NumberOfVenueSignersExceeded
            );
            for signer in &signers {
                ensure!(
                    !Self::venue_signers(&id, &signer),
                    Error::<T>::SignerAlreadyExists
                );
            }
            NumberOfVenueSigners::insert(id, current_number_of_signers + signers.len() as u32);
            for signer in &signers {
                <VenueSigners<T>>::insert(&id, &signer, true);
            }
        } else {
            for signer in &signers {
                ensure!(
                    Self::venue_signers(&id, &signer),
                    Error::<T>::SignerDoesNotExist
                );
            }
            let current_number_of_signers = NumberOfVenueSigners::get(id);
            NumberOfVenueSigners::insert(
                id,
                current_number_of_signers.saturating_sub(signers.len() as u32),
            );
            for signer in &signers {
                <VenueSigners<T>>::remove(&id, &signer);
            }
        }

        Self::deposit_event(RawEvent::VenueSignersUpdated(did, id, signers, add_signers));
        Ok(())
    }

    fn base_reject_instruction(
        origin: T::RuntimeOrigin,
        instruction_id: InstructionId,
        portfolio: Option<PortfolioId>,
        instruction_count: Option<AssetCount>,
    ) -> DispatchResultWithPostInfo {
        // Makes sure the instruction exists
        ensure!(
            Self::instruction_status(instruction_id) != InstructionStatus::Unknown,
            Error::<T>::UnknownInstruction
        );
        // Get all legs for the instruction
        let legs: Vec<(LegId, Leg)> = InstructionLegs::iter_prefix(&instruction_id).collect();
        let instruction_asset_count = AssetCount::from_legs(&legs);
        // If the fee was estimated in advance, the input values must be at least equal to the actual values
        if let Some(instruction_count) = instruction_count {
            Self::ensure_valid_cost(&instruction_asset_count, &instruction_count)?;
        }
        // Check if the caller is a mediator or a portfolio owner
        let origin_data = Identity::<T>::ensure_origin_call_permissions(origin)?;
        let actual_weight = {
            match portfolio {
                Some(portfolio) => {
                    // The portfolio must be present in at least one leg
                    ensure!(
                        Self::is_portfolio_present(&legs, &portfolio),
                        Error::<T>::CallerIsNotAParty
                    );
                    // The caller must have the right permissions to the portfolio
                    T::Portfolio::ensure_portfolio_custody_and_permission(
                        portfolio,
                        origin_data.primary_did,
                        origin_data.secondary_key.as_ref(),
                    )?;
                    Self::reject_instruction_weight(instruction_asset_count, false)
                }
                None => {
                    // The caller must be a mediator
                    ensure!(
                        InstructionMediatorsAffirmations::<T>::get(
                            instruction_id,
                            origin_data.primary_did
                        ) != MediatorAffirmationStatus::Unknown,
                        Error::<T>::CallerIsNotAMediator
                    );
                    Self::reject_instruction_weight(instruction_asset_count, true)
                }
            }
        };
        // All checks have been made - write to storage
        Self::release_locks(instruction_id, &legs)?;
        let _ = T::Scheduler::cancel_named(instruction_id.execution_name());
        // Remove all data from storage
        Self::prune_rejected_instruction(instruction_id);
        Self::deposit_event(RawEvent::InstructionRejected(
            origin_data.primary_did,
            instruction_id,
        ));
        // Return the actual weight for the call
        Ok(PostDispatchInfo::from(Some(actual_weight)))
    }

    /// Returns `Ok` if the number of fungible, nonfungible and offchain assets is under the input given by the user.
    fn ensure_valid_cost(real_cost: &AssetCount, input_cost: &AssetCount) -> DispatchResult {
        // Verifies if the number of nfts being transferred is under the limit
        ensure!(
            real_cost.non_fungible() <= input_cost.non_fungible(),
            Error::<T>::NumberOfTransferredNFTsUnderestimated
        );
        // Verifies if the number of fungible transfers is under the limit
        ensure!(
            real_cost.fungible() <= input_cost.fungible(),
            Error::<T>::NumberOfFungibleTransfersUnderestimated
        );
        // Verifies if the number of off-chain assets is under the limit
        ensure!(
            real_cost.off_chain() <= input_cost.off_chain(),
            Error::<T>::NumberOfOffChainTransfersUnderestimated
        );
        Ok(())
    }

    /// Ensures that all tickers in the instruction that have venue filtering enabled are also
    /// in the venue allowed list.
    fn ensure_allowed_venue(
        instruction_legs: &[(LegId, Leg)],
        venue_id: Option<VenueId>,
    ) -> DispatchResult {
        if let Some(_) = venue_id {
            // Avoids reading the storage multiple times for the same asset_id
            let mut tickers: BTreeSet<AssetId> = BTreeSet::new();
            for (_, leg) in instruction_legs {
                if let Some(asset_id) = leg.asset_id() {
                    Self::ensure_venue_filtering(&mut tickers, *asset_id, &venue_id)?;
                }
            }
        }
        Ok(())
    }

    /// If `tickers` doesn't contain the given `asset_id` and venue_filtering is enabled, ensures that venue_id is in the allowed list
    fn ensure_venue_filtering(
        tickers: &mut BTreeSet<AssetId>,
        asset_id: AssetId,
        venue_id: &Option<VenueId>,
    ) -> DispatchResult {
        if let Some(venue_id) = venue_id {
            if tickers.insert(asset_id) && Self::venue_filtering(asset_id) {
                ensure!(
                    Self::venue_allow_list(asset_id, venue_id),
                    Error::<T>::UnauthorizedVenue
                );
            }
        }
        Ok(())
    }

    /// Executes the instruction of the given `id` returning the consumed weight for executing the instruction.
    fn base_execute_scheduled_instruction(
        id: InstructionId,
        weight_meter: &mut WeightMeter,
    ) -> PostDispatchInfo {
        let caller_did = SettlementDID.as_id();
        if let Err(e) = Self::execute_instruction_retryable(id, caller_did, weight_meter) {
            Self::deposit_event(RawEvent::FailedToExecuteInstruction(id, e));
        }
        PostDispatchInfo::from(Some(weight_meter.consumed()))
    }

    /// Returns `Ok` if the leg is valid, otherwise returns an error.
    /// See also: [`Module::ensure_valid_fungible_leg`], [`Module::ensure_valid_nft_leg`] and [`Module::ensure_valid_off_chain_leg`].
    fn ensure_valid_leg(
        leg: &Leg,
        venue_id: &Option<VenueId>,
        tickers: &mut BTreeSet<AssetId>,
        instruction_asset_count: &mut AssetCount,
    ) -> DispatchResult {
        match leg {
            Leg::Fungible {
                sender,
                receiver,
                asset_id,
                amount,
            } => {
                ensure!(sender.did != receiver.did, Error::<T>::SameSenderReceiver);
                Self::ensure_valid_fungible_leg(tickers, *asset_id, *amount, venue_id)?;
                instruction_asset_count
                    .try_add_fungible()
                    .map_err(|_| Error::<T>::MaxNumberOfFungibleAssetsExceeded)?;
                Ok(())
            }
            Leg::NonFungible {
                sender,
                receiver,
                nfts,
            } => {
                ensure!(sender.did != receiver.did, Error::<T>::SameSenderReceiver);
                Self::ensure_valid_nft_leg(tickers, &nfts, venue_id)?;
                instruction_asset_count
                    .try_add_non_fungible(&nfts)
                    .map_err(|_| Error::<T>::MaxNumberOfNFTsExceeded)?;
                Ok(())
            }
            Leg::OffChain {
                sender_identity,
                receiver_identity,
                amount,
                ..
            } => {
                ensure!(venue_id.is_some(), Error::<T>::OffChainAssetsMustHaveAVenue);
                Self::ensure_valid_off_chain_leg(sender_identity, receiver_identity, *amount)?;
                instruction_asset_count
                    .try_add_off_chain()
                    .map_err(|_| Error::<T>::MaxNumberOfOffChainAssetsExceeded)?;
                Ok(())
            }
        }
    }

    /// Ensures all checks needed for a fungible leg hold. This includes making sure that the `amount` being
    /// transferred is not zero, that `asset_id` exists on chain and that `venue_id` is allowed.
    fn ensure_valid_fungible_leg(
        tickers: &mut BTreeSet<AssetId>,
        asset_id: AssetId,
        amount: Balance,
        venue_id: &Option<VenueId>,
    ) -> DispatchResult {
        ensure!(amount > 0, Error::<T>::ZeroAmount);
        ensure!(
            Self::is_on_chain_asset(&asset_id),
            Error::<T>::UnexpectedOFFChainAsset
        );
        Self::ensure_venue_filtering(tickers, asset_id, venue_id)?;
        Ok(())
    }

    /// Ensures all checks needed for a non fungible leg hold. This includes making sure that the number of NFTs being
    /// transferred is within the defined limits, that there are no duplicate NFTs in the same leg, that `asset_id` exists on chain,
    /// and that `venue_id` is allowed.
    fn ensure_valid_nft_leg(
        tickers: &mut BTreeSet<AssetId>,
        nfts: &NFTs,
        venue_id: &Option<VenueId>,
    ) -> DispatchResult {
        ensure!(
            Self::is_on_chain_asset(nfts.asset_id()),
            Error::<T>::UnexpectedOFFChainAsset
        );
        <Nft<T>>::ensure_within_nfts_transfer_limits(&nfts)?;
        <Nft<T>>::ensure_no_duplicate_nfts(&nfts)?;
        Self::ensure_venue_filtering(tickers, nfts.asset_id().clone(), venue_id)?;
        Ok(())
    }

    /// Ensures all checks needed for an off-chain asset leg hold. This includes making sure that the `amount` being
    /// transferred is not zero, and that `sender_identity` and `receiver_identity` are not the same.
    fn ensure_valid_off_chain_leg(
        sender_identity: &IdentityId,
        receiver_identity: &IdentityId,
        amount: Balance,
    ) -> DispatchResult {
        ensure!(amount > 0, Error::<T>::ZeroAmount);
        ensure!(
            sender_identity != receiver_identity,
            Error::<T>::SameSenderReceiver
        );
        Ok(())
    }

    /// Ensures that the number of fungible, non-fungible and offchain transfers is less or equal
    /// to the maximum allowed in an instruction.
    fn ensure_within_instruction_max(instruction_asset_count: &AssetCount) -> DispatchResult {
        ensure!(
            instruction_asset_count.non_fungible() <= T::MaxNumberOfNFTs::get(),
            Error::<T>::MaxNumberOfNFTsExceeded
        );
        ensure!(
            instruction_asset_count.fungible() <= T::MaxNumberOfFungibleAssets::get(),
            Error::<T>::MaxNumberOfFungibleAssetsExceeded
        );
        ensure!(
            instruction_asset_count.off_chain() <= T::MaxNumberOfOffChainAssets::get(),
            Error::<T>::MaxNumberOfOffChainAssetsExceeded
        );
        Ok(())
    }

    /// Returns true if the asset_id is on-chain and false otherwise.
    fn is_on_chain_asset(asset_id: &AssetId) -> bool {
        pallet_asset::Assets::contains_key(asset_id)
    }

    fn base_execute_manual_instruction(
        origin: T::RuntimeOrigin,
        id: InstructionId,
        portfolio: Option<PortfolioId>,
        input_cost: &AssetCount,
        weight_meter: &mut WeightMeter,
    ) -> DispatchResultWithPostInfo {
        // check origin has the permissions required and valid instruction
        let (caller_did, sk, instruction_details) =
            Self::ensure_origin_perm_and_instruction_validity(origin, id, true)?;

        let instruction_legs: Vec<(LegId, Leg)> = InstructionLegs::iter_prefix(&id).collect();
        match portfolio {
            Some(portfolio) => {
                // Ensure that the caller is a party of this instruction
                T::Portfolio::ensure_portfolio_custody_and_permission(
                    portfolio,
                    caller_did,
                    sk.as_ref(),
                )?;
                ensure!(
                    Self::is_portfolio_present(&instruction_legs, &portfolio),
                    Error::<T>::CallerIsNotAParty
                );
            }
            None => {
                // If the caller is not the venue creator, they should be a counter party in an offchain leg
                match instruction_details.venue_id {
                    Some(venue_id) => {
                        if Self::venue_for_management(venue_id, caller_did).is_err() {
                            ensure!(
                                Self::is_offchain_party(&instruction_legs, &caller_did),
                                Error::<T>::Unauthorized
                            );
                        };
                    }
                    None => {
                        ensure!(
                            Self::is_offchain_party(&instruction_legs, &caller_did),
                            Error::<T>::Unauthorized
                        );
                    }
                }
            }
        }

        let instruction_asset_count = AssetCount::from_legs(&instruction_legs);
        Self::ensure_valid_cost(&instruction_asset_count, input_cost)?;

        Self::execute_instruction_retryable(id, caller_did, weight_meter)?;
        Self::deposit_event(RawEvent::SettlementManuallyExecuted(caller_did, id));

        Ok(PostDispatchInfo::from(Some(weight_meter.consumed())))
    }

    /// Returns `Ok` if `origin` represents the root, otherwise returns an `Err` with the consumed weight for this function.
    fn ensure_root_origin(origin: T::RuntimeOrigin) -> Result<(), DispatchErrorWithPostInfo> {
        ensure_root(origin).map_err(|e| DispatchErrorWithPostInfo {
            post_info: Some(<T as Config>::WeightInfo::ensure_root_origin()).into(),
            error: e.into(),
        })
    }

    /// Returns `true` if the given `portfolio_id` is a party in the given `instruction_set`, otherwise returns `false`.
    fn is_portfolio_present(instruction_set: &[(LegId, Leg)], portfolio_id: &PortfolioId) -> bool {
        for (_, leg) in instruction_set {
            match leg {
                Leg::Fungible {
                    sender, receiver, ..
                }
                | Leg::NonFungible {
                    sender, receiver, ..
                } => {
                    if sender == portfolio_id || receiver == portfolio_id {
                        return true;
                    }
                }
                Leg::OffChain { .. } => continue,
            }
        }
        false
    }

    /// Returns `true` if the given `caller_did` is a party in any [`Leg::OffChain`] in the `instruction_set`.
    fn is_offchain_party(instruction_set: &[(LegId, Leg)], caller_did: &IdentityId) -> bool {
        for (_, leg) in instruction_set {
            if let Leg::OffChain {
                sender_identity,
                receiver_identity,
                ..
            } = leg
            {
                if sender_identity == caller_did || receiver_identity == caller_did {
                    return true;
                }
            }
        }
        false
    }

    /// Ensures the all receipts are valid. A receipt is considered valid if the signer is allowed by the venue,
    /// if the receipt has not been used before, if the receipt's `leg_id` and `instruction_id` are referencing the
    /// correct instruction/leg and if its signature is valid.
    fn ensure_valid_receipts_details(
        venue_id: Option<VenueId>,
        instruction_id: InstructionId,
        receipts_details: &[ReceiptDetails<T::AccountId, T::OffChainSignature>],
    ) -> DispatchResult {
        let mut unique_signers_uid_set = BTreeSet::new();
        let mut unique_legs = BTreeSet::new();
        for receipt_details in receipts_details {
            ensure!(
                receipt_details.instruction_id() == &instruction_id,
                Error::<T>::ReceiptInstructionIdMissmatch
            );
            ensure!(
                unique_signers_uid_set
                    .insert((receipt_details.signer().clone(), receipt_details.uid())),
                Error::<T>::DuplicateReceiptUid
            );
            ensure!(
                unique_legs.insert(receipt_details.leg_id()),
                Error::<T>::MultipleReceiptsForOneLeg
            );

            if let Some(venue_id) = venue_id {
                ensure!(
                    Self::venue_signers(venue_id, receipt_details.signer()),
                    Error::<T>::UnauthorizedSigner
                );
            }

            ensure!(
                !Self::receipts_used(receipt_details.signer(), &receipt_details.uid()),
                Error::<T>::ReceiptAlreadyClaimed
            );

            let leg = InstructionLegs::get(&instruction_id, &receipt_details.leg_id())
                .ok_or(Error::<T>::LegNotFound)?;
            match leg {
                Leg::OffChain {
                    sender_identity,
                    receiver_identity,
                    ticker,
                    amount,
                } => {
                    ensure!(
                        OffChainAffirmations::get(instruction_id, receipt_details.leg_id())
                            == AffirmationStatus::Pending,
                        Error::<T>::UnexpectedAffirmationStatus
                    );
                    let receipt = Receipt::new(
                        receipt_details.uid(),
                        instruction_id,
                        receipt_details.leg_id(),
                        sender_identity,
                        receiver_identity,
                        ticker,
                        amount,
                    );
                    ensure!(
                        receipt_details
                            .signature()
                            .verify(&receipt.encode()[..], receipt_details.signer()),
                        Error::<T>::InvalidSignature
                    );
                }
                Leg::Fungible { .. } | Leg::NonFungible { .. } => {
                    return Err(Error::<T>::ReceiptForInvalidLegType.into())
                }
            }
        }
        Ok(())
    }

    /// Returns [`WeightMeter`] if the provided `weight_limit` is greater than `minimum_weight`, otherwise returns an error.
    fn ensure_valid_weight_meter(
        minimum_weight: Weight,
        weight_limit: Weight,
    ) -> Result<WeightMeter, DispatchErrorWithPostInfo> {
        WeightMeter::from_limit(minimum_weight, weight_limit).map_err(|_| {
            DispatchErrorWithPostInfo {
                post_info: Some(weight_limit).into(),
                error: Error::<T>::InputWeightIsLessThanMinimum.into(),
            }
        })
    }

    fn base_withdraw_affirmation(
        origin: T::RuntimeOrigin,
        id: InstructionId,
        portfolios: BTreeSet<PortfolioId>,
        affirmation_count: Option<AffirmationCount>,
    ) -> DispatchResultWithPostInfo {
        let (did, secondary_key, details) =
            Self::ensure_origin_perm_and_instruction_validity(origin, id, false)?;
        let filtered_legs = Self::unsafe_withdraw_instruction_affirmation(
            did,
            id,
            portfolios,
            secondary_key.as_ref(),
            affirmation_count,
        )?;
        if details.settlement_type == SettlementType::SettleOnAffirmation {
            // Cancel the scheduled task for the execution of a given instruction.
            let _fix_this = T::Scheduler::cancel_named(id.execution_name());
        }
        Ok(PostDispatchInfo::from(Some(
            Self::withdraw_affirmation_actual_weight(
                filtered_legs.sender_asset_count().clone(),
                filtered_legs.receiver_asset_count().clone(),
                filtered_legs.unfiltered_asset_count().off_chain(),
            ),
        )))
    }

    /// Returns `Ok` if the number of assets in [`AffirmationCount`] is greater or equal to the actual number of assets.
    fn ensure_valid_affirmation_count(
        filtered_legs: &FilteredLegs,
        affirmation_count: &AffirmationCount,
    ) -> DispatchResult {
        Self::ensure_valid_cost(
            filtered_legs.sender_asset_count(),
            affirmation_count.sender_asset_count(),
        )?;
        Self::ensure_valid_cost(
            filtered_legs.receiver_asset_count(),
            affirmation_count.receiver_asset_count(),
        )?;
        // Verifies if the number of off-chain assets is under the limit
        ensure!(
            filtered_legs.unfiltered_asset_count().off_chain()
                <= affirmation_count.offchain_count(),
            Error::<T>::NumberOfOffChainTransfersUnderestimated
        );
        Ok(())
    }

    /// Affirms the instruction as a mediator.
    fn base_affirm_instruction_as_mediator(
        origin: T::RuntimeOrigin,
        instruction_id: InstructionId,
        expiry: Option<T::Moment>,
    ) -> DispatchResult {
        let (caller_did, _, instruction) =
            Self::ensure_origin_perm_and_instruction_validity(origin, instruction_id, false)?;

        // Verifies if the caller is a mediator
        let mediator_affirmation_status =
            InstructionMediatorsAffirmations::<T>::get(instruction_id, caller_did);
        ensure!(
            mediator_affirmation_status != MediatorAffirmationStatus::Unknown,
            Error::<T>::CallerIsNotAMediator
        );

        // Verifies if the expiry date is in the future
        if let Some(expiry) = expiry {
            ensure!(
                expiry > <pallet_timestamp::Pallet<T>>::get(),
                Error::<T>::InvalidExpiryDate
            );
        }

        // Updates the mediator's affirmation status to affirmed
        InstructionMediatorsAffirmations::<T>::insert(
            instruction_id,
            caller_did,
            MediatorAffirmationStatus::Affirmed { expiry },
        );
        // If the mediator is not reaffirming the instruction, the number of pending affirmation must be updated
        if MediatorAffirmationStatus::Pending == mediator_affirmation_status {
            InstructionAffirmsPending::mutate(instruction_id, |n| *n = n.saturating_sub(1));
        }
        // If all affirmations have been received, the instruction will be scheduled for the next block
        let n_pending_affirmations = InstructionAffirmsPending::get(instruction_id);
        if n_pending_affirmations == 0
            && instruction.settlement_type == SettlementType::SettleOnAffirmation
        {
            let instruction_asset_count = Self::get_instruction_asset_count(&instruction_id);
            let weight_limit = Self::execute_scheduled_instruction_weight_limit(
                instruction_asset_count.fungible(),
                instruction_asset_count.non_fungible(),
                instruction_asset_count.off_chain(),
            );
            Self::maybe_schedule_instruction(n_pending_affirmations, instruction_id, weight_limit);
        }

        Self::deposit_event(RawEvent::MediatorAffirmationReceived(
            caller_did,
            instruction_id,
            expiry,
        ));
        Ok(())
    }

    /// Removes the mediator's affirmation for the instruction
    fn base_withdraw_affirmation_as_mediator(
        origin: T::RuntimeOrigin,
        instruction_id: InstructionId,
    ) -> DispatchResult {
        let (caller_did, _, instruction) =
            Self::ensure_origin_perm_and_instruction_validity(origin, instruction_id, false)?;

        // Verifies if the caller is a mediator and has already affirmed the instruction
        let mediator_affirmation_status =
            InstructionMediatorsAffirmations::<T>::get(instruction_id, caller_did);
        match mediator_affirmation_status {
            MediatorAffirmationStatus::Unknown => {
                return Err(Error::<T>::CallerIsNotAMediator.into())
            }
            MediatorAffirmationStatus::Pending => {
                return Err(Error::<T>::UnexpectedAffirmationStatus.into())
            }
            MediatorAffirmationStatus::Affirmed { .. } => {}
        }

        // Updates the mediator's affirmation status to pending and add one to the number of pending affirmations
        InstructionMediatorsAffirmations::<T>::insert(
            instruction_id,
            caller_did,
            MediatorAffirmationStatus::Pending,
        );
        let n_pending_before_withdrawal = InstructionAffirmsPending::mutate(instruction_id, |n| {
            let before = n.clone();
            *n = n.saturating_add(1);
            before
        });
        if n_pending_before_withdrawal == 0
            && instruction.settlement_type == SettlementType::SettleOnAffirmation
        {
            // Cancel the scheduled task
            let _ = T::Scheduler::cancel_named(instruction_id.execution_name());
        }
        Self::deposit_event(RawEvent::MediatorAffirmationWithdrawn(
            caller_did,
            instruction_id,
        ));
        Ok(())
    }

    /// Returns the worst case weight for an instruction with `f` fungible legs, `n` nfts being transferred and `o` offchain assets.
    fn execute_scheduled_instruction_weight_limit(f: u32, n: u32, o: u32) -> Weight {
        <T as Config>::WeightInfo::execute_scheduled_instruction(f, n, o)
    }

    /// Returns the minimum weight for calling the `execute_scheduled_instruction` function.
    fn execute_scheduled_instruction_minimum_weight() -> Weight {
        <T as Config>::WeightInfo::execute_scheduled_instruction(0, 0, 0)
    }

    /// Returns the worst case weight for an instruction with `f` fungible legs, `n` nfts being transferred and `o` offchain assets.
    fn execute_manual_instruction_weight_limit(f: u32, n: u32, o: u32) -> Weight {
        <T as Config>::WeightInfo::execute_manual_instruction(f, n, o)
    }

    /// Returns the minimum weight for calling the `execute_manual_instruction` extrinsic.
    pub fn execute_manual_instruction_minimum_weight() -> Weight {
        <T as Config>::WeightInfo::execute_manual_instruction(0, 0, 0)
    }

    /// Returns the weight for calling `affirm_with_receipts` while considering the `sender_asset_count` for the sender, `receiver_asset_count`
    /// for the receiver, and `n_offchain` offchain legs.
    fn affirm_with_receipts_actual_weight(
        sender_asset_count: AssetCount,
        receiver_asset_count: AssetCount,
        n_offchain: u32,
    ) -> Weight {
        let affirmation_count =
            AffirmationCount::new(sender_asset_count, receiver_asset_count, n_offchain);
        <T as Config>::WeightInfo::affirm_with_receipts_input(Some(affirmation_count), 0)
    }

    /// Returns the weight for calling `affirm_instruction` while considering the `sender_asset_count` for the sender and`receiver_asset_count`
    /// for the receiver.
    fn affirm_instruction_actual_weight(
        sender_asset_count: AssetCount,
        receiver_asset_count: AssetCount,
    ) -> Weight {
        let affirmation_count = AffirmationCount::new(sender_asset_count, receiver_asset_count, 0);
        <T as Config>::WeightInfo::affirm_instruction_input(Some(affirmation_count), 0)
    }

    /// Returns the weight for calling `withdraw_affirmation` while considering the `sender_asset_count` for the sender and`receiver_asset_count`
    /// for the receiver, and `n_offchain` offchain legs.
    fn withdraw_affirmation_actual_weight(
        sender_asset_count: AssetCount,
        receiver_asset_count: AssetCount,
        n_offchain: u32,
    ) -> Weight {
        let affirmation_count =
            AffirmationCount::new(sender_asset_count, receiver_asset_count, n_offchain);
        <T as Config>::WeightInfo::withdraw_affirmation_input(Some(affirmation_count), 0)
    }

    /// Returns the weight for calling `reject_instruction_weight` with the number of assets in `instruction_asset_count`.
    fn reject_instruction_weight(instruction_asset_count: AssetCount, as_mediator: bool) -> Weight {
        <T as Config>::WeightInfo::reject_instruction_input(
            Some(instruction_asset_count),
            as_mediator,
        )
    }

    pub fn get_actual_weight(call: &Call<T>) -> Option<Weight> {
        match call {
            Call::affirm_instruction { id, portfolios } => {
                let filtered_legs = Self::filtered_legs(*id, &portfolios);
                Some(Self::affirm_instruction_actual_weight(
                    *filtered_legs.sender_asset_count(),
                    *filtered_legs.receiver_asset_count(),
                ))
            }
            Call::affirm_with_receipts { id, portfolios, .. } => {
                let filtered_legs = Self::filtered_legs(*id, &portfolios);
                Some(Self::affirm_with_receipts_actual_weight(
                    *filtered_legs.sender_asset_count(),
                    *filtered_legs.receiver_asset_count(),
                    filtered_legs.unfiltered_asset_count().off_chain(),
                ))
            }
            Call::withdraw_affirmation { id, portfolios } => {
                let filtered_legs = Self::filtered_legs(*id, &portfolios);
                Some(Self::withdraw_affirmation_actual_weight(
                    *filtered_legs.sender_asset_count(),
                    *filtered_legs.receiver_asset_count(),
                    filtered_legs.unfiltered_asset_count().off_chain(),
                ))
            }
            Call::reject_instruction { id, .. } => {
                let asset_count = Self::get_instruction_asset_count(id);
                Some(Self::reject_instruction_weight(asset_count, false))
            }
            _ => None,
        }
    }

    /// Returns an instance of [`ExecuteInstructionInfo`].
    pub fn execute_instruction_info(
        instruction_id: &InstructionId,
    ) -> Option<ExecuteInstructionInfo> {
        if !InstructionDetails::<T>::contains_key(instruction_id) {
            return None;
        }

        let caller_did = SettlementDID.as_id();
        let instruction_asset_count = Self::get_instruction_asset_count(instruction_id);
        let mut weight_meter =
            WeightMeter::max_limit(Self::execute_manual_instruction_minimum_weight());
        match Self::execute_instruction_retryable(*instruction_id, caller_did, &mut weight_meter) {
            Ok(_) => Some(ExecuteInstructionInfo::new(
                instruction_asset_count.fungible(),
                instruction_asset_count.non_fungible(),
                instruction_asset_count.off_chain(),
                weight_meter.consumed(),
                None,
            )),
            Err(e) => Some(ExecuteInstructionInfo::new(
                instruction_asset_count.fungible(),
                instruction_asset_count.non_fungible(),
                instruction_asset_count.off_chain(),
                weight_meter.consumed(),
                Some(e.into()),
            )),
        }
    }

    /// Returns an instance of [`AffirmationCount`].
    pub fn affirmation_count(
        instruction_id: InstructionId,
        portfolios: Vec<PortfolioId>,
    ) -> AffirmationCount {
        let portfolios = portfolios.into_iter().collect::<BTreeSet<_>>();
        let filtered_legs = Self::filtered_legs(instruction_id, &portfolios);
        AffirmationCount::new(
            filtered_legs.sender_asset_count().clone(),
            filtered_legs.receiver_asset_count().clone(),
            filtered_legs.unfiltered_asset_count().off_chain(),
        )
    }

    /// Returns a vector containing all errors for the transfer. An empty vec means there's no error.
    #[rustfmt::skip]
    pub fn transfer_report(
        leg: Leg,
        skip_locked_check: bool,
        weight_meter: &mut WeightMeter,
    ) -> Vec<DispatchError> {
        match leg {
            Leg::Fungible { sender, receiver, asset_id, amount } => {
                <Asset<T>>::asset_transfer_report(
                    &sender,
                    &receiver,
                    &asset_id,
                    amount,
                    skip_locked_check,
                    weight_meter,
                )
            }
            Leg::NonFungible { sender, receiver, nfts} => {
                <Nft<T>>::nft_transfer_report(
                    &sender,
                    &receiver,
                    &nfts,
                    skip_locked_check,
                    weight_meter
                )
            }
            Leg::OffChain { .. } => {
                Vec::new()
            },
        }
    }

    /// Returns a vector containing all errors for the execution. An empty vec means there's no error.
    pub fn execute_instruction_report(
        instruction_id: &InstructionId,
        weight_meter: &mut WeightMeter,
    ) -> Vec<DispatchError> {
        let mut execution_errors = Vec::new();

        if Self::instruction_affirms_pending(instruction_id) != 0 {
            execution_errors.push(Error::<T>::NotAllAffirmationsHaveBeenReceived.into());
        }

        if let Err(e) = Self::ensure_non_expired_affirmations(&instruction_id) {
            execution_errors.push(e);
        }

        match Self::instruction_status(instruction_id) {
            InstructionStatus::Unknown
            | InstructionStatus::Success(_)
            | InstructionStatus::Rejected(_) => {
                execution_errors.push(Error::<T>::InvalidInstructionStatusForExecution.into());
            }
            InstructionStatus::Pending | InstructionStatus::Failed => {}
        }

        let instruction_legs: Vec<(LegId, Leg)> =
            InstructionLegs::iter_prefix(&instruction_id).collect();
        let venue_id = Self::instruction_details(instruction_id).venue_id;
        if let Err(e) = Self::ensure_allowed_venue(&instruction_legs, venue_id) {
            execution_errors.push(e);
        }

        for (leg_id, leg) in instruction_legs {
            let leg_status = Self::instruction_leg_status(instruction_id, leg_id);
            if leg_status == LegStatus::ExecutionPending {
                let transfer_errors = Self::transfer_report(leg, true, weight_meter);
                execution_errors.extend_from_slice(&transfer_errors);
            }
        }

        execution_errors
    }
}
