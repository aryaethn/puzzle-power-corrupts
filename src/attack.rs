use ark_bls12_cheon::{Bls12Cheon, G1Projective as G1, G2Projective as G2, Fr};
use crate::utils::{pow_sp, pow_sp2, bigInt_to_u128};
use ark_ec::pairing::Pairing;
use ark_ff::{Field, PrimeField};
use std::time::Instant;
use std::ops::Mul;

pub fn attack(P: G1, tau_P: G1, tau_d1_P: G1, Q: G2, tau_d2_Q: G2) -> i128 {
    // Step 1: Define g = e(P, Q), h = e(tau*P, Q), k = e((tau^{d1})P, (tau^{d2})Q)
    let g = Bls12Cheon::pairing(P, Q);
    let h = Bls12Cheon::pairing(tau_P, Q);
    let k = Bls12Cheon::pairing(tau_d1_P, tau_d2_Q);

    let d1: u128 = 11726539;
    let d2: u128 = 690320833;
    let d = d1 + d2;

    let q: u128 = 1114157594638178892192613;
    let m: u128 = (q - 1) / d;
    let A: u128 = 1089478584172543;
    let B: u128 = 1089547303649280;
    let k0_base = A + 1;

    let two_pow_d = pow_sp(Fr::from(2u64), d as u128, 64);
    

    println!("=== Cheon Attack Parameters ===");
    println!("d = d1 + d2 = {}", d);
    println!("q (subgroup order) = {}", q);
    println!("m = |H1| = (q-1)/d = {}", m);
    println!("Computed 2^d (generator of H1) {}", two_pow_d);
    println!();
    println!("=== Cheon Attack Start ===");
    // Step 2: Compute g^(gamma^c) = k using BSGS
    // We know that if tau = 2 ^ (k0 + k1 * (q-1)/d) then k0 is the c we are looking for above
    // Also k0 is bounded by A and B
    // So we can compute g^(gamma^c) for c in [A, B] and see if we find k
    // But, [A, B] is too large to compute, so we need to use a BSGS algorithm with the noted bound.
    
    // First, shift the search: we're looking for c in [A, B], so let c' = c - A, c' in [0, B-A]
    // Then g^(gamma^c) = k becomes g^(gamma^(c'+A)) = k
    // Which means g^(gamma^c') = k / g^(gamma^A)
    println!("=== Cheon Attack: BSGS to find k0 ===");
    let mut k0 = 0u128;
    let step = (1<<20) as u64;
    println!("Baby step for k0...");
    let start = Instant::now();
    let table = pow_sp2(g.0, two_pow_d, step);
    let end = Instant::now();
    println!("Baby step for k0 took {:?}", end.duration_since(start));

    let mut giant = k.0.pow(pow_sp(two_pow_d.inverse().unwrap(), k0_base, 51).into_bigint());
    let giant_inc = pow_sp(two_pow_d.inverse().unwrap(), step.into(), 64).into_bigint();

    println!("Giant step for k0...");
    for i in 0..m / u128::from(step) {
        if let Some(j) = table.get(&giant) {
            let end = Instant::now();
            println!("Giant step for k0 took {:?}", end.duration_since(start));
            k0 = k0_base + (i * u128::from(step)) + u128::from(*j);
            println!("Found i = {}, j = {}, k0 = {}", i, j, k0);
            break;
        }
        giant = giant.pow(giant_inc);
    }

    // Step 3: Compute tau = 2^(k0 + k1 * (q-1)/d) = 2^k0 * eta ^ k1 to find k1
    // use the exact same method (i.e., BSGS) to find k1, with two 2^15-bit parts.
    println!("=== Cheon Attack: BSGS to find k1 ===");
    let mut k1 = 0u128;
    let step = (1<<16) as u64;
    let eta = pow_sp(Fr::from(2u64), m, 64);
    
    println!("Baby step for k1...");
    let start = Instant::now();
    let table = pow_sp2(g.0, eta, step);
    let end = Instant::now();
    println!("Baby step for k1 took {:?}", end.duration_since(start));

    let mut giant = h.0.pow(pow_sp(Fr::from(2u64).inverse().unwrap(), k0, 51).into_bigint());
    let giant_inc = pow_sp(eta.inverse().unwrap(), step.into(), 64).into_bigint();
    println!("Giant step for k1...");
    let start = Instant::now();
    for i in 0..d / u128::from(step) {
        if let Some(j) = table.get(&giant) {
            let end = Instant::now();
            println!("Giant step for k1 took {:?}", end.duration_since(start));
            k1 = u128::from(i) * u128::from(step) + u128::from(*j);
            println!("Found i = {}, j = {}, k1 = {}", i, j, k1);
            break;
        }
        giant = giant.pow(giant_inc);
    }

    println!("=== Cheon Attack: Compute tau ===");

    let tau = pow_sp(Fr::from(2u64), k0 + k1 * m, 80);
    println!("Found tau = {}", tau);
    println!();

    println!("=== Cheon Attack: Verify tau ===");
    assert_eq!(P.mul(tau), tau_P);
    println!("Cheon Attack: Verification successful: P * tau = tau_P ✅");

    return bigInt_to_u128(tau.into_bigint()) as i128;
}
