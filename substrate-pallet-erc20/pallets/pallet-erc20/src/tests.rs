use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

const ALICE:u64 = 1;
const BOB:u64 = 2;

#[test]
fn init_should_work() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(Erc20::init(Origin::signed(ALICE), 100));
        assert_eq!(Erc20::get_balance(ALICE),100);

        //Error::AlreadyInitialized
        assert_noop!(Erc20::init(Origin::signed(BOB), 100),Error::<Test>::AlreadyInitialized);
	});
}

#[test]
fn transfer_should_work() {
	new_test_ext().execute_with(|| {
		// Error::UnInitialized
		assert_noop!( Erc20::transfer(Origin::signed(ALICE), BOB,100),Error::<Test>::UnInitialized);
        
        // Error::InsufficientFunds
        assert_ok!(Erc20::init(Origin::signed(ALICE), 100));
		assert_noop!( Erc20::transfer(Origin::signed(ALICE), BOB,101),Error::<Test>::InsufficientFunds);

        //transfer successfully
        assert_ok!(Erc20::transfer(Origin::signed(ALICE),BOB,10));
        assert_eq!(Erc20::get_balance(ALICE),90);
        assert_eq!(Erc20::get_balance(BOB),10);
	});
}

#[test]
fn approve_should_work() {
	new_test_ext().execute_with(|| {
		// Error::UnInitialized
		assert_noop!( Erc20::approve(Origin::signed(ALICE), BOB,30),Error::<Test>::UnInitialized);
        
        // Error::InsufficientFunds
        assert_ok!(Erc20::init(Origin::signed(ALICE), 100));
		assert_noop!( Erc20::approve(Origin::signed(ALICE), BOB,101),Error::<Test>::InsufficientFunds);

        //approve successfully
        assert_ok!(Erc20::approve(Origin::signed(ALICE),BOB,10));
        assert_eq!(Erc20::get_balance(ALICE),100);
        assert_eq!(Erc20::get_allowed_info(ALICE),(BOB,10));
	});
}

#[test]
fn transfer_from_should_work() {
	new_test_ext().execute_with(|| {
		// Error::UnInitialized
		assert_noop!( Erc20::transfer_from(Origin::signed(BOB),ALICE, BOB,30),Error::<Test>::UnInitialized);
        
        // Error::Unauthorized
        assert_ok!(Erc20::init(Origin::signed(ALICE), 100));
		assert_noop!(Erc20::transfer_from(Origin::signed(BOB),ALICE ,BOB,10),Error::<Test>::Unauthorized);

        // Error::InsufficientFunds
        assert_ok!(Erc20::approve(Origin::signed(ALICE),BOB,30));
		assert_noop!(Erc20::transfer_from(Origin::signed(BOB),ALICE ,BOB,40),Error::<Test>::InsufficientFunds);
        
        //transfer_from successfully
		assert_ok!(Erc20::transfer_from(Origin::signed(BOB),ALICE ,BOB,20));
        assert_eq!(Erc20::get_balance(ALICE),80);
        assert_eq!(Erc20::get_balance(BOB),20);
        assert_eq!(Erc20::get_allowed_info(ALICE),(BOB,10));
	});
}

