fn main() {
	for n in 1..100 {
		let is_fizz = n % 3 == 0;
		let is_buzz = n % 5 == 0;
		if is_fizz && is_buzz {
			println!("FizzBuzz");
		} else if is_fizz {
			println!("Fizz");
		} else if is_buzz {
			println!("Buzz");
		} else {
			println!("{}", n);
		}
	}
}