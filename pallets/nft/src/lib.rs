#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::dispatch::{
    DispatchError, DispatchResult, DispatchResultWithPostInfo, PostDispatchInfo,
};
use frame_support::storage::StorageDoubleMap;
use frame_support::traits::Get;
use frame_support::weights::Weight;
use frame_support::{decl_error, decl_module, decl_storage, ensure, require_transactional};
use sp_std::collections::btree_map::BTreeMap;
use sp_std::collections::btree_set::BTreeSet;
use sp_std::{vec, vec::Vec};

use pallet_asset::Frozen;
use pallet_base::try_next_pre;
use pallet_portfolio::{PortfolioLockedNFT, PortfolioNFT};
use polymesh_common_utilities::compliance_manager::ComplianceFnConfig;
pub use polymesh_common_utilities::traits::nft::{Config, Event, NFTTrait, WeightInfo};
use polymesh_primitives::asset::{AssetId, AssetName, AssetType, NonFungibleType};
use polymesh_primitives::asset_metadata::{AssetMetadataKey, AssetMetadataValue};
use polymesh_primitives::nft::{
    NFTCollection, NFTCollectionId, NFTCollectionKeys, NFTCount, NFTId, NFTMetadataAttribute, NFTs,
};
use polymesh_primitives::settlement::InstructionId;
use polymesh_primitives::{
    storage_migrate_on, storage_migration_ver, IdentityId, Memo, PortfolioId, PortfolioKind,
    PortfolioUpdateReason, WeightMeter,
};

type Asset<T> = pallet_asset::Module<T>;
type ExternalAgents<T> = pallet_external_agents::Module<T>;
type Identity<T> = pallet_identity::Module<T>;
type Portfolio<T> = pallet_portfolio::Module<T>;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;
mod migrations;

storage_migration_ver!(4);

decl_storage!(
    trait Store for Module<T: Config> as NFT {
        /// The total number of NFTs per identity.
        pub NumberOfNFTs get(fn balance_of): double_map hasher(blake2_128_concat) AssetId, hasher(identity) IdentityId => NFTCount;

        /// The collection id corresponding to each asset.
        pub CollectionAsset get(fn collection_asset): map hasher(blake2_128_concat) AssetId => NFTCollectionId;

        /// All collection details for a given collection id.
        pub Collection get(fn nft_collection): map hasher(blake2_128_concat) NFTCollectionId => NFTCollection;

        /// All mandatory metadata keys for a given collection.
        pub CollectionKeys get(fn collection_keys): map hasher(blake2_128_concat) NFTCollectionId => BTreeSet<AssetMetadataKey>;

        /// The metadata value of an nft given its collection id, token id and metadata key.
        pub MetadataValue get(fn metadata_value):
            double_map hasher(blake2_128_concat) (NFTCollectionId, NFTId), hasher(blake2_128_concat) AssetMetadataKey => AssetMetadataValue;

        /// The total number of NFTs in a collection
        pub NFTsInCollection get(fn nfts_in_collection): map hasher(blake2_128_concat) AssetId => NFTCount;

        /// Tracks the owner of an NFT
        pub NFTOwner get(fn nft_owner): double_map hasher(blake2_128_concat) AssetId, hasher(blake2_128_concat) NFTId => Option<PortfolioId>;

        /// The last `NFTId` used for an NFT.
        pub CurrentNFTId get(fn current_nft_id): map hasher(blake2_128_concat) NFTCollectionId => Option<NFTId>;

        /// The last `NFTCollectionId` used for a collection.
        pub CurrentCollectionId get(fn current_collection_id): Option<NFTCollectionId>;

        /// Storage version.
        StorageVersion get(fn storage_version) build(|_| Version::new(4)): Version;
    }
);

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::RuntimeOrigin {

        type Error = Error<T>;

        const MaxNumberOfCollectionKeys: u8 = T::MaxNumberOfCollectionKeys::get();
        const MaxNumberOfNFTsCount: u32 = T::MaxNumberOfNFTsCount::get();

        /// Initializes the default event for this module.
        fn deposit_event() = default;

        fn on_runtime_upgrade() -> Weight {
            storage_migrate_on!(StorageVersion, 4, {
                migrations::migrate_to_v4::<T>();
            });
            Weight::zero()
        }

        /// Cretes a new `NFTCollection`.
        ///
        /// # Arguments
        /// * `origin` - contains the secondary key of the caller (i.e. who signed the transaction to execute this function).
        /// * `asset_id` - optional [`AssetId`] associated to the new collection. `None` will create a new asset.
        /// * `nft_type` - in case the asset hasn't been created yet, one will be created with the given type.
        /// * `collection_keys` - all mandatory metadata keys that the tokens in the collection must have.
        ///
        /// ## Errors
        /// - `CollectionAlredyRegistered` - if the asset_id is already associated to an NFT collection.
        /// - `InvalidAssetType` - if the associated asset is not of type NFT.
        /// - `MaxNumberOfKeysExceeded` - if the number of metadata keys for the collection is greater than the maximum allowed.
        /// - `UnregisteredMetadataKey` - if any of the metadata keys needed for the collection has not been registered.
        /// - `DuplicateMetadataKey` - if a duplicate metadata keys has been passed as input.
        ///
        /// # Permissions
        /// * Asset
        #[weight = <T as Config>::WeightInfo::create_nft_collection(collection_keys.len() as u32)]
        pub fn create_nft_collection(
            origin,
            asset_id: Option<AssetId>,
            nft_type: Option<NonFungibleType>,
            collection_keys: NFTCollectionKeys
        ) -> DispatchResult {
            Self::base_create_nft_collection(origin, asset_id, nft_type, collection_keys)
        }

        /// Issues an NFT to the caller.
        ///
        /// # Arguments
        /// * `origin` - is a signer that has permissions to act as an agent of `asset_id`.
        /// * `asset_id` - the [`AssetId`] of the NFT collection.
        /// * `nft_metadata_attributes` - all mandatory metadata keys and values for the NFT.
        /// - `portfolio_kind` - the portfolio that will receive the minted nft.
        ///
        /// ## Errors
        /// - `CollectionNotFound` - if the collection associated to the given asset_id has not been created.
        /// - `InvalidMetadataAttribute` - if the number of attributes is not equal to the number set in the collection or attempting to set a value for a key not definied in the collection.
        /// - `DuplicateMetadataKey` - if a duplicate metadata keys has been passed as input.
        ///
        ///
        /// # Permissions
        /// * Asset
        /// * Portfolio
        #[weight = <T as Config>::WeightInfo::issue_nft(nft_metadata_attributes.len() as u32)]
        pub fn issue_nft(origin, asset_id: AssetId, nft_metadata_attributes: Vec<NFTMetadataAttribute>, portfolio_kind: PortfolioKind) -> DispatchResult {
            Self::base_issue_nft(origin, asset_id, nft_metadata_attributes, portfolio_kind)
        }

        /// Redeems the given NFT from the caller's portfolio.
        ///
        /// # Arguments
        /// * `origin` - is a signer that has permissions to act as an agent of `asset_id`.
        /// * `asset_id` - the [`AssetId`] of the NFT collection.
        /// * `nft_id` - the id of the NFT to be burned.
        /// * `portfolio_kind` - the portfolio that contains the nft.
        ///
        /// ## Errors
        /// - `CollectionNotFound` - if the collection associated to the given asset_id has not been created.
        /// - `NFTNotFound` - if the given NFT does not exist in the portfolio.
        ///
        /// # Permissions
        /// * Asset
        /// * Portfolio
        #[weight = <T as Config>::WeightInfo::redeem_nft(
            number_of_keys.map_or(
                u32::from(T::MaxNumberOfCollectionKeys::get()),
                |v| u32::from(v)
            )
        )]
        pub fn redeem_nft(
            origin,
            asset_id: AssetId,
            nft_id: NFTId,
            portfolio_kind: PortfolioKind,
            number_of_keys: Option<u8>
        ) -> DispatchResultWithPostInfo {
            Self::base_redeem_nft(origin, asset_id, nft_id, portfolio_kind, number_of_keys)
        }

        /// Forces the transfer of NFTs from a given portfolio to the caller's portfolio.
        ///
        /// # Arguments
        /// * `origin` - is a signer that has permissions to act as an agent of `asset_id`.
        /// * `nft_id` - the [`NFTId`] of the NFT to be transferred.
        /// * `source_portfolio` - the [`PortfolioId`] that currently holds the NFT.
        /// * `callers_portfolio_kind` - the [`PortfolioKind`] of the caller's portfolio.
        ///
        /// # Permissions
        /// * Asset
        /// * Portfolio
        #[weight = <T as Config>::WeightInfo::controller_transfer(nfts.len() as u32)]
        pub fn controller_transfer(
            origin,
            nfts: NFTs,
            source_portfolio: PortfolioId,
            callers_portfolio_kind: PortfolioKind
        ) -> DispatchResult {
            Self::base_controller_transfer(origin, nfts, source_portfolio, callers_portfolio_kind)
        }
    }
}

decl_error! {
    pub enum Error for Module<T: Config> {
        /// An overflow while calculating the balance.
        BalanceOverflow,
        /// An underflow while calculating the balance.
        BalanceUnderflow,
        /// The asset_id is already associated to an NFT collection.
        CollectionAlredyRegistered,
        /// The NFT collection does not exist.
        CollectionNotFound,
        /// A duplicate metadata key has been passed as parameter.
        DuplicateMetadataKey,
        /// Duplicate ids are not allowed.
        DuplicatedNFTId,
        /// The asset must be of type non-fungible.
        InvalidAssetType,
        /// Either the number of keys or the key identifier does not match the keys defined for the collection.
        InvalidMetadataAttribute,
        /// Failed to transfer an NFT - NFT collection not found.
        InvalidNFTTransferCollectionNotFound,
        /// Failed to transfer an NFT - attempt to move to the same portfolio.
        InvalidNFTTransferSamePortfolio,
        /// Failed to transfer an NFT - NFT not found in portfolio.
        InvalidNFTTransferNFTNotOwned,
        /// Failed to transfer an NFT - identity count would overflow.
        InvalidNFTTransferCountOverflow,
        /// Failed to transfer an NFT - compliance failed.
        InvalidNFTTransferComplianceFailure,
        /// Failed to transfer an NFT - asset is frozen.
        InvalidNFTTransferFrozenAsset,
        /// Failed to transfer an NFT - the number of nfts in the identity is insufficient.
        InvalidNFTTransferInsufficientCount,
        /// The maximum number of metadata keys was exceeded.
        MaxNumberOfKeysExceeded,
        /// The maximum number of nfts being transferred in one leg was exceeded.
        MaxNumberOfNFTsPerLegExceeded,
        /// The NFT does not exist.
        NFTNotFound,
        /// At least one of the metadata keys has not been registered.
        UnregisteredMetadataKey,
        /// It is not possible to transferr zero nft.
        ZeroCount,
        /// An overflow while calculating the updated supply.
        SupplyOverflow,
        /// An underflow while calculating the updated supply.
        SupplyUnderflow,
        /// Failed to transfer an NFT - nft is locked.
        InvalidNFTTransferNFTIsLocked,
        /// The sender identity can't be the same as the receiver identity.
        InvalidNFTTransferSenderIdMatchesReceiverId,
        /// The receiver has an invalid CDD.
        InvalidNFTTransferInvalidReceiverCDD,
        /// The sender has an invalid CDD.
        InvalidNFTTransferInvalidSenderCDD,
        /// There's no asset associated to the given asset_id.
        InvalidAssetId,
        /// The NFT is locked.
        NFTIsLocked,
        /// The number of keys in the collection is greater than the input.
        NumberOfKeysIsLessThanExpected,
    }
}

impl<T: Config> Module<T> {
    fn base_create_nft_collection(
        origin: T::RuntimeOrigin,
        asset_id: Option<AssetId>,
        nft_type: Option<NonFungibleType>,
        collection_keys: NFTCollectionKeys,
    ) -> DispatchResult {
        // Verifies if the caller has asset permission and if the asset is an NFT.
        let (create_asset, caller_did, asset_id) = {
            match asset_id {
                Some(asset_id) => match Asset::<T>::nft_asset(&asset_id) {
                    Some(is_nft_asset) => {
                        ensure!(is_nft_asset, Error::<T>::InvalidAssetType);
                        let caller_did = <ExternalAgents<T>>::ensure_agent_asset_perms(
                            origin.clone(),
                            asset_id,
                        )?
                        .primary_did;
                        (false, caller_did, asset_id)
                    }
                    None => return Err(Error::<T>::InvalidAssetId.into()),
                },
                None => {
                    let caller_data =
                        Identity::<T>::ensure_origin_call_permissions(origin.clone())?;
                    let asset_id = Asset::<T>::generate_asset_id(caller_data.sender, false);
                    (true, caller_data.primary_did, asset_id)
                }
            }
        };

        // Verifies if the asset_id is already associated to an NFT collection
        ensure!(
            !CollectionAsset::contains_key(&asset_id),
            Error::<T>::CollectionAlredyRegistered
        );

        // Verifies if the maximum number of keys is respected
        ensure!(
            collection_keys.len() <= (T::MaxNumberOfCollectionKeys::get() as usize),
            Error::<T>::MaxNumberOfKeysExceeded
        );

        // Verifies that there are no duplicated keys
        let n_keys = collection_keys.len();
        let collection_keys: BTreeSet<AssetMetadataKey> = collection_keys.into_iter().collect();
        ensure!(
            n_keys == collection_keys.len(),
            Error::<T>::DuplicateMetadataKey
        );

        // Verifies that all keys have been registered
        for key in &collection_keys {
            ensure!(
                Asset::<T>::check_asset_metadata_key_exists(&asset_id, key),
                Error::<T>::UnregisteredMetadataKey
            )
        }

        // Creates an nft asset if it hasn't been created yet
        if create_asset {
            let nft_type = nft_type.ok_or(Error::<T>::InvalidAssetType)?;
            Asset::<T>::create_asset(
                origin,
                AssetName(Vec::new()),
                false,
                AssetType::NonFungible(nft_type),
                Vec::new(),
                None,
            )?;
        }

        // Creates the nft collection
        let collection_id = Self::update_current_collection_id()?;
        let nft_collection = NFTCollection::new(collection_id, asset_id.clone());
        Collection::insert(&collection_id, nft_collection);
        CollectionKeys::insert(&collection_id, collection_keys);
        CollectionAsset::insert(&asset_id, &collection_id);

        Self::deposit_event(Event::NftCollectionCreated(
            caller_did,
            asset_id,
            collection_id,
        ));
        Ok(())
    }

    fn base_issue_nft(
        origin: T::RuntimeOrigin,
        asset_id: AssetId,
        metadata_attributes: Vec<NFTMetadataAttribute>,
        portfolio_kind: PortfolioKind,
    ) -> DispatchResult {
        // Verifies if the collection exists
        let collection_id =
            CollectionAsset::try_get(&asset_id).map_err(|_| Error::<T>::CollectionNotFound)?;

        // Verifies if the caller has the right permissions (regarding asset and portfolio)
        let caller_portfolio = Asset::<T>::ensure_origin_asset_and_portfolio_permissions(
            origin,
            asset_id.clone(),
            portfolio_kind,
            false,
        )?;

        Portfolio::<T>::ensure_portfolio_validity(&caller_portfolio)?;

        // Verifies that all mandatory keys are being set and that there are no duplicated keys
        let mandatory_keys: BTreeSet<AssetMetadataKey> = Self::collection_keys(&collection_id);
        ensure!(
            mandatory_keys.len() == metadata_attributes.len(),
            Error::<T>::InvalidMetadataAttribute
        );

        let n_keys = metadata_attributes.len();
        let nft_attributes: BTreeMap<_, _> = metadata_attributes
            .into_iter()
            .map(|a| (a.key, a.value))
            .collect();
        ensure!(
            n_keys == nft_attributes.len(),
            Error::<T>::DuplicateMetadataKey
        );

        for metadata_key in nft_attributes.keys() {
            ensure!(
                mandatory_keys.contains(metadata_key),
                Error::<T>::InvalidMetadataAttribute
            );
        }

        // Mints the NFT and adds it to the caller's portfolio
        let new_supply = NFTsInCollection::get(&asset_id)
            .checked_add(1)
            .ok_or(Error::<T>::SupplyOverflow)?;
        let new_balance = NumberOfNFTs::get(&asset_id, &caller_portfolio.did)
            .checked_add(1)
            .ok_or(Error::<T>::BalanceOverflow)?;
        let nft_id = Self::update_current_nft_id(&collection_id)?;
        NFTsInCollection::insert(&asset_id, new_supply);
        NumberOfNFTs::insert(&asset_id, &caller_portfolio.did, new_balance);
        for (metadata_key, metadata_value) in nft_attributes.into_iter() {
            MetadataValue::insert((&collection_id, &nft_id), metadata_key, metadata_value);
        }
        PortfolioNFT::insert(caller_portfolio, (asset_id, nft_id), true);
        NFTOwner::insert(asset_id, nft_id, caller_portfolio);

        Self::deposit_event(Event::NFTPortfolioUpdated(
            caller_portfolio.did,
            NFTs::new_unverified(asset_id, vec![nft_id]),
            None,
            Some(caller_portfolio),
            PortfolioUpdateReason::Issued {
                funding_round_name: None,
            },
        ));
        Ok(())
    }

    fn base_redeem_nft(
        origin: T::RuntimeOrigin,
        asset_id: AssetId,
        nft_id: NFTId,
        portfolio_kind: PortfolioKind,
        number_of_keys: Option<u8>,
    ) -> DispatchResultWithPostInfo {
        // Verifies if the collection exists
        let collection_id =
            CollectionAsset::try_get(&asset_id).map_err(|_| Error::<T>::CollectionNotFound)?;

        // Ensure origin is agent with custody and permissions for portfolio.
        let caller_portfolio = Asset::<T>::ensure_origin_asset_and_portfolio_permissions(
            origin,
            asset_id,
            portfolio_kind,
            true,
        )?;

        // Verifies if the NFT exists
        ensure!(
            PortfolioNFT::contains_key(&caller_portfolio, (&asset_id, &nft_id)),
            Error::<T>::NFTNotFound
        );
        ensure!(
            !PortfolioLockedNFT::contains_key(&caller_portfolio, (&asset_id, &nft_id)),
            Error::<T>::NFTIsLocked
        );

        // Burns the NFT
        let new_supply = NFTsInCollection::get(&asset_id)
            .checked_sub(1)
            .ok_or(Error::<T>::SupplyUnderflow)?;
        let new_balance = NumberOfNFTs::get(&asset_id, &caller_portfolio.did)
            .checked_sub(1)
            .ok_or(Error::<T>::BalanceUnderflow)?;
        NFTsInCollection::insert(&asset_id, new_supply);
        NumberOfNFTs::insert(&asset_id, &caller_portfolio.did, new_balance);
        PortfolioNFT::remove(&caller_portfolio, (&asset_id, &nft_id));
        NFTOwner::remove(asset_id, nft_id);
        let removed_keys = MetadataValue::drain_prefix((&collection_id, &nft_id)).count();
        if let Some(number_of_keys) = number_of_keys {
            ensure!(
                usize::from(number_of_keys) >= removed_keys,
                Error::<T>::NumberOfKeysIsLessThanExpected,
            );
        }

        Self::deposit_event(Event::NFTPortfolioUpdated(
            caller_portfolio.did,
            NFTs::new_unverified(asset_id, vec![nft_id]),
            Some(caller_portfolio),
            None,
            PortfolioUpdateReason::Redeemed,
        ));
        Ok(PostDispatchInfo::from(Some(
            <T as Config>::WeightInfo::redeem_nft(removed_keys as u32),
        )))
    }

    /// Tranfer ownership of all NFTs.
    #[require_transactional]
    pub fn base_nft_transfer(
        sender_portfolio: PortfolioId,
        receiver_portfolio: PortfolioId,
        nfts: NFTs,
        instruction_id: InstructionId,
        instruction_memo: Option<Memo>,
        caller_did: IdentityId,
        weight_meter: &mut WeightMeter,
    ) -> DispatchResult {
        // Verifies if all rules for transfering the NFTs are being respected
        Self::validate_nft_transfer(
            &sender_portfolio,
            &receiver_portfolio,
            &nfts,
            false,
            Some(weight_meter),
        )?;

        // Transfer ownership of the NFTs
        Self::unverified_nfts_transfer(&sender_portfolio, &receiver_portfolio, &nfts);

        Self::deposit_event(Event::NFTPortfolioUpdated(
            caller_did,
            nfts,
            Some(sender_portfolio),
            Some(receiver_portfolio),
            PortfolioUpdateReason::Transferred {
                instruction_id: Some(instruction_id),
                instruction_memo,
            },
        ));
        Ok(())
    }

    /// Returns `Ok` if all rules for transferring the NFTs are satisfied.
    pub fn validate_nft_transfer(
        sender_portfolio: &PortfolioId,
        receiver_portfolio: &PortfolioId,
        nfts: &NFTs,
        is_controller_transfer: bool,
        weight_meter: Option<&mut WeightMeter>,
    ) -> DispatchResult {
        // Verifies if there is a collection associated to the NFTs
        if !CollectionAsset::contains_key(nfts.asset_id()) {
            return Err(Error::<T>::InvalidNFTTransferCollectionNotFound.into());
        }

        // Verifies that the sender and receiver are not the same
        ensure!(
            sender_portfolio.did != receiver_portfolio.did,
            Error::<T>::InvalidNFTTransferSenderIdMatchesReceiverId
        );

        // Verifies that the sender has the required nft count
        let nfts_transferred = nfts.len() as u64;
        ensure!(
            NumberOfNFTs::get(nfts.asset_id(), sender_portfolio.did) >= nfts_transferred,
            Error::<T>::InvalidNFTTransferInsufficientCount
        );

        // Verifies that the number of nfts being transferred are within the allowed limits
        Self::ensure_within_nfts_transfer_limits(nfts)?;
        // Verifies that all ids are unique
        Self::ensure_no_duplicate_nfts(nfts)?;
        // Verfies that the sender owns the nfts
        Self::ensure_nft_ownership(sender_portfolio, nfts)?;

        // Verfies that the receiver will not overflow
        NumberOfNFTs::get(nfts.asset_id(), receiver_portfolio.did)
            .checked_add(nfts_transferred)
            .ok_or(Error::<T>::InvalidNFTTransferCountOverflow)?;

        // Controllers are exempt from compliance and frozen rules.
        if is_controller_transfer {
            return Ok(());
        }

        // Verifies that the asset is not frozen
        ensure!(
            !Frozen::get(nfts.asset_id()),
            Error::<T>::InvalidNFTTransferFrozenAsset
        );

        // Verifies if the receiver has a valid CDD claim.
        ensure!(
            Identity::<T>::has_valid_cdd(receiver_portfolio.did),
            Error::<T>::InvalidNFTTransferInvalidReceiverCDD
        );

        // Verifies if the sender has a valid CDD claim.
        ensure!(
            Identity::<T>::has_valid_cdd(sender_portfolio.did),
            Error::<T>::InvalidNFTTransferInvalidSenderCDD
        );

        // Verifies that all compliance rules are being respected
        if !T::Compliance::is_compliant(
            nfts.asset_id(),
            sender_portfolio.did,
            receiver_portfolio.did,
            weight_meter.ok_or(Error::<T>::InvalidNFTTransferComplianceFailure)?,
        )? {
            return Err(Error::<T>::InvalidNFTTransferComplianceFailure.into());
        }

        Ok(())
    }

    /// Returns `Ok` if `sender_portfolio` has all nfts and they are not locked. Otherwise, returns an `Err`.
    fn ensure_nft_ownership(sender_portfolio: &PortfolioId, nfts: &NFTs) -> DispatchResult {
        // Verfies that the sender owns the nfts and that they are not locked
        for nft_id in nfts.ids() {
            ensure!(
                PortfolioNFT::contains_key(sender_portfolio, (nfts.asset_id(), nft_id)),
                Error::<T>::InvalidNFTTransferNFTNotOwned
            );
            ensure!(
                !PortfolioLockedNFT::contains_key(sender_portfolio, (nfts.asset_id(), nft_id)),
                Error::<T>::InvalidNFTTransferNFTIsLocked
            );
        }

        Ok(())
    }

    /// Verifies that the number of NFTs being transferred is greater than zero and less or equal to `MaxNumberOfNFTsPerLeg`.
    pub fn ensure_within_nfts_transfer_limits(nfts: &NFTs) -> DispatchResult {
        ensure!(nfts.len() > 0, Error::<T>::ZeroCount);
        ensure!(
            nfts.len() <= (T::MaxNumberOfNFTsCount::get() as usize),
            Error::<T>::MaxNumberOfNFTsPerLegExceeded
        );
        Ok(())
    }

    /// Verifies that there are no duplicate ids in the `NFTs` struct.
    pub fn ensure_no_duplicate_nfts(nfts: &NFTs) -> DispatchResult {
        let unique_nfts: BTreeSet<&NFTId> = nfts.ids().iter().collect();
        ensure!(unique_nfts.len() == nfts.len(), Error::<T>::DuplicatedNFTId);
        Ok(())
    }

    /// Updates the storage for transferring all `nfts` from `sender_portfolio` to `receiver_portfolio`.
    fn unverified_nfts_transfer(
        sender_portfolio: &PortfolioId,
        receiver_portfolio: &PortfolioId,
        nfts: &NFTs,
    ) {
        // Update the balance of the sender and the receiver
        let transferred_amount = nfts.len() as u64;
        NumberOfNFTs::mutate(nfts.asset_id(), sender_portfolio.did, |balance| {
            *balance -= transferred_amount
        });
        NumberOfNFTs::mutate(nfts.asset_id(), receiver_portfolio.did, |balance| {
            *balance += transferred_amount
        });
        // Update the portfolio of the sender and the receiver
        for nft_id in nfts.ids() {
            PortfolioNFT::remove(sender_portfolio, (nfts.asset_id(), nft_id));
            PortfolioNFT::insert(receiver_portfolio, (nfts.asset_id(), nft_id), true);
            NFTOwner::insert(nfts.asset_id(), nft_id, receiver_portfolio);
        }
    }

    pub fn base_controller_transfer(
        origin: T::RuntimeOrigin,
        nfts: NFTs,
        source_portfolio: PortfolioId,
        callers_portfolio_kind: PortfolioKind,
    ) -> DispatchResult {
        // Ensure origin is agent with custody and permissions for portfolio.
        let caller_portfolio = Asset::<T>::ensure_origin_asset_and_portfolio_permissions(
            origin,
            *nfts.asset_id(),
            callers_portfolio_kind,
            true,
        )?;

        // Verifies if all rules for transfering the NFTs are being respected
        Self::validate_nft_transfer(&source_portfolio, &caller_portfolio, &nfts, true, None)?;
        // Transfer ownership of the NFTs
        Self::unverified_nfts_transfer(&source_portfolio, &caller_portfolio, &nfts);

        Self::deposit_event(Event::NFTPortfolioUpdated(
            caller_portfolio.did,
            nfts,
            Some(source_portfolio),
            Some(caller_portfolio),
            PortfolioUpdateReason::ControllerTransfer,
        ));
        Ok(())
    }

    /// Returns a vector containing all errors for the transfer. An empty vec means there's no error.
    pub fn nft_transfer_report(
        sender_portfolio: &PortfolioId,
        receiver_portfolio: &PortfolioId,
        nfts: &NFTs,
        skip_locked_check: bool,
        weight_meter: &mut WeightMeter,
    ) -> Vec<DispatchError> {
        let mut nft_transfer_errors = Vec::new();

        // If the collection doesn't exist, there's no point in assessing anything else
        if !CollectionAsset::contains_key(nfts.asset_id()) {
            return vec![Error::<T>::InvalidNFTTransferCollectionNotFound.into()];
        }

        if Frozen::get(nfts.asset_id()) {
            nft_transfer_errors.push(Error::<T>::InvalidNFTTransferFrozenAsset.into());
        }

        if sender_portfolio.did == receiver_portfolio.did {
            nft_transfer_errors
                .push(Error::<T>::InvalidNFTTransferSenderIdMatchesReceiverId.into());
        }

        let nfts_transferred = nfts.len() as u64;
        if NumberOfNFTs::get(nfts.asset_id(), &sender_portfolio.did) < nfts_transferred {
            nft_transfer_errors.push(Error::<T>::InvalidNFTTransferInsufficientCount.into());
        }

        if let Err(e) = Self::ensure_within_nfts_transfer_limits(nfts) {
            nft_transfer_errors.push(e);
        }

        if let Err(e) = Self::ensure_no_duplicate_nfts(nfts) {
            nft_transfer_errors.push(e);
        }

        if skip_locked_check {
            for nft_id in nfts.ids() {
                if !PortfolioNFT::contains_key(sender_portfolio, (nfts.asset_id(), nft_id)) {
                    nft_transfer_errors.push(Error::<T>::InvalidNFTTransferNFTNotOwned.into());
                    break;
                }
            }
        } else {
            if let Err(e) = Self::ensure_nft_ownership(sender_portfolio, nfts) {
                nft_transfer_errors.push(e);
            }
        }

        if !Identity::<T>::has_valid_cdd(receiver_portfolio.did) {
            nft_transfer_errors.push(Error::<T>::InvalidNFTTransferInvalidReceiverCDD.into());
        }

        if !Identity::<T>::has_valid_cdd(sender_portfolio.did) {
            nft_transfer_errors.push(Error::<T>::InvalidNFTTransferInvalidSenderCDD.into());
        }

        if NumberOfNFTs::get(nfts.asset_id(), &receiver_portfolio.did)
            .checked_add(nfts_transferred)
            .is_none()
        {
            nft_transfer_errors.push(Error::<T>::InvalidNFTTransferCountOverflow.into());
        }

        match T::Compliance::is_compliant(
            nfts.asset_id(),
            sender_portfolio.did,
            receiver_portfolio.did,
            weight_meter,
        ) {
            Ok(is_compliant) => {
                if !is_compliant {
                    nft_transfer_errors
                        .push(Error::<T>::InvalidNFTTransferComplianceFailure.into());
                }
            }
            Err(e) => {
                nft_transfer_errors.push(e);
            }
        }

        nft_transfer_errors
    }

    /// Adds one to `CurrentCollectionId`.
    fn update_current_collection_id() -> Result<NFTCollectionId, DispatchError> {
        CurrentCollectionId::try_mutate(|current_collection_id| match current_collection_id {
            Some(current_id) => {
                let new_id = try_next_pre::<T, _>(current_id)?;
                *current_collection_id = Some(new_id);
                Ok::<NFTCollectionId, DispatchError>(new_id)
            }
            None => {
                let new_id = NFTCollectionId(1);
                *current_collection_id = Some(new_id);
                Ok::<NFTCollectionId, DispatchError>(new_id)
            }
        })
    }

    /// Adds one to the `NFTId` that belongs to `collection_id`.
    fn update_current_nft_id(collection_id: &NFTCollectionId) -> Result<NFTId, DispatchError> {
        CurrentNFTId::try_mutate(collection_id, |current_nft_id| match current_nft_id {
            Some(current_id) => {
                let new_nft_id = try_next_pre::<T, _>(current_id)?;
                *current_nft_id = Some(new_nft_id);
                Ok::<NFTId, DispatchError>(new_nft_id)
            }
            None => {
                let new_nft_id = NFTId(1);
                *current_nft_id = Some(new_nft_id);
                Ok::<NFTId, DispatchError>(new_nft_id)
            }
        })
    }
}

impl<T: Config> NFTTrait<T::RuntimeOrigin> for Module<T> {
    fn is_collection_key(asset_id: &AssetId, metadata_key: &AssetMetadataKey) -> bool {
        match CollectionAsset::try_get(asset_id) {
            Ok(collection_id) => {
                let key_set = CollectionKeys::get(&collection_id);
                key_set.contains(metadata_key)
            }
            Err(_) => false,
        }
    }

    fn move_portfolio_owner(asset_id: AssetId, nft_id: NFTId, new_owner_portfolio: PortfolioId) {
        NFTOwner::insert(asset_id, nft_id, new_owner_portfolio);
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn create_nft_collection(
        origin: T::RuntimeOrigin,
        asset_id: Option<AssetId>,
        nft_type: Option<NonFungibleType>,
        collection_keys: NFTCollectionKeys,
    ) -> DispatchResult {
        Module::<T>::create_nft_collection(origin, asset_id, nft_type, collection_keys)
    }
}
