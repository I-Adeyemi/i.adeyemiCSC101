fn main() {
	let qt:f32 = 2.0;
	let t:f32 = 450_000.0;
	let qm:f32 = 1.0;
	let m:f32 = 1_500_000.0;
	let qh:f32 = 3.0;
	let h:f32 = 750_000.0;
	let qd:f32 = 3.0;
	let d:f32 = 2_850_000.0;
	let qa:f32 = 1.0;
	let a:f32 = 250_000.0;

	// sum
	let sum = (qt * t) + (qm * m) + (qh * h) + (qd * d) + (qa * a);
	println!("Sum is {} ", sum);
	let qty = qt + qm + qh + qd + qa;

	// average
	let a = sum/qty; 
	println!("Average is {}", a);
}