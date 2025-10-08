# Power Corrupts Puzzle - Summary

## Overview
This is a cryptographic attack puzzle based on the **Cheon Attack** against a trusted setup for a pairing-friendly elliptic curve.

## The Scenario
- **Bob** created a new pairing-friendly elliptic curve for use with Groth16
- **Bob** performed a trusted setup, generating an SRS (Structured Reference String) containing a secret value τ raised to various powers
- **Alice** wants to recover the secret τ using the Cheon attack

## Curve Parameters
- **p** (field size): 5739212180072054691886037078074598258234828766015465682035977006377650233269727206694786492328072119776769214299497
- **q** (subgroup order): 1114157594638178892192613
- **k** (embedding degree): 12
- **Curve equation**: y² = x³ + 4

## Given Information from Trusted Setup

### G1 Group Elements:
- **P** (generator): Point in G1
- **τP**: tau times P
- **τ^{d₁}P** where d₁ = 11,726,539

### G2 Group Elements:
- **Q** (generator): Point in G2
- **τ^{d₂}Q** where d₂ = 690,320,833

### Key Observation:
- **d = d₁ + d₂ = 702,047,372**
- **d divides (q-1)** ← This is the vulnerability!
- q - 1 = 1,114,157,594,638,178,892,192,612
- (q-1) / d = 1,586,839,272,426

## Social Engineering Intelligence
Alice extracted this information from Bob:
- τ can be expressed as: **τ = 2^{k₀ + k₁·((q-1)/d)} mod r**
- k₀ is 51 bits
- The 15 most significant bits of k₀ are: `10111101110` (15854 in decimal)
- Range: **1,089,478,584,172,543 < k₀ < 1,089,547,303,649,280**

## The Cheon Attack

The Cheon attack exploits the fact that d divides (q-1). The attack works as follows:

1. **Setup**: We have τ^d in the exponent across both groups
   - From G1: We can compute τ^d using τ^{d₁}P and other elements
   - From G2: We have τ^{d₂}Q

2. **Structure of τ**: τ = 2^{k₀ + k₁·((q-1)/d)}
   - Since d divides (q-1), let m = (q-1)/d = 1,586,839,272,426
   - Then: τ = 2^{k₀ + k₁·m}

3. **Attack Strategy**:
   - Use baby-step giant-step to find k₀ and k₁
   - The constraint on k₀ (51 bits with known MSBs) significantly reduces the search space
   - Use the provided `pow_sp` and `pow_sp2` functions for efficient exponentiation

## Implementation Task
The goal is to implement the `attack()` function in `src/attack.rs` that:
1. Takes the given group elements (P, τP, τ^{d₁}P, Q, τ^{d₂}Q)
2. Applies the Cheon attack algorithm
3. Returns the secret value τ as an i128

## Success Criteria
The attack is successful when:
```rust
verify(P, tau_P, tau) == true
```
This verifies that the recovered τ satisfies: τP = τ · P

## References
- [Cheon's Attack Paper](https://iacr.org/archive/eurocrypt2006/40040001/40040001.pdf)
- The attack exploits weak parameters where d | (q-1)
- Curve parameters: https://gist.github.com/kobigurk/352036cee6cb8e44ddf0e231ee9c3f9b

