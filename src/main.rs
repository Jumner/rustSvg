use ndarray_vision::core::*;
use ndarray_vision::format::netpbm::*;
use ndarray_vision::format::*;
use ndarray_vision::processing::*;
use std::env;
use std::env::current_exe;
use std::path::PathBuf;

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

		println!("{:?}", cameraman);

		let decoder = PpmDecoder::default();
		let image: Image<u8, _> = decoder.decode_file(cameraman).expect("Could not open file");
		let image: Image<f64, _> = image.into_type();
		let image: Image<_, Gray> = image.into();

		let canny = canny_edge(&image);

		let image = image.apply_sobel().expect("Sobel error");
		let image: Image<_, RGB> = image.into();
		let mut cameraman = PathBuf::from(&root);
		cameraman.push("sobel.ppm");
		let ppm = PpmEncoder::new_plaintext_encoder();
		ppm
			.encode_file(&image, cameraman)
			.expect("Error encoding file");

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
