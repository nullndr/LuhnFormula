mod luhn;

fn main() {
	match luhn::validate(79927398710) {
		Ok(_) => println!("All right"),
		Err(_) => println!("It all went wrong")
	}
}
