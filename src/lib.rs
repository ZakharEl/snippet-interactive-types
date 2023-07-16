#![allow(dead_code)]
use snippet_body::*;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Placeholder(Vec<Rc<RefCell<Segment>>>);
impl InteractiveSegment for Placeholder {
	fn get_type(&self) -> &str {
		"placeholder"
	}
}
impl Field for Placeholder {
	fn get_children(&self) -> &Vec<Rc<RefCell<Segment>>> {
		let Placeholder(ref children) = self;
		children
	}
	fn get_mut_children(&mut self) -> &mut Vec<Rc<RefCell<Segment>>> {
		let Placeholder(ref mut children) = self;
		children
	}
}

#[cfg(feature = "choice")]
pub struct Choice {
	pub index_of_choice: usize,
	pub choices: Vec<Vec<Rc<RefCell<Segment>>>>,
}
#[cfg(feature = "choice")]
impl InteractiveSegment for Choice {
	fn get_type(&self) -> &str {
		"choice"
	}
}
#[cfg(feature = "choice")]
impl Field for Choice {
	fn get_children(&self) -> &Vec<Rc<RefCell<Segment>>> {
		let Choice {
			index_of_choice,
			ref choices,
		} = self;
		&choices[*index_of_choice]
	}
	fn get_mut_children(&mut self) -> &mut Vec<Rc<RefCell<Segment>>> {
		let Choice {
			index_of_choice,
			ref mut choices,
		} = self;
		&mut choices[*index_of_choice]
	}
}

#[cfg(feature = "shell-code")]
pub struct ShellCode;
#[cfg(feature = "shell-code")]
impl InteractiveSegment for ShellCode {
	fn get_type(&self) -> &str {
		"shell code"
	}
}
#[cfg(feature = "shell-code")]
impl Interpreter for ShellCode {
	fn evaluate(&self, source_code: &str) -> String {
		let options = run_script::ScriptOptions::new();
		let args = vec![];
		let (_, output, _) = run_script::run(&source_code, &args, &options).unwrap();
		output
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
