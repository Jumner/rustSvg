use image::pnm::PnmDecoder;
use image::ImageFormat;
use ndarray_vision::core::*;
use ndarray_vision::format::netpbm::*;
use ndarray_vision::format::*;
use ndarray_vision::processing::*;
use std::env;
use std::env::current_exe;
use std::fs::File;
use std::path::PathBuf;

extern crate image;

fn main() {
	println!("Program started!");
	let directory = match parse_args() {
		Err(msg) => panic!("{:?}", msg),
		Ok(directory) => directory,
	};
	println!("Directory {:?}", directory);

	if let Ok(mut root) = current_exe() {
		root.pop();
		root.pop();
		root.pop();
		let mut cameraman = PathBuf::from(&root);
		cameraman.push(directory);

		let img = image::open(cameraman).expect("Error opening file");
		let mut output = File::create(&format!("tmp.jpg")).expect("tmp File creation failed");
		img
			.write_to(&mut output, ImageFormat::Pnm)
			.expect("tmp write failed");

		let decoder = PpmDecoder::default();
		// let de = PnmDecoder::new(read)
		let image: Image<u8, _> = decoder.decode_file("tmp.ppm").expect("Could not open file");
		let image: Image<f64, _> = image.into_type();
		let image: Image<f64, Gray> = image.into();

		let canny = canny_edge(&image);

		let ppm = PpmEncoder::new_plaintext_encoder();

		let mut cameraman = PathBuf::from(&root);
		cameraman.push("canny.ppm");
		ppm
			.encode_file(&canny.into(), cameraman)
			.expect("Error encoding file");
	}
}

fn parse_args() -> Result<String, String> {
	if env::args().len() < 2 {
		return Err("Not enough arguments".to_string());
	} else if env::args().len() > 2 {
		return Err("Too many arguments".to_string());
	}
	return Ok(env::args().into_iter().last().unwrap());
}

fn canny_edge(img: &Image<f64, Gray>) -> Image<f64, Gray> {
	let x = CannyBuilder::<f64>::new()
		.lower_threshold(0.3)
		.upper_threshold(0.5)
		.blur((5, 5), [0.4, 0.4])
		.build();
	let res = img.canny_edge_detector(x).expect("Canny failed");
	Image::from_data(res.data.mapv(|x| if x { 1.0 } else { 0.0 }))
}
