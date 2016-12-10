use std::io::Read;
use std::io::Write;
use std::io::Result;
use std::fs::File;

pub struct FileData
{
	path: Option<String>, //the original path of the file, if provided
	content: Vec<Vec<char>>, //the content as a semi 2D-array of chars
	cursor_line: usize, //the line the cursor is in
	cursor_char: usize, //the character the cursor is in the current line
	modified: bool //ind. wether data has been changed since last save
}

impl FileData
{
	pub fn new() -> FileData
	{
		let mut linevec = Vec::new();
		let charvec = Vec::new();
		linevec.push(charvec);
		FileData
		{
			path: None,
			content: linevec,
			cursor_line: 0,
			cursor_char: 0,
			modified: false
		}
	}

	//load data from path
	pub fn from(filepath: String) -> Result<FileData>
	{
		let mut file = try!(File::open(filepath.clone()));
		let mut filecontent = String::new();
		try!(file.read_to_string(&mut filecontent));
		let mut linevec = Vec::new();
		let mut charvec = Vec::new();
		for ch in filecontent.chars()
		{
			if ch == '\n'
			{
				linevec.push(charvec);
				charvec = Vec::new();
			}
			else 
			{
			    charvec.push(ch);
			}
		}
		linevec.push(charvec);
		Ok(FileData
		{
			path: Some(filepath),
			content: linevec,
			cursor_line: 0,
			cursor_char: 0,
			modified: false
		})
	}

	//reset to untitled document
	pub fn clear(&mut self)
	{
		let dummy = FileData::new();
		self.path = dummy.path;
		self.content = dummy.content;
		self.cursor_line = dummy.cursor_line;
		self.cursor_char = dummy.cursor_char;
		self.modified = dummy.modified;
	}

	//return line at "line"
	pub fn get_line(&self, line: usize) -> Option<&Vec<char>>
	{
		if line < self.content.len()
		{
			Some(self.content.get(line).unwrap())
		}
		else 
		{
		    None
		}
	}

	pub fn get_cursor_line(&self) -> usize
	{
		self.cursor_line
	}

	pub fn get_cursor_char(&self) -> usize
	{
		self.cursor_char
	}

	pub fn get_path(&self) -> Option<String>
	{
		self.path.clone()
	}

	pub fn set_path(&mut self, path: Option<String>)
	{
		self.path = path;
	}

	pub fn get_title(&self) -> String
	{
		match self.path
		{
			None => String::from("<Untitled>"),
			Some(ref v) => v.clone()
		}
	}

	pub fn get_lines(&self) -> usize
	{
		self.content.len()
	}

	pub fn get_line_number_len(&self) -> usize
	{
		format!("{}", self.get_lines()).len()
	}

	pub fn is_modified(&self) -> bool
	{
		self.modified
	}

	//CURSOR FUNCTIONS

	pub fn move_cursor_up(&mut self)
	{
		if self.cursor_line != 0
		{
			self.cursor_line -= 1;
			let line_len = self.content.get(self.cursor_line).unwrap().len();
			if self.cursor_char > line_len
			{
				self.cursor_char = line_len;
			}
		}
	}

	pub fn move_cursor_left(&mut self)
	{
		if self.cursor_char != 0
		{
			self.cursor_char -= 1;
		}
		else 
		{
			if self.cursor_line != 0
			{
				self.move_cursor_up();
		    	self.cursor_char = self.get_line(self.get_cursor_line()).unwrap().len();
		    }
		}
	}

	pub fn move_cursor_down(&mut self)
	{
		if self.cursor_line < self.get_lines() - 1
		{
			self.cursor_line += 1;
			let line_len = self.content.get(self.cursor_line).unwrap().len();
			if self.cursor_char > line_len
			{
				self.cursor_char = line_len;
			}
		}
	}

	pub fn move_cursor_right(&mut self)
	{
		let cur_line_len = self.get_line(self.get_cursor_line()).unwrap().len();
		if self.cursor_char < cur_line_len
		{
			self.cursor_char += 1;
		}
		else 
		{
		    if self.cursor_line < self.get_lines() - 1 
		    {
		    	self.move_cursor_down();
		    	self.cursor_char = 0;
		    }
		}
	}

	//EDITING FUNCTIONS

	pub fn write_char(&mut self, ch: char)
	{
		let cline = self.get_cursor_line();
		let cchar = self.get_cursor_char();
		self.content.get_mut(cline).unwrap().insert(cchar, ch);
		self.move_cursor_right();
		self.modified = true;
	}

	pub fn backspace(&mut self)
	{
		let cline = self.get_cursor_line();
		let cchar = self.get_cursor_char();
		if cchar != 0
		{
			self.content.get_mut(cline).unwrap().remove(cchar-1);
			self.cursor_char -= 1;
		}
		else if cline != 0
		{
		    let mut cur_line = self.content.get(cline).unwrap().clone();
		    let new_char = self.content.get(cline-1).unwrap().len();
		    self.content.get_mut(cline-1).unwrap().append(&mut cur_line);
		    self.content.remove(cline);
		    self.cursor_char = new_char;
		    self.cursor_line -= 1;
		}
		self.modified = true;
	}

	pub fn remove(&mut self)
	{
		let cline = self.get_cursor_line();
		let cchar = self.get_cursor_char();
		let line_len = self.content.get(cline).unwrap().len();
		if cchar != line_len
		{
			self.content.get_mut(cline).unwrap().remove(cchar);
		}
		else if cline != self.content.len()-1
		{
		    let mut next_line = self.content.get(cline+1).unwrap().clone();
		    self.content.get_mut(cline).unwrap().append(&mut next_line);
		    self.content.remove(cline + 1);
		}
		self.modified = true;
	}

	pub fn enter(&mut self)
	{
		let cline = self.get_cursor_line();
		let cchar = self.get_cursor_char();
		let clip = self.content.get_mut(cline).unwrap().split_off(cchar);
		self.content.insert(cline+1, clip);
		self.cursor_char = 0;
		self.cursor_line += 1;
		self.modified = true;
	}

	//copy-move to string
	pub fn to_string_copy(&self) -> String
	{
		let mut result = String::new();
		for line in &self.content
		{
			for ch in line
			{
				result.push(ch.clone());
			}
			result.push('\n');
		}
		result
	}

	//load from file
	pub fn open(&mut self, path: String) -> Result<()>
	{
		let dummy = try!(FileData::from(path));
		self.path = dummy.path;
		self.content = dummy.content;
		self.cursor_line = dummy.cursor_line;
		self.cursor_char = dummy.cursor_char;
		self.modified = dummy.modified;
		Ok(())
	}

	//save to original file at self.path
	pub fn save(&mut self) -> Result<()>
	{
		let dummy = String::new();
		let path = match self.path
		{
			Some(ref v) => v.clone(),
			None => dummy //should not happen! check this!
		};
		self.save_to(path)
	}

	//save to file other than original file at self.path
	pub fn save_to(&mut self, path: String) -> Result<()>
	{
		let content = self.to_string_copy();
		let bytes = content.into_bytes();
		let mut file = try!(File::create(path));
		try!(file.write_all(&bytes[..]));
		self.modified = false;
		Ok(())
	}
}