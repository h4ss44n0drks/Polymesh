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

use crate::*;

use frame_benchmarking::{account, benchmarks};
use frame_system::RawOrigin;
use sp_core::H512;
use sp_std::prelude::*;

use polymesh_common_utilities::benchs::{
    cdd_provider, user, user_without_did, AccountIdOf, UserBuilder,
};
use polymesh_common_utilities::traits::{identity::TargetIdAuthorization, TestUtilsFn};
use polymesh_primitives::asset::AssetId;
use polymesh_primitives::identity::limits::{
    MAX_ASSETS, MAX_EXTRINSICS, MAX_PALLETS, MAX_PORTFOLIOS, MAX_SECONDARY_KEYS,
};
use polymesh_primitives::secondary_key::ExtrinsicNames;
use polymesh_primitives::{
    AssetPermissions, AuthorizationData, Claim, CountryCode, ExtrinsicName, ExtrinsicPermissions,
    PalletName, PalletPermissions, Permissions, PortfolioId, PortfolioNumber, PortfolioPermissions,
    Scope, SecondaryKey, Signatory,
};

const SEED: u32 = 0;

pub fn generate_secondary_keys<T: Config>(n: usize) -> Vec<SecondaryKey<T::AccountId>> {
    let mut secondary_keys = Vec::with_capacity(n);
    for x in 0..n {
        secondary_keys.push(SecondaryKey {
            key: account("key", x as u32, SEED),
            permissions: Default::default(),
        });
    }
    secondary_keys
}

benchmarks! {
    where_clause { where T: TestUtilsFn<AccountIdOf<T>> }

    create_child_identity {
        // Create parent identity.
        let parent = user::<T>("parent", 0);
        let parent_did = parent.did.unwrap();

        let child_key: T::AccountId = account("child", 0, SEED);
        Module::<T>::unsafe_join_identity(parent_did, Permissions::default(), child_key.clone());

    }: _(parent.origin, child_key.clone())
    verify {
        let child_did = Module::<T>::get_identity(&child_key).unwrap();
        assert_ne!(child_did, parent_did);
    }

    create_child_identities {
        // Number of keys.
        let i in 0 .. 100;

        // Create parent identity.
        let parent = user::<T>("parent", 0);
        let parent_did = parent.did.unwrap();

        let expires_at: T::Moment = 600u32.into();
        let authorization = TargetIdAuthorization::<T::Moment> {
            target_id: parent_did,
            nonce: Module::<T>::offchain_authorization_nonce(parent_did),
            expires_at,
        };
        let auth_encoded = authorization.encode();

        let child_keys_with_auth = (0..i).map(|x| {
            let user = user_without_did::<T>("key", x);
            CreateChildIdentityWithAuth {
                key: user.account(),
                auth_signature: H512::from(user.sign(&auth_encoded).unwrap()),
            }
        }).collect::<Vec<_>>();
    }: _(parent.origin, child_keys_with_auth.clone(), expires_at)
    verify {
        for auth in child_keys_with_auth {
            let child_did = Module::<T>::get_identity(&auth.key).unwrap();
            assert_ne!(child_did, parent_did);
        }
    }

    unlink_child_identity {
        // Create parent identity.
        let parent = user::<T>("parent", 0);
        let parent_did = parent.did.unwrap();

        // Create a secondary key.
        let child_key: T::AccountId = account("child", 0, SEED);
        Module::<T>::unsafe_join_identity(parent_did, Permissions::default(), child_key.clone());

        // Create a child identity using the secondary key.
        Module::<T>::create_child_identity(
            parent.origin().into(),
            child_key.clone()
        ).unwrap();
        let child_did = Module::<T>::get_identity(&child_key).unwrap();

        // Generate valid CDD claim for child identity.
        let cdd_claim = Claim::CustomerDueDiligence(CddId::default());

        // Add CDD claim to the child identity.
        let cdd = cdd_provider::<T>("cdd", 0).did.unwrap();
        Module::<T>::unverified_add_claim_with_scope(child_did, cdd_claim, None, cdd, None);

    }: _(parent.origin, child_did)
    verify {
        assert!(Module::<T>::has_valid_cdd(child_did));
    }

    cdd_register_did {
        // Number of secondary items.
        let i in 0 .. MAX_SECONDARY_KEYS;

        let cdd = cdd_provider::<T>("cdd", 0);
        let target: T::AccountId = account("target", SEED, SEED);
        let secondary_keys = generate_secondary_keys::<T>(i as usize);
    }: _(cdd.origin, target, secondary_keys)

    invalidate_cdd_claims {
        // NB: This function loops over all cdd claims issued by the cdd provider.
        // Therefore, it's unbounded in complexity. However, this can only be called by governance.
        // Hence, the weight is for best case scenario

        let cdd = cdd_provider::<T>("cdd", 0);

    }: _(RawOrigin::Root, cdd.did(), 0u32.into(), None)

    remove_secondary_keys {
        // Number of secondary items.
        let i in 0 .. MAX_SECONDARY_KEYS;

        let target = user::<T>("target", 0);

        let mut signatories = Vec::with_capacity(i as usize);
        for x in 0..i {
            let key: T::AccountId = account("key", x, SEED);
            signatories.push(key.clone());
            Module::<T>::unsafe_join_identity(target.did(), Permissions::default(), key);
        }
    }: _(target.origin, signatories.clone())

    accept_primary_key {
        let cdd = cdd_provider::<T>("cdd", 0);
        let target = user::<T>("target", 0);
        let new_key = UserBuilder::<T>::default().build("key");
        let signatory = Signatory::Account(new_key.account());

        let cdd_auth_id =  Module::<T>::add_auth(
            cdd.did(), signatory.clone(),
            AuthorizationData::AttestPrimaryKeyRotation(target.did()),
            None,
        )
        .unwrap();
        Module::<T>::change_cdd_requirement_for_mk_rotation(
            RawOrigin::Root.into(),
            true
        ).unwrap();

        let owner_auth_id =  Module::<T>::add_auth(
            target.did(), signatory,
            AuthorizationData::RotatePrimaryKey,
            None,
        ).
        unwrap();
    }: _(new_key.origin, owner_auth_id, Some(cdd_auth_id))

    rotate_primary_key_to_secondary {
        let cdd = cdd_provider::<T>("cdd", 0);
        let target = user::<T>("target", 0);
        let new_key = UserBuilder::<T>::default().build("key");
        let signatory = Signatory::Account(new_key.account());

        let cdd_auth_id =  Module::<T>::add_auth(
            cdd.did(), signatory.clone(),
            AuthorizationData::AttestPrimaryKeyRotation(target.did()),
            None,
        )
        .unwrap();
        let rotate_auth_id =  Module::<T>::add_auth(
            target.did(), signatory.clone(),
            AuthorizationData::RotatePrimaryKeyToSecondary(Permissions::default()),
            None,
        )
        .unwrap();
        Module::<T>::change_cdd_requirement_for_mk_rotation(
            RawOrigin::Root.into(),
            true
        ).unwrap();

    }: _(new_key.origin, rotate_auth_id, Some(cdd_auth_id))

    change_cdd_requirement_for_mk_rotation {
        assert!(
            !Module::<T>::cdd_auth_for_primary_key_rotation(),
            "CDD auth for primary key rotation is enabled"
        );
    }: _(RawOrigin::Root, true)
    verify {
        assert!(
            Module::<T>::cdd_auth_for_primary_key_rotation(),
            "CDD auth for primary key rotation did not change"
        );
    }

    join_identity_as_key {
        let target = user::<T>("target", 0);
        let new_key = UserBuilder::<T>::default().build("key");

        let auth_id =  Module::<T>::add_auth(
            target.did(),
            Signatory::Account(new_key.account()),
            AuthorizationData::JoinIdentity(Permissions::default()),
            None,
        )
        .unwrap();
    }: _(new_key.origin, auth_id)

    leave_identity_as_key {
        let target = user::<T>("target", 0);
        let key = UserBuilder::<T>::default().build("key");
        let signatory = Signatory::Account(key.account());

        let auth_id =  Module::<T>::add_auth(
            target.did(),
            signatory,
            AuthorizationData::JoinIdentity(Permissions::default()),
            None,
        )
        .unwrap();
        Module::<T>::join_identity_as_key(key.origin().into(), auth_id)
            .expect("Key cannot be joined to identity");

    }: _(key.origin())
    verify {
        assert!(
            !KeyRecords::<T>::contains_key(key.account),
            "Key was not removed from its identity"
        );
    }

    add_claim {
        let caller = user::<T>("caller", 0);
        let target = user::<T>("target", 0);
        let scope = Scope::Identity(caller.did());
        let claim = Claim::Jurisdiction(CountryCode::BB, scope);
    }: _(caller.origin, target.did(), claim, Some(666u32.into()))

    revoke_claim {
        let caller = user::<T>("caller", 0);
        let scope = Scope::Identity(caller.did());
        let claim = Claim::Jurisdiction(CountryCode::BB, scope);
        Module::<T>::add_claim(caller.origin.clone().into(), caller.did(), claim.clone(), Some(666u32.into())).unwrap();
    }: _(caller.origin, caller.did(), claim)

    revoke_claim_by_index {
        let caller = user::<T>("caller", 0);
        let scope = Scope::Identity(caller.did());
        let claim = Claim::Jurisdiction(CountryCode::BB, scope.clone());
        let claim_type = claim.claim_type();
        Module::<T>::add_claim(caller.origin.clone().into(), caller.did(), claim.clone(), Some(666u32.into())).unwrap();
    }: _(caller.origin, caller.did(), claim_type, Some(scope))

    set_secondary_key_permissions {
        let target = user::<T>("target", 0);
        let key = UserBuilder::<T>::default().build("key");
        let account_id = key.account();

        Module::<T>::unsafe_join_identity(target.did(), Permissions::empty(), account_id.clone());
    }: _(target.origin, account_id, Permissions::default().into())

    // Benchmark the memory/cpu complexity of Permissions.
    permissions_cost {
        // Number of assets/portfolios/pallets/extrinsics.
        let a in 0 .. MAX_ASSETS as u32; // a=(A)ssets
        let p in 0 .. MAX_PORTFOLIOS as u32; // p=(P)ortfolios
        let l in 0 .. MAX_PALLETS as u32; // l=pa(L)lets
        let e in 0 .. MAX_EXTRINSICS as u32; // e=(E)xtrinsics
        // When the benchmarks run for parameter `e` (number of extrinsics)
        // it will use `l == MAX_PALLETS`.  `e` will be the number of
        // extrinsics per pallet.  So the total number of extrinsics in
        // the `Permissions` will be `MAX_PALLETS * e`.
        //
        // When calculating the weight of a `Permissions` value in a
        // transaction, we use the total number of extrinsics in the
        // permissions.  This is to make sure that the worst-case cost
        // is covered.

        let asset = AssetPermissions::elems(
            (0..a as u64).map(|a| AssetId::new([a as u8; 16]))
        );
        let portfolio = PortfolioPermissions::elems(
            (0..p as u128).map(|did| {
                PortfolioId::user_portfolio(did.into(), PortfolioNumber(0))
            })
        );
        let extrinsics = ExtrinsicNames::elems(
            (0..e as u64).map(|e| {
                ExtrinsicName::generate(e)
            })
        );
        let extrinsic = ExtrinsicPermissions::these(
            (0..l as u64).map(|p| {
                (PalletName::generate(p), PalletPermissions {
                    extrinsics: extrinsics.clone(),
                })
            })
        );

        let permissions = Permissions {
            asset,
            extrinsic,
            portfolio
        };
    }: {
        // For this benchmark we need to do some "work" based on
        // how complex the permissions object is.

        // 1. Encode the Permissions value.
        let encoded = permissions.encode();
        // 2. Decode the Permissions value.
        let decoded = Permissions::decode(&mut encoded.as_slice())
            .expect("This shouldn't fail since we just encoded a Permissions value.");
        // 3. Compare the original and decoded values.  This will touch the full value.
        if !permissions.eq(&decoded) {
            panic!("This shouldn't fail.");
        }
    }

    freeze_secondary_keys {
        let caller = user::<T>("caller", 0);
    }: _(caller.origin)

    unfreeze_secondary_keys {
        let caller = user::<T>("caller", 0);
        Module::<T>::freeze_secondary_keys(caller.origin.clone().into()).unwrap();
    }: _(caller.origin)

    add_authorization {
        let caller = user::<T>("caller", 0);
        let signatory = Signatory::Identity(caller.did());
        let auth_data = AuthorizationData::JoinIdentity(Permissions::default());
    }: _(caller.origin, signatory, auth_data, Some(666u32.into()))

    remove_authorization {
        let caller = user::<T>("caller", 0);
        let signatory = Signatory::Identity(caller.did());
        let auth_id =  Module::<T>::add_auth(
            caller.did(),
            signatory.clone(),
            AuthorizationData::JoinIdentity(Permissions::default()),
            Some(666u32.into()),
        )
        .unwrap();
    }: _(caller.origin, signatory, auth_id, true)

    add_secondary_keys_with_authorization {
        // Number of keys.
        let i in 0 .. MAX_SECONDARY_KEYS;

        let caller = user::<T>("caller", SEED);

        let expires_at: T::Moment = 600u32.into();
        let authorization = TargetIdAuthorization::<T::Moment> {
            target_id: caller.did(),
            nonce: Module::<T>::offchain_authorization_nonce(caller.did()),
            expires_at,
        };
        let auth_encoded = authorization.encode();

        let secondary_keys_with_auth = (0..i).map(|x| {
            let user = user_without_did::<T>("key", x);
            SecondaryKeyWithAuth {
                secondary_key: SecondaryKey::from_account_id(user.account()).into(),
                auth_signature: H512::from(user.sign(&auth_encoded).unwrap()),
            }
        }).collect::<Vec<_>>();
    }: _(caller.origin, secondary_keys_with_auth, expires_at)

    register_custom_claim_type {
        let n in 1 .. T::MaxLen::get() as u32;

        let id = Module::<T>::custom_claim_id_seq();
        let caller = user::<T>("caller", 0);
        let ty = vec![b'X'; n as usize];
    }: _(caller.origin, ty)
    verify {
        assert_ne!(id, Module::<T>::custom_claim_id_seq());
    }

}
