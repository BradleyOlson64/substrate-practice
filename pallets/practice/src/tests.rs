use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;

#[test]
fn leah_test() {
	new_test_ext().execute_with(|| {
		let origin = RawOrigin::Signed(1u64);
		let origin2 = RawOrigin::Signed(2u64);
		let origin3 = RawOrigin::Signed(3u64);
		//assert_ok!(IdentityPallet::try_add_as_social_graph_originator(origin.clone()));
		//assert_noop!(IdentityPallet::try_add_as_social_graph_originator(origin.clone()), Error::<Test>::AlreadyInSet);
		run_to_block(10);
		//assert_ok!(IdentityPallet::try_add_as_social_graph_originator(origin2.clone()));
		//assert_noop!(IdentityPallet::try_add_as_social_graph_originator(origin3.clone()), Error::<Test>::NoNeedForAdditionalOriginators);
	});
}