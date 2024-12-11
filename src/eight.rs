use nom::{
    character::complete::{line_ending, anychar},
    multi::{many1, separated_list1},
    IResult,
};
use std::collections::HashMap;
use glam::IVec2;
use itertools::Itertools;
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

pub fn parse(input: Span) -> IResult<Span, HashMap<char, Vec<IVec2>>> {
    let (input, items) = separated_list1(
        line_ending,
        many1(token),
    )(input)?;

	let mut result: HashMap<char, Vec<IVec2>> = HashMap::new();

    // Filter out None results from tokens
    items
        .into_iter()
        .flatten()              // Flattens the nested Vec
        .filter_map(|item| item) // Filters out None
        .for_each(|(vec, ch)| {
            result.entry(ch).or_insert_with(Vec::new).push(vec);
        });
	
	Ok((input, result))
}

pub fn read() -> (IVec2, HashMap<char, Vec<IVec2>>) {
    let input = include_str!("../res/08_input.txt");

	let bounds = IVec2::new(line_length(&input) as i32,  num_lines(&input) as i32);

    match parse(Span::new(input)) {
        Ok((_, result)) => (bounds, result), // Return the result if parsing succeeds
        Err(err) => panic!("Parsing failed: {:?}", err), // Panic if parsing fails
    }
}
pub fn eight_a() {
	let (bounds, data) = read();

	let mut antinodes = vec![];
	for (_, vecs) in &data {
		let it : Vec<Vec<&IVec2>> = vecs.into_iter().combinations(2).collect();
		let diffs : Vec<_> = it.iter().flat_map(|item| {
			let diff = item[1] - item[0];
			vec![item[0] - diff, item[1] + diff]
		})
		.filter(| item |
			item.x >= 0 && item.y >= 0 && item.x < bounds.x && item.y < bounds.y
		)
		.collect();

		antinodes.push(diffs);
	}

	let antinodes : Vec<_> = antinodes.iter().flatten().unique().collect();
	println!("Num antinodes: {}", antinodes.len());
}

pub fn eight_b() {
    let (bounds, data) = read();

    let mut antinodes = vec![];
    for (_ch, vecs) in &data {
        let it: Vec<Vec<&IVec2>> = vecs.into_iter().combinations(2).collect();
        let diffs: Vec<_> = it
            .iter()
            .flat_map(|item| {
                let diff = item[1] - item[0];
				let node = item[0];
                
                let pos_points: Vec<IVec2> = (0..)
                    .map(|n| node + n * diff)
                    .take_while(|&p| p.x >= 0 && p.y >= 0 && p.x < bounds.x && p.y < bounds.y)
                    .collect();

                let neg_points: Vec<IVec2> = (1..)
                    .map(|n| node - n * diff)
                    .take_while(|&p| p.x >= 0 && p.y >= 0 && p.x < bounds.x && p.y < bounds.y)
                    .collect();

                pos_points.into_iter().chain(neg_points)
            })
            .collect();

        antinodes.push(diffs);
    }

    let antinodes: Vec<_> = antinodes.iter().flatten().unique().collect();
    println!("Num antinodes: {}", antinodes.len());
}