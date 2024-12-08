use std::{
	collections::{HashSet, HashMap},
	ops::{AddAssign, SubAssign, Add}
};

fn get_maze() -> Vec<String> {
	vec![
		"....................................#............##........#..#.#..#.........#.............#..............#.......................".to_string(),
		"..................##.........#....................##.....#.......#.......................................#..........#.............".to_string(),
		".......#...........#.........................................................................................#.#.#................".to_string(),
		".#..........#...............................#....................................#...#.....#...#.#....................#....#......".to_string(),
		"..........#..........................................#............#.........................#..#......#....#..#..........#........".to_string(),
		"..........................................................................#..........................................#.........#..".to_string(),
		".....#....#...........................#......#..........#........................#..........#.............#...........#..........#".to_string(),
		"................#.......................#...........#.............................#.....#..#..................................#...".to_string(),
		"........................................................................#..#.......................................#..............".to_string(),
		"........#......................#.....#.......#.............#....#.........#....#........................#.............#...........".to_string(),
		".............#....#.......#........#..#.......##................#.......................................#.........................".to_string(),
		"......#..............#.#...#...............#.........................#.....#............#..#.........#...............#............".to_string(),
		"..................#...#..............................................#................#.......#................#..................".to_string(),
		"....#...#.............#........................#....................................#......#................#......#..............".to_string(),
		"..........................................................................................#...#.........##........................".to_string(),
		".............#.#...............#.....#.#......#.............................#.......................................#.....#.......".to_string(),
		".................................................#................................................................#...............".to_string(),
		"........#.#......................#......#........#.......................................................#........................".to_string(),
		"....#........#......#..................................................................................#............#.............".to_string(),
		"...........#.........#.......#...............#........................#........................................................#.#".to_string(),
		"........................##..................#........................#..........................................#.............#...".to_string(),
		"..........#........#........#.........#..................#.....#..............................#..#..............#.................".to_string(),
		"#..............................................................#..#............................#..........#.......................".to_string(),
		".................................................................#...............#...#.#.........#.#.................#...........#".to_string(),
		".....................................#.....................#.....................#........#....#....#.............................".to_string(),
		"#..#................#.....#....................................##............#...............#...#..........#.....................".to_string(),
		".................#.....#.....................#................#...........#...................................................#...".to_string(),
		"..................................#........#.................#....#..................#.......................................#....".to_string(),
		"..#....#............................#................................................##.........................................#.".to_string(),
		"...............#......................................................#......##...................................................".to_string(),
		".........................#..................................#................##.#..#.#.................#.......#..................".to_string(),
		".................#......................................#...#.....................................................................".to_string(),
		".........#..........................................................................................#.......#.....................".to_string(),
		"....##..#.........#..............#..#............#............#...................................#...............................".to_string(),
		"......#.......................................#................................................................#..................".to_string(),
		"....#..................#............................#................................................................#............".to_string(),
		"...............................##.#..............#....#............#.........................................#.....#..............".to_string(),
		"........#........#....#.................................................................#.....#...................#...............".to_string(),
		"#...........#..#.........................................#..................................................#..##.......#.....#..#".to_string(),
		"................#......#...........##..........................................................#....#.............................".to_string(),
		"..........#..........#.............................................................#..............#.................#..........#..".to_string(),
		"#.................#....................................................................................#..#..............#........".to_string(),
		".....#..............#.........................#.....#...........#......................#..........................................".to_string(),
		".....##.....................#...............................................................#.....................................".to_string(),
		"...................#.................................................................................#.....................#..#...".to_string(),
		"...............................................^......................#...........................#...............................".to_string(),
		".......#...............#.........#.........#......#.......................................................................#.......".to_string(),
		"........#.........#..#..##.................................#.....#..............#...........#..#........#.........................".to_string(),
		"..............................................................................#......#.#...............#..#..#.....#....#.........".to_string(),
		"............#...#.......##............#....#...........................................................................#..........".to_string(),
		"..................................##........#.....##.......#......................................................................".to_string(),
		".......................#...........................................#..................................#...#.........#.............".to_string(),
		"............#......................................................#....#..........#...................#........................#.".to_string(),
		"....................................................#......................#..........................#....#......................".to_string(),
		"..................#.##....................................#..................................#.................#....#...#.........".to_string(),
		"........#.....................................................................................#...#...............................".to_string(),
		"....................................................#..........................................#........#..#.#.................#..".to_string(),
		"....#...........#..#......#.........................#.....................#............................#.....#.#..............#...".to_string(),
		".............................#.......#..........#..................................................................#....#.....#...".to_string(),
		".....#.#................#...................#...............................................#.....................................".to_string(),
		"....#........................................#......#.............................#...............................................".to_string(),
		".......#..........#........................#.....................................#..............#......#...##.....................".to_string(),
		".....#..#.............................##........................................................#.....#..............#.#..........".to_string(),
		"...........................................#......#......................................#..............#.........................".to_string(),
		"...................#.........#....#................#.....#.........#....................#............#..#........#...............#".to_string(),
		".....#.....................#......................................................................#...............................".to_string(),
		"...............#.......................................#...#.................#..............#.....................................".to_string(),
		".........................#........................................................#...............................................".to_string(),
		"..................................#..........................................#...........#.....................#..................".to_string(),
		".......................#.#............#.........................................#.................................................".to_string(),
		"..........................#...................#......................#......................................#.....................".to_string(),
		".........#.........................#..............................................................#...............................".to_string(),
		"..#..............................#...............#..................................###..........................................#".to_string(),
		".............................................#...............#.....#........................................................#..#..".to_string(),
		"................#..........................#.......................................#........................................##.#..".to_string(),
		"......##........#.....................................#............#..................#...............................#...........".to_string(),
		"..#..................#......................................#....#.......................#................................#.......".to_string(),
		"........................#...............#....#...#..........................................................................#.....".to_string(),
		".............#.#..........#...#....#.................##...........................................................................".to_string(),
		"......#....#.......................................................................#.....................................#..#.....".to_string(),
		".............#.......#............#...#..........#...............#..................................................#..........#..".to_string(),
		"..........................#...........................................................#.............#.............................".to_string(),
		"#......................................#........#......#..........................................................................".to_string(),
		"................#..........................................................#......................................................".to_string(),
		"..................#................................................................#..........................#..........#.......#".to_string(),
		".......#.....#............##..............................................................#....................#.....#......#.....".to_string(),
		"...........................................#..........#.#............................................#............................".to_string(),
		".......#.......................#..........................................................#.......................................".to_string(),
		".................#........................#.........#...#............................................................#............".to_string(),
		"................................................#...................#.........#...........#...........................#...........".to_string(),
		"..#..............#......#..............................##............#..................#.......#.................................".to_string(),
		".#.#.........................................#......##.....##........#.............#..#..................##....#.#...#............".to_string(),
		"..#.....................#.......#....#...............#...............#...................................................#........".to_string(),
		".#...#...............#..................................................................#..........#....#.................#.#.....".to_string(),
		".#...................#...................................................................#.............#.......................#..".to_string(),
		".....................................#.....#..#...........#...........#..........#....................................#...........".to_string(),
		".................................................................................#..#.............##....#...#............#........".to_string(),
		"......................#....#........#......#...................#.............................................#....................".to_string(),
		".......................................#..........................#............................#.#..........................#.....".to_string(),
		".#........................##....................................#.......#...........................#..#.......................#..".to_string(),
		".........#.........#................................................................#.................#........#..................".to_string(),
		"..............#.......#.....#....................#........##.............#....#...............#.............#...........#...#.....".to_string(),
		".................................................................#..#........#...#...#.........#..................................".to_string(),
		"................#......##..........#..........#.........#....#............................................#.#.............#.......".to_string(),
		"...#...................................................................#.......................................#..................".to_string(),
		".......#..................#............#.....#................#..#....##.......#...............#..................................".to_string(),
		"............#............##....#...................#..........................#......................................#...#...#....".to_string(),
		"......#.....#..........##...............#......#...............................#......#...........................................".to_string(),
		".................#.......#.......................................................#............#........#...#.......#...........#..".to_string(),
		"......................##........#...................#.....#....................#...#....#........................#................".to_string(),
		".................#.....#.......#...........#.......#....................##....................................................#.#.".to_string(),
		"......#....................................#.........................#..................................#................#.#..#...".to_string(),
		"..............#...........................................................................................................#.......".to_string(),
		"#.#......#.................................................................................#...........#...#......................".to_string(),
		"...........................................#.#........#....#............#.....................#...................................".to_string(),
		"...#.............#.....................................................................................................#..........".to_string(),
		"..............#..........#............#...#..............#........................................##........#.....#.......#.......".to_string(),
		"..................##........#.............................................#.........................................###...........".to_string(),
		"..........#......................#............................................................................#....#..............".to_string(),
		".............#........#.............#.#...............#.....#..........#.....#....#...#...........#....#..........................".to_string(),
		"........#...#.......#........................#.............#..........#........#.......................#..........................".to_string(),
		"..................................................................#...............#......#...#.......#............................".to_string(),
		".............#..........#...................##.............#.........#..................#.........................................".to_string(),
		".#.....#..#...#...............#.#.............#...........#.......................................................................".to_string(),
		"..............#..#.........................#..............#..#..............#.......................#.#...........................".to_string(),
		"............#.......................#....#................#..........#.......................#.....................#..............".to_string(),
		"#.................#............................#...##....#.....#.....................................................#............".to_string(),
		".............................#........................#..................................................#....#...................".to_string(),
		".##...........................................#.....#....#....#.......................#...#.....#...#............#................".to_string(),
		"........#.............#.........#..............#........................#.....................#...#...............................".to_string()
	]
}

const UP : char = '^';
const DOWN : char = 'v';
const RIGHT : char = '>';
const LEFT : char = '<';
const WALL : char = '#';
const EMPTY : char = '.';

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Vec2{
	row : i16,
	column: i16
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.row += rhs.row;
        self.column += rhs.column;
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            row: self.row + rhs.row,
            column: self.column + rhs.column,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.row -= rhs.row;
        self.column -= rhs.column;
    }
}

fn in_bounds(
		position : &Vec2, num_rows: i16, num_columns : i16
	) -> bool {
		position.row >= 0 && position.column >= 0 && position.row < num_rows && position.column < num_columns
}

fn get_value(
	maze : &Vec<String>, position : &Vec2, num_rows: i16, num_columns : i16
) -> Option<char> {
	if in_bounds(&position, num_rows, num_columns) {
		return maze[position.row as usize].chars().nth(position.column as usize);
	} else {
		return None;
	}
}

fn adjust_direction(current_direction : char) -> char {
	match current_direction {
		UP => return RIGHT,
		RIGHT => return DOWN,
		DOWN => return LEFT,
		LEFT => return UP,
		_ => panic!("Aghhh!")
	}
}

fn get_initial_position_and_direction(
    maze: &Vec<String>,
    directions: &HashSet<char>,
) -> (Option<Vec2>, Option<char>) {
    let mut initial_position: Option<Vec2> = None;
    let mut initial_direction: Option<char> = None;

    for (row_index, row) in maze.iter().enumerate() {
        if let Some((column_index, value)) = row
            .chars()
            .enumerate()
            .find(|(_, item)| directions.contains(item))
        {
            initial_position = Some(Vec2 {
                row: row_index as i16,
                column: column_index as i16,
            });
            initial_direction = Some(value);
            break;
        };
    }

    (initial_position, initial_direction)
}


fn check_escape(mut maze : Vec<String>, initial_position : Option<Vec2>, initial_direction : Option<char>, direction_map : &HashMap<char, Vec2> ) -> i32 {

	let mut current_direction : Option<char> = initial_direction;
	let mut count = 1;
	
	if let Some(mut position) = initial_position {
		let num_rows = maze[0].len() as i16;
		let num_columns = maze.len() as i16;

		// Ensure current_direction is Some. If not, handle that case.
		while let Some(current_character) = get_value(&maze, &position, num_rows, num_columns) {

			if current_character != WALL {
				if let Some(current_direction) = current_direction {

					let next_character = get_value(&maze, &(position + direction_map[&current_direction]), num_rows, num_columns);
					if  next_character == Some(current_direction)  {
						return -1;
					} 

					if current_character == EMPTY { 
						maze[position.row as usize].replace_range(
							position.column as usize..position.column as usize+1, &current_direction.to_string()
						);

						count += 1;
					}
					position += direction_map[&current_direction];
				} else {
					panic!("No current direction set!");
				}
			} else {
				if let Some(dir) = current_direction {
					position -= direction_map[&dir];
					current_direction = Some(adjust_direction(dir));
					continue;
				} else {
					panic!("No current direction set!");
				}
			}
		}
	}

	count
} 


pub fn six_a() {
	let maze: Vec<String> = get_maze();

	let directions: Vec<char> = vec![UP, LEFT, DOWN, RIGHT];	
	let direction_set: HashSet<_> = directions.iter().copied().collect();

	let (initial_position, initial_direction) = get_initial_position_and_direction(&maze, &direction_set);

	let direction_map:HashMap<char, Vec2>  = HashMap::from(
		[
			(UP, Vec2{row : -1, column : 0}),
			(LEFT, Vec2{row : 0, column : -1}),
			(DOWN, Vec2{row : 1, column : 0}),
			(RIGHT, Vec2{row : 0, column : 1}),
		]
	);


	println!("Route length {}", check_escape(maze, initial_position, initial_direction, &direction_map));
}

pub fn six_b() {

	let maze = get_maze();
	let mut checked_obstructions = HashSet::new();
	let mut obs_positions : i32 = 0;

	let directions: Vec<char> = vec![UP, LEFT, DOWN, RIGHT];	
	let direction_set: HashSet<_> = directions.iter().copied().collect();

	let direction_map: HashMap<char, Vec2> = HashMap::from(
		[
			(UP, Vec2{row : -1, column : 0}),
			(LEFT, Vec2{row : 0, column : -1}),
			(DOWN, Vec2{row : 1, column : 0}),
			(RIGHT, Vec2{row : 0, column : 1}),
		]
	);

	let (initial_position, initial_direction) = get_initial_position_and_direction(&maze, &direction_set);
	let mut current_direction : Option<char> = initial_direction;

	if let Some(mut position) = initial_position {
		let num_rows = maze[0].len() as i16;
		let num_columns = maze.len() as i16;

		// Ensure current_direction is Some. If not, handle that case.
		while let Some(current_character) = get_value(&maze, &position, num_rows, num_columns) {
			match current_character {
				EMPTY | UP | DOWN | RIGHT | LEFT => {
					if let Some(current_direction) = current_direction {

						if !checked_obstructions.contains(&position) {
							checked_obstructions.insert(position);

							let mut new_maze = maze.clone();

							if maze[position.row as usize].chars().nth(position.column as usize) != Some(WALL) {
								// Place a wall and reset the guard's path
								new_maze[position.row as usize].replace_range(
									position.column as usize..position.column as usize + 1,
									&WALL.to_string(),
								);
							}

							if check_escape(
								new_maze, 
								Some(position),  
								Some(current_direction),
								&direction_map
							) == -1 {
								obs_positions += 1;
							}
						}
						
						position += direction_map[&current_direction];
					} else {
						panic!("No current direction set!");
					}
				}
				WALL => {
					if let Some(dir) = current_direction {
						position -= direction_map[&dir];
						current_direction = Some(adjust_direction(dir));
						continue;
					} else {
						panic!("No current direction set!");
					}
				}
				_ => panic!("Unrecognized symbol!"),
			}
		}
	}

	println!("Obstructions: {}", obs_positions);
} 

