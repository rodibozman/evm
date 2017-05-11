extern crate jsontests;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

use serde_json::Value;
use jsontests::test_transaction;

lazy_static! {
    static ref TESTS: Value =
        serde_json::from_str(include_str!("files/vmArithmeticTest.json")).unwrap();
}

#[test] fn add0() { assert_eq!(test_transaction("add0", &TESTS["add0"], true), true); }
#[test] fn add1() { assert_eq!(test_transaction("add1", &TESTS["add1"], true), true); }
#[test] fn add2() { assert_eq!(test_transaction("add2", &TESTS["add2"], true), true); }
#[test] fn add3() { assert_eq!(test_transaction("add3", &TESTS["add3"], true), true); }
#[test] fn add4() { assert_eq!(test_transaction("add4", &TESTS["add4"], true), true); }
#[test] fn addmod0() { assert_eq!(test_transaction("addmod0", &TESTS["addmod0"], true), true); }
#[test] fn addmod1() { assert_eq!(test_transaction("addmod1", &TESTS["addmod1"], true), true); }
#[test] fn addmod1_overflow2() { assert_eq!(test_transaction("addmod1_overflow2", &TESTS["addmod1_overflow2"], true), true); }
#[test] fn addmod1_overflow3() { assert_eq!(test_transaction("addmod1_overflow3", &TESTS["addmod1_overflow3"], true), true); }
#[test] fn addmod1_overflow4() { assert_eq!(test_transaction("addmod1_overflow4", &TESTS["addmod1_overflow4"], true), true); }
#[test] fn addmod1_overflowDiff() { assert_eq!(test_transaction("addmod1_overflowDiff", &TESTS["addmod1_overflowDiff"], true), true); }
#[test] fn addmod2() { assert_eq!(test_transaction("addmod2", &TESTS["addmod2"], true), true); }
#[test] fn addmod2_0() { assert_eq!(test_transaction("addmod2_0", &TESTS["addmod2_0"], true), true); }
#[test] fn addmod2_1() { assert_eq!(test_transaction("addmod2_1", &TESTS["addmod2_1"], true), true); }
#[test] fn addmod3() { assert_eq!(test_transaction("addmod3", &TESTS["addmod3"], true), true); }
#[test] fn addmod3_0() { assert_eq!(test_transaction("addmod3_0", &TESTS["addmod3_0"], true), true); }
#[test] fn addmodBigIntCast() { assert_eq!(test_transaction("addmodBigIntCast", &TESTS["addmodBigIntCast"], true), true); }
#[test] fn addmodDivByZero() { assert_eq!(test_transaction("addmodDivByZero", &TESTS["addmodDivByZero"], true), true); }
#[test] fn addmodDivByZero1() { assert_eq!(test_transaction("addmodDivByZero1", &TESTS["addmodDivByZero1"], true), true); }
#[test] fn addmodDivByZero2() { assert_eq!(test_transaction("addmodDivByZero2", &TESTS["addmodDivByZero2"], true), true); }
#[test] fn addmodDivByZero3() { assert_eq!(test_transaction("addmodDivByZero3", &TESTS["addmodDivByZero3"], true), true); }
#[test] fn arith1() { assert_eq!(test_transaction("arith1", &TESTS["arith1"], true), true); }
#[test] fn div1() { assert_eq!(test_transaction("div1", &TESTS["div1"], true), true); }
#[test] fn divBoostBug() { assert_eq!(test_transaction("divBoostBug", &TESTS["divBoostBug"], true), true); }
#[test] fn divByNonZero0() { assert_eq!(test_transaction("divByNonZero0", &TESTS["divByNonZero0"], true), true); }
#[test] fn divByNonZero1() { assert_eq!(test_transaction("divByNonZero1", &TESTS["divByNonZero1"], true), true); }
#[test] fn divByNonZero2() { assert_eq!(test_transaction("divByNonZero2", &TESTS["divByNonZero2"], true), true); }
#[test] fn divByNonZero3() { assert_eq!(test_transaction("divByNonZero3", &TESTS["divByNonZero3"], true), true); }
#[test] fn divByZero() { assert_eq!(test_transaction("divByZero", &TESTS["divByZero"], true), true); }
#[test] fn divByZero_2() { assert_eq!(test_transaction("divByZero_2", &TESTS["divByZero_2"], true), true); }
#[test] fn exp0() { assert_eq!(test_transaction("exp0", &TESTS["exp0"], true), true); }
#[test] fn exp1() { assert_eq!(test_transaction("exp1", &TESTS["exp1"], true), true); }
#[test] fn exp2() { assert_eq!(test_transaction("exp2", &TESTS["exp2"], true), true); }
#[test] fn exp3() { assert_eq!(test_transaction("exp3", &TESTS["exp3"], true), true); }
#[test] fn exp4() { assert_eq!(test_transaction("exp4", &TESTS["exp4"], true), true); }
#[test] fn exp5() { assert_eq!(test_transaction("exp5", &TESTS["exp5"], true), true); }
#[test] fn exp6() { assert_eq!(test_transaction("exp6", &TESTS["exp6"], true), true); }
#[test] fn exp7() { assert_eq!(test_transaction("exp7", &TESTS["exp7"], true), true); }
#[test] fn expPowerOf256Of256_0() { assert_eq!(test_transaction("expPowerOf256Of256_0", &TESTS["expPowerOf256Of256_0"], true), true); }
#[test] fn expPowerOf256Of256_1() { assert_eq!(test_transaction("expPowerOf256Of256_1", &TESTS["expPowerOf256Of256_1"], true), true); }
#[test] fn expPowerOf256Of256_10() { assert_eq!(test_transaction("expPowerOf256Of256_10", &TESTS["expPowerOf256Of256_10"], true), true); }
#[test] fn expPowerOf256Of256_11() { assert_eq!(test_transaction("expPowerOf256Of256_11", &TESTS["expPowerOf256Of256_11"], true), true); }
#[test] fn expPowerOf256Of256_12() { assert_eq!(test_transaction("expPowerOf256Of256_12", &TESTS["expPowerOf256Of256_12"], true), true); }
#[test] fn expPowerOf256Of256_13() { assert_eq!(test_transaction("expPowerOf256Of256_13", &TESTS["expPowerOf256Of256_13"], true), true); }
#[test] fn expPowerOf256Of256_14() { assert_eq!(test_transaction("expPowerOf256Of256_14", &TESTS["expPowerOf256Of256_14"], true), true); }
#[test] fn expPowerOf256Of256_15() { assert_eq!(test_transaction("expPowerOf256Of256_15", &TESTS["expPowerOf256Of256_15"], true), true); }
#[test] fn expPowerOf256Of256_16() { assert_eq!(test_transaction("expPowerOf256Of256_16", &TESTS["expPowerOf256Of256_16"], true), true); }
#[test] fn expPowerOf256Of256_17() { assert_eq!(test_transaction("expPowerOf256Of256_17", &TESTS["expPowerOf256Of256_17"], true), true); }
#[test] fn expPowerOf256Of256_18() { assert_eq!(test_transaction("expPowerOf256Of256_18", &TESTS["expPowerOf256Of256_18"], true), true); }
#[test] fn expPowerOf256Of256_19() { assert_eq!(test_transaction("expPowerOf256Of256_19", &TESTS["expPowerOf256Of256_19"], true), true); }
#[test] fn expPowerOf256Of256_2() { assert_eq!(test_transaction("expPowerOf256Of256_2", &TESTS["expPowerOf256Of256_2"], true), true); }
#[test] fn expPowerOf256Of256_20() { assert_eq!(test_transaction("expPowerOf256Of256_20", &TESTS["expPowerOf256Of256_20"], true), true); }
#[test] fn expPowerOf256Of256_21() { assert_eq!(test_transaction("expPowerOf256Of256_21", &TESTS["expPowerOf256Of256_21"], true), true); }
#[test] fn expPowerOf256Of256_22() { assert_eq!(test_transaction("expPowerOf256Of256_22", &TESTS["expPowerOf256Of256_22"], true), true); }
#[test] fn expPowerOf256Of256_23() { assert_eq!(test_transaction("expPowerOf256Of256_23", &TESTS["expPowerOf256Of256_23"], true), true); }
#[test] fn expPowerOf256Of256_24() { assert_eq!(test_transaction("expPowerOf256Of256_24", &TESTS["expPowerOf256Of256_24"], true), true); }
#[test] fn expPowerOf256Of256_25() { assert_eq!(test_transaction("expPowerOf256Of256_25", &TESTS["expPowerOf256Of256_25"], true), true); }
#[test] fn expPowerOf256Of256_26() { assert_eq!(test_transaction("expPowerOf256Of256_26", &TESTS["expPowerOf256Of256_26"], true), true); }
#[test] fn expPowerOf256Of256_27() { assert_eq!(test_transaction("expPowerOf256Of256_27", &TESTS["expPowerOf256Of256_27"], true), true); }
#[test] fn expPowerOf256Of256_28() { assert_eq!(test_transaction("expPowerOf256Of256_28", &TESTS["expPowerOf256Of256_28"], true), true); }
#[test] fn expPowerOf256Of256_29() { assert_eq!(test_transaction("expPowerOf256Of256_29", &TESTS["expPowerOf256Of256_29"], true), true); }
#[test] fn expPowerOf256Of256_3() { assert_eq!(test_transaction("expPowerOf256Of256_3", &TESTS["expPowerOf256Of256_3"], true), true); }
#[test] fn expPowerOf256Of256_30() { assert_eq!(test_transaction("expPowerOf256Of256_30", &TESTS["expPowerOf256Of256_30"], true), true); }
#[test] fn expPowerOf256Of256_31() { assert_eq!(test_transaction("expPowerOf256Of256_31", &TESTS["expPowerOf256Of256_31"], true), true); }
#[test] fn expPowerOf256Of256_32() { assert_eq!(test_transaction("expPowerOf256Of256_32", &TESTS["expPowerOf256Of256_32"], true), true); }
#[test] fn expPowerOf256Of256_33() { assert_eq!(test_transaction("expPowerOf256Of256_33", &TESTS["expPowerOf256Of256_33"], true), true); }
#[test] fn expPowerOf256Of256_4() { assert_eq!(test_transaction("expPowerOf256Of256_4", &TESTS["expPowerOf256Of256_4"], true), true); }
#[test] fn expPowerOf256Of256_5() { assert_eq!(test_transaction("expPowerOf256Of256_5", &TESTS["expPowerOf256Of256_5"], true), true); }
#[test] fn expPowerOf256Of256_6() { assert_eq!(test_transaction("expPowerOf256Of256_6", &TESTS["expPowerOf256Of256_6"], true), true); }
#[test] fn expPowerOf256Of256_7() { assert_eq!(test_transaction("expPowerOf256Of256_7", &TESTS["expPowerOf256Of256_7"], true), true); }
#[test] fn expPowerOf256Of256_8() { assert_eq!(test_transaction("expPowerOf256Of256_8", &TESTS["expPowerOf256Of256_8"], true), true); }
#[test] fn expPowerOf256Of256_9() { assert_eq!(test_transaction("expPowerOf256Of256_9", &TESTS["expPowerOf256Of256_9"], true), true); }
#[test] fn expPowerOf256_1() { assert_eq!(test_transaction("expPowerOf256_1", &TESTS["expPowerOf256_1"], true), true); }
#[test] fn expPowerOf256_10() { assert_eq!(test_transaction("expPowerOf256_10", &TESTS["expPowerOf256_10"], true), true); }
#[test] fn expPowerOf256_11() { assert_eq!(test_transaction("expPowerOf256_11", &TESTS["expPowerOf256_11"], true), true); }
#[test] fn expPowerOf256_12() { assert_eq!(test_transaction("expPowerOf256_12", &TESTS["expPowerOf256_12"], true), true); }
#[test] fn expPowerOf256_13() { assert_eq!(test_transaction("expPowerOf256_13", &TESTS["expPowerOf256_13"], true), true); }
#[test] fn expPowerOf256_14() { assert_eq!(test_transaction("expPowerOf256_14", &TESTS["expPowerOf256_14"], true), true); }
#[test] fn expPowerOf256_15() { assert_eq!(test_transaction("expPowerOf256_15", &TESTS["expPowerOf256_15"], true), true); }
#[test] fn expPowerOf256_16() { assert_eq!(test_transaction("expPowerOf256_16", &TESTS["expPowerOf256_16"], true), true); }
#[test] fn expPowerOf256_17() { assert_eq!(test_transaction("expPowerOf256_17", &TESTS["expPowerOf256_17"], true), true); }
#[test] fn expPowerOf256_18() { assert_eq!(test_transaction("expPowerOf256_18", &TESTS["expPowerOf256_18"], true), true); }
#[test] fn expPowerOf256_19() { assert_eq!(test_transaction("expPowerOf256_19", &TESTS["expPowerOf256_19"], true), true); }
#[test] fn expPowerOf256_2() { assert_eq!(test_transaction("expPowerOf256_2", &TESTS["expPowerOf256_2"], true), true); }
#[test] fn expPowerOf256_20() { assert_eq!(test_transaction("expPowerOf256_20", &TESTS["expPowerOf256_20"], true), true); }
#[test] fn expPowerOf256_21() { assert_eq!(test_transaction("expPowerOf256_21", &TESTS["expPowerOf256_21"], true), true); }
#[test] fn expPowerOf256_22() { assert_eq!(test_transaction("expPowerOf256_22", &TESTS["expPowerOf256_22"], true), true); }
#[test] fn expPowerOf256_23() { assert_eq!(test_transaction("expPowerOf256_23", &TESTS["expPowerOf256_23"], true), true); }
#[test] fn expPowerOf256_24() { assert_eq!(test_transaction("expPowerOf256_24", &TESTS["expPowerOf256_24"], true), true); }
#[test] fn expPowerOf256_25() { assert_eq!(test_transaction("expPowerOf256_25", &TESTS["expPowerOf256_25"], true), true); }
#[test] fn expPowerOf256_26() { assert_eq!(test_transaction("expPowerOf256_26", &TESTS["expPowerOf256_26"], true), true); }
#[test] fn expPowerOf256_27() { assert_eq!(test_transaction("expPowerOf256_27", &TESTS["expPowerOf256_27"], true), true); }
#[test] fn expPowerOf256_28() { assert_eq!(test_transaction("expPowerOf256_28", &TESTS["expPowerOf256_28"], true), true); }
#[test] fn expPowerOf256_29() { assert_eq!(test_transaction("expPowerOf256_29", &TESTS["expPowerOf256_29"], true), true); }
#[test] fn expPowerOf256_3() { assert_eq!(test_transaction("expPowerOf256_3", &TESTS["expPowerOf256_3"], true), true); }
#[test] fn expPowerOf256_30() { assert_eq!(test_transaction("expPowerOf256_30", &TESTS["expPowerOf256_30"], true), true); }
#[test] fn expPowerOf256_31() { assert_eq!(test_transaction("expPowerOf256_31", &TESTS["expPowerOf256_31"], true), true); }
#[test] fn expPowerOf256_32() { assert_eq!(test_transaction("expPowerOf256_32", &TESTS["expPowerOf256_32"], true), true); }
#[test] fn expPowerOf256_33() { assert_eq!(test_transaction("expPowerOf256_33", &TESTS["expPowerOf256_33"], true), true); }
#[test] fn expPowerOf256_4() { assert_eq!(test_transaction("expPowerOf256_4", &TESTS["expPowerOf256_4"], true), true); }
#[test] fn expPowerOf256_5() { assert_eq!(test_transaction("expPowerOf256_5", &TESTS["expPowerOf256_5"], true), true); }
#[test] fn expPowerOf256_6() { assert_eq!(test_transaction("expPowerOf256_6", &TESTS["expPowerOf256_6"], true), true); }
#[test] fn expPowerOf256_7() { assert_eq!(test_transaction("expPowerOf256_7", &TESTS["expPowerOf256_7"], true), true); }
#[test] fn expPowerOf256_8() { assert_eq!(test_transaction("expPowerOf256_8", &TESTS["expPowerOf256_8"], true), true); }
#[test] fn expPowerOf256_9() { assert_eq!(test_transaction("expPowerOf256_9", &TESTS["expPowerOf256_9"], true), true); }
#[test] fn expPowerOf2_128() { assert_eq!(test_transaction("expPowerOf2_128", &TESTS["expPowerOf2_128"], true), true); }
#[test] fn expPowerOf2_16() { assert_eq!(test_transaction("expPowerOf2_16", &TESTS["expPowerOf2_16"], true), true); }
#[test] fn expPowerOf2_2() { assert_eq!(test_transaction("expPowerOf2_2", &TESTS["expPowerOf2_2"], true), true); }
#[test] fn expPowerOf2_256() { assert_eq!(test_transaction("expPowerOf2_256", &TESTS["expPowerOf2_256"], true), true); }
#[test] fn expPowerOf2_32() { assert_eq!(test_transaction("expPowerOf2_32", &TESTS["expPowerOf2_32"], true), true); }
#[test] fn expPowerOf2_4() { assert_eq!(test_transaction("expPowerOf2_4", &TESTS["expPowerOf2_4"], true), true); }
#[test] fn expPowerOf2_64() { assert_eq!(test_transaction("expPowerOf2_64", &TESTS["expPowerOf2_64"], true), true); }
#[test] fn expPowerOf2_8() { assert_eq!(test_transaction("expPowerOf2_8", &TESTS["expPowerOf2_8"], true), true); }
#[test] fn expXY() { assert_eq!(test_transaction("expXY", &TESTS["expXY"], true), true); }
#[test] fn expXY_success() { assert_eq!(test_transaction("expXY_success", &TESTS["expXY_success"], true), true); }
#[test] fn fibbonacci_unrolled() { assert_eq!(test_transaction("fibbonacci_unrolled", &TESTS["fibbonacci_unrolled"], true), true); }
#[test] fn mod0() { assert_eq!(test_transaction("mod0", &TESTS["mod0"], true), true); }
#[test] fn mod1() { assert_eq!(test_transaction("mod1", &TESTS["mod1"], true), true); }
#[test] fn mod2() { assert_eq!(test_transaction("mod2", &TESTS["mod2"], true), true); }
#[test] fn mod3() { assert_eq!(test_transaction("mod3", &TESTS["mod3"], true), true); }
#[test] fn mod4() { assert_eq!(test_transaction("mod4", &TESTS["mod4"], true), true); }
#[test] fn modByZero() { assert_eq!(test_transaction("modByZero", &TESTS["modByZero"], true), true); }
#[test] fn mul0() { assert_eq!(test_transaction("mul0", &TESTS["mul0"], true), true); }
#[test] fn mul1() { assert_eq!(test_transaction("mul1", &TESTS["mul1"], true), true); }
#[test] fn mul2() { assert_eq!(test_transaction("mul2", &TESTS["mul2"], true), true); }
#[test] fn mul3() { assert_eq!(test_transaction("mul3", &TESTS["mul3"], true), true); }
#[test] fn mul4() { assert_eq!(test_transaction("mul4", &TESTS["mul4"], true), true); }
#[test] fn mul5() { assert_eq!(test_transaction("mul5", &TESTS["mul5"], true), true); }
#[test] fn mul6() { assert_eq!(test_transaction("mul6", &TESTS["mul6"], true), true); }
#[test] fn mul7() { assert_eq!(test_transaction("mul7", &TESTS["mul7"], true), true); }
#[test] fn mulUnderFlow() { assert_eq!(test_transaction("mulUnderFlow", &TESTS["mulUnderFlow"], true), true); }
#[test] fn mulmod0() { assert_eq!(test_transaction("mulmod0", &TESTS["mulmod0"], true), true); }
#[test] fn mulmod1() { assert_eq!(test_transaction("mulmod1", &TESTS["mulmod1"], true), true); }
#[test] fn mulmod1_overflow() { assert_eq!(test_transaction("mulmod1_overflow", &TESTS["mulmod1_overflow"], true), true); }
#[test] fn mulmod1_overflow2() { assert_eq!(test_transaction("mulmod1_overflow2", &TESTS["mulmod1_overflow2"], true), true); }
#[test] fn mulmod1_overflow3() { assert_eq!(test_transaction("mulmod1_overflow3", &TESTS["mulmod1_overflow3"], true), true); }
#[test] fn mulmod1_overflow4() { assert_eq!(test_transaction("mulmod1_overflow4", &TESTS["mulmod1_overflow4"], true), true); }
#[test] fn mulmod2() { assert_eq!(test_transaction("mulmod2", &TESTS["mulmod2"], true), true); }
#[test] fn mulmod2_0() { assert_eq!(test_transaction("mulmod2_0", &TESTS["mulmod2_0"], true), true); }
#[test] fn mulmod2_1() { assert_eq!(test_transaction("mulmod2_1", &TESTS["mulmod2_1"], true), true); }
#[test] fn mulmod3() { assert_eq!(test_transaction("mulmod3", &TESTS["mulmod3"], true), true); }
#[test] fn mulmod3_0() { assert_eq!(test_transaction("mulmod3_0", &TESTS["mulmod3_0"], true), true); }
#[test] fn mulmod4() { assert_eq!(test_transaction("mulmod4", &TESTS["mulmod4"], true), true); }
#[test] fn mulmoddivByZero() { assert_eq!(test_transaction("mulmoddivByZero", &TESTS["mulmoddivByZero"], true), true); }
#[test] fn mulmoddivByZero1() { assert_eq!(test_transaction("mulmoddivByZero1", &TESTS["mulmoddivByZero1"], true), true); }
#[test] fn mulmoddivByZero2() { assert_eq!(test_transaction("mulmoddivByZero2", &TESTS["mulmoddivByZero2"], true), true); }
#[test] fn mulmoddivByZero3() { assert_eq!(test_transaction("mulmoddivByZero3", &TESTS["mulmoddivByZero3"], true), true); }
#[test] fn not1() { assert_eq!(test_transaction("not1", &TESTS["not1"], true), true); }
#[test] fn sdiv0() { assert_eq!(test_transaction("sdiv0", &TESTS["sdiv0"], true), true); }
#[test] fn sdiv1() { assert_eq!(test_transaction("sdiv1", &TESTS["sdiv1"], true), true); }
#[test] fn sdiv2() { assert_eq!(test_transaction("sdiv2", &TESTS["sdiv2"], true), true); }
#[test] fn sdiv3() { assert_eq!(test_transaction("sdiv3", &TESTS["sdiv3"], true), true); }
#[test] fn sdiv4() { assert_eq!(test_transaction("sdiv4", &TESTS["sdiv4"], true), true); }
#[test] fn sdiv5() { assert_eq!(test_transaction("sdiv5", &TESTS["sdiv5"], true), true); }
#[test] fn sdiv6() { assert_eq!(test_transaction("sdiv6", &TESTS["sdiv6"], true), true); }
#[test] fn sdiv7() { assert_eq!(test_transaction("sdiv7", &TESTS["sdiv7"], true), true); }
#[test] fn sdiv8() { assert_eq!(test_transaction("sdiv8", &TESTS["sdiv8"], true), true); }
#[test] fn sdiv9() { assert_eq!(test_transaction("sdiv9", &TESTS["sdiv9"], true), true); }
#[test] fn sdivByZero0() { assert_eq!(test_transaction("sdivByZero0", &TESTS["sdivByZero0"], true), true); }
#[test] fn sdivByZero1() { assert_eq!(test_transaction("sdivByZero1", &TESTS["sdivByZero1"], true), true); }
#[test] fn sdivByZero2() { assert_eq!(test_transaction("sdivByZero2", &TESTS["sdivByZero2"], true), true); }
#[test] fn sdiv_dejavu() { assert_eq!(test_transaction("sdiv_dejavu", &TESTS["sdiv_dejavu"], true), true); }
#[test] fn sdiv_i256min() { assert_eq!(test_transaction("sdiv_i256min", &TESTS["sdiv_i256min"], true), true); }
#[test] fn sdiv_i256min2() { assert_eq!(test_transaction("sdiv_i256min2", &TESTS["sdiv_i256min2"], true), true); }
#[test] fn sdiv_i256min3() { assert_eq!(test_transaction("sdiv_i256min3", &TESTS["sdiv_i256min3"], true), true); }
#[test] fn signextendInvalidByteNumber() { assert_eq!(test_transaction("signextendInvalidByteNumber", &TESTS["signextendInvalidByteNumber"], true), true); }
#[test] fn signextend_00() { assert_eq!(test_transaction("signextend_00", &TESTS["signextend_00"], true), true); }
#[test] fn signextend_0_BigByte() { assert_eq!(test_transaction("signextend_0_BigByte", &TESTS["signextend_0_BigByte"], true), true); }
#[test] fn signextend_AlmostBiggestByte() { assert_eq!(test_transaction("signextend_AlmostBiggestByte", &TESTS["signextend_AlmostBiggestByte"], true), true); }
#[test] fn signextend_BigByteBigByte() { assert_eq!(test_transaction("signextend_BigByteBigByte", &TESTS["signextend_BigByteBigByte"], true), true); }
#[test] fn signextend_BigBytePlus1_2() { assert_eq!(test_transaction("signextend_BigBytePlus1_2", &TESTS["signextend_BigBytePlus1_2"], true), true); }
#[test] fn signextend_BigByte_0() { assert_eq!(test_transaction("signextend_BigByte_0", &TESTS["signextend_BigByte_0"], true), true); }
#[test] fn signextend_BitIsNotSet() { assert_eq!(test_transaction("signextend_BitIsNotSet", &TESTS["signextend_BitIsNotSet"], true), true); }
#[test] fn signextend_BitIsNotSetInHigherByte() { assert_eq!(test_transaction("signextend_BitIsNotSetInHigherByte", &TESTS["signextend_BitIsNotSetInHigherByte"], true), true); }
#[test] fn signextend_BitIsSetInHigherByte() { assert_eq!(test_transaction("signextend_BitIsSetInHigherByte", &TESTS["signextend_BitIsSetInHigherByte"], true), true); }
#[test] fn signextend_Overflow_dj42() { assert_eq!(test_transaction("signextend_Overflow_dj42", &TESTS["signextend_Overflow_dj42"], true), true); }
#[test] fn signextend_bigBytePlus1() { assert_eq!(test_transaction("signextend_bigBytePlus1", &TESTS["signextend_bigBytePlus1"], true), true); }
#[test] fn signextend_bitIsSet() { assert_eq!(test_transaction("signextend_bitIsSet", &TESTS["signextend_bitIsSet"], true), true); }
#[test] fn smod0() { assert_eq!(test_transaction("smod0", &TESTS["smod0"], true), true); }
#[test] fn smod1() { assert_eq!(test_transaction("smod1", &TESTS["smod1"], true), true); }
#[test] fn smod2() { assert_eq!(test_transaction("smod2", &TESTS["smod2"], true), true); }
#[test] fn smod3() { assert_eq!(test_transaction("smod3", &TESTS["smod3"], true), true); }
#[test] fn smod4() { assert_eq!(test_transaction("smod4", &TESTS["smod4"], true), true); }
#[test] fn smod5() { assert_eq!(test_transaction("smod5", &TESTS["smod5"], true), true); }
#[test] fn smod6() { assert_eq!(test_transaction("smod6", &TESTS["smod6"], true), true); }
#[test] fn smod7() { assert_eq!(test_transaction("smod7", &TESTS["smod7"], true), true); }
#[test] fn smod8_byZero() { assert_eq!(test_transaction("smod8_byZero", &TESTS["smod8_byZero"], true), true); }
#[test] fn smod_i256min1() { assert_eq!(test_transaction("smod_i256min1", &TESTS["smod_i256min1"], true), true); }
#[test] fn smod_i256min2() { assert_eq!(test_transaction("smod_i256min2", &TESTS["smod_i256min2"], true), true); }
#[test] fn stop() { assert_eq!(test_transaction("stop", &TESTS["stop"], true), true); }
#[test] fn sub0() { assert_eq!(test_transaction("sub0", &TESTS["sub0"], true), true); }
#[test] fn sub1() { assert_eq!(test_transaction("sub1", &TESTS["sub1"], true), true); }
#[test] fn sub2() { assert_eq!(test_transaction("sub2", &TESTS["sub2"], true), true); }
#[test] fn sub3() { assert_eq!(test_transaction("sub3", &TESTS["sub3"], true), true); }
#[test] fn sub4() { assert_eq!(test_transaction("sub4", &TESTS["sub4"], true), true); }