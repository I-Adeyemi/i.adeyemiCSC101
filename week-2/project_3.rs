fn main() {
	let p:f32 = 210_000.0;
	let r:f32 = 5.0;
	//ask how to use raised to power
	let n:f32 = powf(3.0);

	// compound interest
	let a = p * (1.0 - (r/100.0)) n;
	println!("Amount is {}", a);
	let ci = a - p;
	println!("Compound Interest is {} ", ci);
}
