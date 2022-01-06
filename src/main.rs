use std::env;
use std::env::current_exe;
use std::path::PathBuf;

use image::DynamicImage;

extern crate image;

fn main() {
	let directory = match parse_args() {
		Err(msg) => panic!("{:?}", msg),
		Ok(directory) => directory,
	};

	if let Ok(mut root) = current_exe() {
		root.pop();
		root.pop();
		root.pop();
		let mut path = PathBuf::from(&root);
		path.push(directory);

		let image = image::open(path).expect("Image open failed").to_luma8();
		let detection = edge_detection::canny(image, 1.2, 0.2, 0.01);
		// detection.interpolate(x, y)
		save(detection.as_image(), "canny.png");
		for y in 0..detection.height() {
			for x in 0..detection.width() {
				let edge = detection.interpolate(x as f32, y as f32);
				if edge.magnitude() > 0.0 {
					println!("{:?}", edge.dir_norm());
				}
			}
		}
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

fn save(img: DynamicImage, path: &str) {
	img.save_with_format(path, image::ImageFormat::Png).unwrap();
	println!("Saved {:?}", path);
}
