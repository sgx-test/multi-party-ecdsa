use multi_party_ecdsa::*;

use multi_party_ecdsa::protocols::multi_party_ecdsa::gg_2020::orchestrate::*;

use std::prelude::v1::*;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

//#[test]
pub fn key_gen_stage_1() {
     let stage1_input = KeyGenStage1Input{
         index : 0usize,
     };
     let res = keygen_stage1(&stage1_input);
}

