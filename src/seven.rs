use std::{
	fs::File,
	io::{self, BufRead},
	path::Path
};

#[derive(Debug, Clone)]
struct Equation{
	target : i64,
	parts : Vec<i64>,
	check_mult : bool, 
	check_add : bool,
	check_concat : bool
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operations {
	ADD,
	MULTIPLY,
	CONCATINATE
}

impl Equation {
	fn check(&self, result : i64, index : i64) -> bool {

		if index == 0 {
			return result == self.parts[0];
		} 
		
		if result < self.parts[0] {
			return false;
		} 

		let part = self.parts[index as usize];
		let new_goal = result - part;
		let tens: i64 = 10i64.pow(part.ilog10() + 1);

		let concat_check = new_goal % tens == 0 && self.check(new_goal / tens, index - 1) && self.check_concat;
		let mult_check = (result % part == 0 && self.check(result / part, index - 1)) && self.check_mult;
		let add_check = self.check(result - part, index - 1) && self.check_add;

		add_check || mult_check || concat_check
	}
}

#[derive(Debug)]
struct Equations{
	total :i64,
	equations : Vec<Equation>
}

impl Equations {
	fn parse_line(line: &str) -> Option<Equation> {
		let parts: Vec<&str> = line.split(':').collect();
		if parts.len() != 2 {
			return None;
		}
		
		let target = parts[0].trim().parse::<i64>().ok()?;
		let parts_vec = parts[1]
			.split_whitespace()
			.filter_map(|x| x.parse::<i64>().ok())
			.collect();
	
		Some(Equation {
			target, 
			parts: parts_vec, 			
			check_mult : false, 
			check_add : false,
			check_concat : false
		})
	}

	fn read(filename: &str) -> Result<Equations, std::io::Error>{
		let path = Path::new(filename);
    	let file = File::open(&path)?;
		let reader = io::BufReader::new(file);

		let equations: Vec<Equation> = reader
			.lines()
			.filter_map(|line| line.ok())
			.filter_map(|line| Self::parse_line(&line))
			.collect();

		Ok(Equations {
			total : 0,
			equations : equations
		})
	}

	fn test(&mut self, operations_options: &[Operations]) {
		self.total = 0;
		for equation in &mut self.equations {

			equation.check_add = operations_options.contains(&Operations::ADD);
			equation.check_mult = operations_options.contains(&Operations::MULTIPLY);
			equation.check_concat = operations_options.contains(&Operations::CONCATINATE);

			if equation.check(equation.target, equation.parts.len() as i64 - 1) {
				self.total += equation.target;
			}
		}
	}
}

pub fn seven_a() {
	match Equations::read("./res/07_input.txt") {
        Ok(mut equations) => {
			equations.test(&[Operations::ADD, Operations::MULTIPLY]);
			println!("Total: {}", equations.total); 
		},
        Err(err) => println!("Error reading file: {}", err),
    }
}

pub fn seven_b() {
	match Equations::read("./res/07_input.txt") {
        Ok(mut equations) => {
			equations.test(&[Operations::ADD, Operations::MULTIPLY, Operations::CONCATINATE]);
			println!("Total: {}", equations.total); 
		},
        Err(err) => println!("Error reading file: {}", err),
    }
}