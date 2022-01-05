use std::env;

fn main() {
	println!("Program started!");
	let directory = match parse_args() {
		Err(msg) => panic!("{:?}", msg),
		Ok(directory) => directory,
	};
	println!("Directory {:?}", directory);
}

fn parse_args() -> Result<String, String> {
	if env::args().len() < 2 {
		return Err("Not enough arguments".to_string());
	} else if env::args().len() > 2 {
		return Err("Too many arguments".to_string());
	}
	return Ok(env::args().into_iter().last().unwrap());
}
