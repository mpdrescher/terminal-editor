extern crate rustbox;

use std::env;
use std::io::Result;
use std::path::Path;

mod filedata;
use filedata::FileData;

mod display;
use display::Display;

fn main() 
{
	let fd = match init_data()
	{
		Ok(v) => v,
		Err(e) => {
			println!("could not open specified file: {}", e); 
			return;
		}
	};
	let display = Display::new(fd);
	display.run();
}

//create a new data object from cmd args
fn init_data() -> Result<FileData>
{
	let mut args = env::args().skip(1);
	if args.len() > 0
	{	
		let path_str = args.next().unwrap_or(String::new());
		let path_str_copy = path_str.clone();
		let path = Path::new(&path_str_copy);
		if path.exists() == false
		{
			Ok(FileData::new_with_name(path_str))
		}
		else 
		{
			FileData::from(path_str)    
		}
	}
	else
	{
		Ok(FileData::new())
	}
}