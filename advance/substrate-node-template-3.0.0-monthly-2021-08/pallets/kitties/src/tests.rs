use crate::{Error, mock::*, KittiesCount};
use frame_support::{assert_ok, assert_noop};
use super::*;
use frame_system as system;

//https://gitee.com/vicowong/node-template-kitties


pub const KITTY_RESERVE: u128 = 1_000;
pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const NOBODY: u64 = 99;


#[test]
fn create_kitty_ok() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		let r=KittiesCount::<Test>::try_get();
		println!("{}",r.unwrap());
		assert_eq!(r,
			Ok(2)
		);
	});
}



/*#[test]
fn can_create_work() {
	new_test_ext().execute_with(|| {
		//创建Kittiy
		assert_ok!(KittiesModule::create(Origin::signed(ALICE)));
		//检查事件
		/*System::assert_last_event(mock::Event::KittiesModule(crate::Event::KittyCreate(
			ALICE, 0,
		)));*/
		//检查总数量
		assert_eq!(KittiesCount::<Test>::get(), 1);
		//检查拥有者
		assert_eq!(Owner::<Test>::get(0), Some(ALICE));
		//检查质押数量
		//assert_eq!(Balances::reserved_balance(ALICE), KITTY_RESERVE);
	});
}*/












