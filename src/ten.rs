use nom::{
    character::complete::{line_ending, anychar},
    multi::{many1, separated_list1},
    IResult,
    
};
use std::collections::HashMap;
use std::collections::HashSet;

use glam::IVec2;
use nom_locate::LocatedSpan;

fn line_length(input: &str) -> usize {
    input.lines().next().map_or(0, |line| line.len())
}

fn num_lines(input: &str) -> usize {
    input.lines().count()
}

fn token(input: Span) -> IResult<Span, Option<(IVec2, char)>> {
    let y = input.location_line();
    let x = input.get_column();

    // Parse any character
    let (input, token) = anychar(input)?;

    // Skip unwanted characters
    if ['.', '\n'].contains(&token) {
        Ok((input, None)) // Skip the character
    } else {
        Ok((
            input,
            Some((IVec2::new(x as i32 - 1, y as i32 - 1), token)),
        ))
    }
}

pub type Span<'a> = LocatedSpan<&'a str>;

pub fn parse(input: Span) -> IResult<Span, HashMap<i8, Vec<IVec2>>> {
    let (input, items) = separated_list1(
        line_ending,
        many1(token),
    )(input)?;

	let mut result: HashMap<i8, Vec<IVec2>> = HashMap::new();

    // Filter out None results from tokens
    items
        .into_iter()
        .flatten()              // Flattens the nested Vec
        .filter_map(|item| item) // Filters out None
        .for_each(|(vec, ch)| {
            result.entry(ch.to_string().parse::<i8>().unwrap()).or_insert_with(Vec::new).push(vec);
        });
	
	Ok((input, result))
}

pub fn read() -> (IVec2, HashMap<i8, Vec<IVec2>>) {
    let input = include_str!("../res/10_input.txt");

	let bounds = IVec2::new(line_length(&input) as i32,  num_lines(&input) as i32);

    match parse(Span::new(input)) {
        Ok((_, result)) => (bounds, result), // Return the result if parsing succeeds
        Err(err) => panic!("Parsing failed: {:?}", err), // Panic if parsing fails
    }
}

fn find_neighbors(a: &Vec<IVec2>, b: &Vec<IVec2>) -> Vec<IVec2> {
    // Convert `a` into a HashSet for faster lookup
    let a_set: HashSet<_> = a.iter().collect();

    b.iter()
        .cloned()
        .filter(|&point_b| {
            a_set.iter().any(|&&point_a| {
                (point_a.x - point_b.x).abs() == 1 && point_a.y == point_b.y
                    || (point_a.y - point_b.y).abs() == 1 && point_a.x == point_b.x
            })
        })
        .collect()
}

fn find_step_up(data : &HashMap<i8, Vec<IVec2>>, possible_nodes : &Vec<IVec2>, value : i8) -> usize {

    let next_nodes = data.get(&value).unwrap();
    let neighbours = find_neighbors(&possible_nodes,&next_nodes);

    if value == 9 {
        return neighbours.len();
    }

    return find_step_up(data,&neighbours, value + 1);
}

fn find_step_up_one(data : &HashMap<i8, Vec<IVec2>>, possible_nodes : &Vec<IVec2>, value : i8) -> usize {

    let next_nodes = data.get(&value).unwrap();
    let neighbours = find_neighbors(&possible_nodes,&next_nodes);

    if value == 9 {
        return neighbours.len();
    }
    
    let mut num = 0;
    for item in neighbours {
        num += find_step_up_one(data,&vec![item], value + 1);
    }

    return num;
}

pub fn ten_a() {
    let (_, input) = read();

    let nodes = input.get(&0).unwrap();

    let mut result = 0;
    for node in nodes {
        let this = find_step_up(&input, &vec![*node],  1);
        result += this;
    }

    println!("{}", result);
}

pub fn ten_b() {
    let (_, input) = read();

    let nodes = input.get(&0).unwrap();

    let mut result = 0;
    for node in nodes {
        let this = find_step_up_one(&input, &vec![*node],  1);
        result += this;
    }

    println!("{}", result);
}