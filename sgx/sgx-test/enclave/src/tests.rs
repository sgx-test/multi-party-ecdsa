use multi_party_ecdsa::*;

use multi_party_ecdsa::protocols::multi_party_ecdsa::gg_2020::orchestrate::*;
use multi_party_ecdsa::curv::elliptic::curves::secp256_k1::{FE,GE};

use multi_party_ecdsa::protocols::multi_party_ecdsa::gg_2020::party_i::{
    KeyGenBroadcastMessage1, KeyGenDecommitMessage1, Keys, LocalSignature, Parameters, SharedKeys,
    SignBroadcastPhase1, SignDecommitPhase1, SignKeys, SignatureRecid,
};

use multi_party_ecdsa::curv::elliptic::curves::traits::ECScalar;
use multi_party_ecdsa::curv::elliptic::curves::traits::ECPoint;

use std::prelude::v1::*;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

pub fn keys(){
    println!("start A");
    let u = FE::new_random();
    println!("start B");
    let g = GE::generator();
    println!("start C");
    let y = g * u;
    println!("start D");
    //let party_keys = Keys::create(0usize);
    //println!("party_keys = {:?}",party_keys);
}


pub fn key_gen_stage_1() {
    println!("1");
    let stage1_input = KeyGenStage1Input{
        index : 0usize,
    };


    let res = keygen_stage1(&stage1_input);
}
