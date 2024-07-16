// SPDX-License-Identifier: GPL-3.0-or-later


use crate::{Balance, ExtrinsicBaseWeight};
use frame_support::weights::{
	WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
};
use smallvec::smallvec;
use sp_consensus_poscan::CENTS;
use sp_runtime::Perbill;

pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
	type Balance = Balance;
	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		// in IO, extrinsic base weight (smallest non-zero weight) is mapped to 1/10 CENT:
		let p = CENTS;
		let q = 10 * Balance::from(ExtrinsicBaseWeight::get());
		smallvec![WeightToFeeCoefficient {
			degree: 1,
			negative: false,
			coeff_frac: Perbill::from_rational(p % q, q),
			coeff_integer: p / q,
		}]
	}
}

#[cfg(test)]
mod tests {
	use super::WeightToFee;
	use crate::{ExtrinsicBaseWeight, MAXIMUM_BLOCK_WEIGHT};
	use frame_support::weights::WeightToFeePolynomial;
	use sp_consensus_poscan::{CENTS, DOLLARS};

	#[test]
	// This function tests that the fee for `MaximumBlockWeight` of weight is correct
	fn full_block_fee_is_correct() {
		// A full block should cost 16 DOLLARS
		assert_eq!(WeightToFee::calc(&MAXIMUM_BLOCK_WEIGHT), 16 * DOLLARS)
	}

	#[test]
	// This function tests that the fee for `ExtrinsicBaseWeight` of weight is correct
	fn extrinsic_base_fee_is_correct() {
		// `ExtrinsicBaseWeight` should cost 1/10 of a CENT
		assert_eq!(WeightToFee::calc(&ExtrinsicBaseWeight::get()), CENTS / 10)
	}
}
