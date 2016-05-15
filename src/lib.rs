extern crate rustfft;
extern crate num;

pub fn run() {
	// Settings:
	let fft_len = 16;
	// Create fft instance:
	let mut fft = rustfft::FFT::new(fft_len, false);
	// Create & print signal:
	let mut signal = Vec::new();
	print!("---> ");
	for i in 0..fft_len {
		let x = (i % 2) as f64;
		signal.push(num::Complex{ re: x, im: 0.0 });
		print!("{} ", x);
	}
	println!("");
	// Create & print spectrum:
	let mut spectrum = signal.clone();
	fft.process(&signal, &mut spectrum);
	print!("<--- ");
	for x in spectrum {
		print!("{} ", x.norm());
	}
	println!("");
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn it_works() {
		run();
	}
}
