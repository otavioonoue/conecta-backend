use rust_decimal::{Decimal, prelude::ToPrimitive};
pub struct CurrencyHelper;

impl CurrencyHelper {
	pub fn to_cents(decimal_value: Decimal) -> i64 {
		decimal_value
			.round_dp(2)
			.checked_mul(Decimal::new(100, 0))
			.and_then(|v| v.to_i64())
			.unwrap_or_default()
	}
}