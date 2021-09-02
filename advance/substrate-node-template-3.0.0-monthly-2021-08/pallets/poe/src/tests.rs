use crate::{Error, mock::*, Proofs};
use frame_support::{assert_ok, assert_noop};
use super::*;
use frame_system as system;

/*
对于单元测试，是由开发人员完成的最低级别测试，直接影响软件后期测试及产品质量。通常我们有两种方法进行单测设计，以确保达到足够的测试强度和准确度：
基于控制流覆盖准则：
① 语句覆盖(Statement Coverage，SC)；
② 分支覆盖(Branch Coverage，BC)；
③ 修正判定条件覆盖（ModifiedCondition/Decision Coverage，MC/DC）。
基于数据流覆盖准则：
① 定义覆盖（all-defs coverage,ADC）；
② 引用覆盖（all-use coverage，AUC）；
③ 定义引用路径覆盖（all-du-paths coverage，ADUPC）。

在默认情况下，采用分支覆盖100%来满足单测要求，针对第1课create_claim、revoke_claim和transfer_claim函数，
每个函数均有两个判断语句，正常语句覆盖会包含ensure!的true情况，则只需再设计ensure!的false情况即可，故单测用例数量为9个。
除此之外，通常单测设计思路有：正常、异常、边界值、等价类和状态变换等。
*/

#[test]
fn create_claim_ok() {
	new_test_ext().execute_with(|| {
		let claim =vec![0,1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim),
			Some((1,frame_system::Pallet::<Test>::block_number())
			)
		);
	});
}


#[test]
fn create_claim_ok_2() {
	new_test_ext().execute_with(|| {
		let claim =vec![0,1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone()));
		let rest=Proofs::<Test>::try_get(&claim);
		//println!("{}",rest.unwrap());
		assert_eq!(rest.unwrap(),
				   (1,frame_system::Pallet::<Test>::block_number())
		);
	});
}


#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim =vec![0,1];
		let _=PoeModule::create_claim(Origin::signed(1),claim.clone());
		assert_noop!(
		PoeModule::create_claim(Origin::signed(1),claim.clone()),
				   Error::<Test>::ProofAlreadyExist
		);
	});
}



#[test]
fn revoke_claim_ok() {
	new_test_ext().execute_with(|| {
		let claim =vec![0,1];
		let _=PoeModule::create_claim(Origin::signed(1),claim.clone());
		assert_ok!(PoeModule::revoke_claim(Origin::signed(1),claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim),
				   None
		);
	});
}

#[test]
fn revoke_claim_failed_when_no_claim_exist() {
	new_test_ext().execute_with(|| {
		let claim =vec![0,1];
		assert_noop!(
		PoeModule::revoke_claim(Origin::signed(1),claim.clone()),
				   Error::<Test>::ProofNotExist
		);
	});
}


#[test]
fn transfer_claim_failed_when_no_claim_exist() {
	new_test_ext().execute_with(|| {
		let claim =vec![0,1];
		assert_noop!(
		PoeModule::transfer_claim(Origin::signed(1),2,claim.clone()),
				   Error::<Test>::ProofNotExist
		);
	});
}


#[test]
fn transfer_claim_to_b() {
	new_test_ext().execute_with(|| {
		let claim =vec![0,1];
		let user_b= 2 as u64;
		let _=PoeModule::create_claim(Origin::signed(1),claim.clone());

		assert_ok!(PoeModule::transfer_claim(Origin::signed(1),user_b,claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim),
				   Some((2,frame_system::Pallet::<Test>::block_number())
				   )
		);
	});
}

#[test]
fn transfer_claim_to_b_failed_not_owner() {
	new_test_ext().execute_with(|| {
		let claim =vec![0,1];
		let user_b: <Test as system::Config>::AccountId = 2 ; //使用 AccountId，不适用 u64
		//let user_b: u64 = 2 ;
		let _=PoeModule::create_claim(Origin::signed(1),claim.clone());
		assert_noop!(
		PoeModule::transfer_claim(Origin::signed(3),user_b,claim.clone()),
				   Error::<Test>::NotClaimOwner
		);

	});
}

#[test]
fn crete_claim_failed_when_too_large_claim() {
	new_test_ext().execute_with(|| {

		//let claim =vec![0,1,3,4,5,6,7,8,9,10,11];
		let claim =vec![0; (MaxClaimLength::get() + 1) as usize]; //自动比最大长度大1
		assert_noop!(
		PoeModule::create_claim(Origin::signed(1),claim.clone()),
				   Error::<Test>::ClaimTooLarge
		);
	});
}









