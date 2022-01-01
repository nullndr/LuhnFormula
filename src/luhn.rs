
fn double_digit_value(digit: &u32) -> u32 {
	let double_digit = digit * 2;
	if double_digit >= 10 {
		1 + double_digit % 10
	} else {
		double_digit
	}
}

pub fn validate(number: u64) -> Result<(), ()> {

	let mut digit_sum: u32 = 0;

	for (index, value) in number.to_string().chars().enumerate() {
		digit_sum += if (index + 1) % 2 == 0 {
			value.to_digit(10).unwrap()
		} else {
			double_digit_value(&(value.to_digit(10).unwrap()))
		};
	}

	let checksum = digit_sum * 9 % 10;
	digit_sum += checksum;

	if digit_sum % 10 == 0 { Ok(()) } else { Err(()) }
}
