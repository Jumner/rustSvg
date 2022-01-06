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

	let mut normals = Normals::new();
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
					let my_edge = MyEdge::new(x, y, edge.dir_norm().0, edge.dir_norm().1);
					normals.push(my_edge);
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

struct MyEdge {
	position: (usize, usize),
	direction: (f32, f32),
}

impl MyEdge {
	pub fn new(x: usize, y: usize, dx: f32, dy: f32) -> Self {
		Self {
			position: (x, y),
			direction: (dx, dy),
		}
	}
}

struct Normals {
	list: Vec<MyEdge>,
}

impl Normals {
	pub fn new() -> Self {
		Self { list: vec![] }
	}
	pub fn push(&mut self, item: MyEdge) {
		self.list.push(item);
	}
}
