use nom::{
    character::complete::{line_ending, anychar},
    multi::{many1, separated_list1},
    IResult,
    
};
use std::collections::{HashMap, HashSet};

use glam::IVec2;
use nom_locate::LocatedSpan;

struct FieldMap {
    data: HashMap<char, HashSet<IVec2>>,
}

impl FieldMap {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn insert(&mut self, plot: IVec2, crop: char) {
        self.data.entry(crop).or_insert_with(HashSet::new).insert(plot);
    }
}

fn token(input: Span) -> IResult<Span, Option<(IVec2, char)>> {
    let y = input.location_line();
    let x = input.get_column();

    let (input, token) = anychar(input)?;

    if ['\n'].contains(&token) {
        Ok((input, None)) // Skip the character
    } else {
        Ok((
            input,
            Some((IVec2::new(x as i32 - 1, y as i32 - 1), token)),
        ))
    }
}

pub type Span<'a> = LocatedSpan<&'a str>;

pub fn parse(input: Span) -> IResult<Span, FieldMap> {
    let (input, items) = separated_list1(line_ending, many1(token))(input)?;

    let mut result = FieldMap::new();

    items.into_iter().flatten().filter_map(|item| item).for_each(|(vec, crop)| {
        result.insert(vec, crop);
    });

    Ok((input, result))
}

pub fn read() -> FieldMap {
    let input = include_str!("../res/12_input.txt");

    match parse(Span::new(input)) {
        Ok((_, result)) => result,
        Err(err) => panic!("Parsing failed: {:?}", err),
    }
}

fn pop_first<T: Clone + Eq + std::hash::Hash>(set: &mut HashSet<T>) -> Option<T> {
    let item = set.iter().next()?.clone();
    set.remove(&item).then_some(item)
}

fn find_neighbors(a: &HashSet<IVec2>, b: &HashSet<IVec2>) -> HashSet<IVec2> {
    b.iter()
        .cloned()
        .filter(|&plot_b| {
            a.iter().any(|&plot_a| {
                (plot_a.x - plot_b.x).abs() == 1 && plot_a.y == plot_b.y
                    || (plot_a.y - plot_b.y).abs() == 1 && plot_a.x == plot_b.x
            })
        })
        .collect()
}

const ORTH_NEIGHBORS: [(i32, i32); 4] = [
    (0, -1),
    (1,  0),
    (0,  1),
    (-1, 0),
];

fn count_missing_neighbors(plot: &IVec2, set: &HashSet<IVec2>) -> usize {
    ORTH_NEIGHBORS.iter()
        .filter(|&&(dx, dy)| {
            let neighbor = *plot + IVec2::new(dx, dy);
            !set.contains(&neighbor)
        })
        .count()
}

fn count_perimeter_edges(set: &HashSet<IVec2>) -> usize {
    set.iter().map(|plot| count_missing_neighbors(plot, set)).sum()
}

pub fn calculate_perimeter_cost(mut value: &mut HashSet<IVec2>) -> usize {
	let mut result = 0;
    while let Some(start_point) = pop_first(&mut value) {
        let mut contiguous_field = HashSet::from([start_point]);
        let mut region = HashSet::from([start_point]);

        loop {
            let neighbors = find_neighbors(&region, &value);

            if neighbors.is_empty() {
				let area = contiguous_field.len();
				result += area*count_perimeter_edges(&contiguous_field);;

                break;
            } else {
                for neighbor in &neighbors {
                    value.remove(neighbor); // Remove from the original set
                    contiguous_field.insert(*neighbor);
                }
                region = neighbors;
            }
        }
    }

	return result;
}

fn neighbor_kernel(plot: &IVec2, field: &HashSet<IVec2>) -> [[bool; 3]; 3] {
    let mut kernel = [[false; 3]; 3];
    for dy in -1..=1 {
        for dx in -1..=1 {
            let neighbor = *plot + IVec2::new(dx, dy);
            kernel[(dy + 1) as usize][(dx + 1) as usize] = field.contains(&neighbor);
        }
    }
    kernel
}

fn count_field_sides(field: &HashSet<IVec2>) -> usize {
    let mut side_count = 0;
    for dir in ORTH_NEIGHBORS {
        let mut sides = HashSet::new();
        for pos in region {
            let tmp = IVec2::new(pos.x + dir.0, pos.y + dir.1);
            if !region.contains(&tmp) {
                sides.insert(tmp);
            }
        }
        let mut remove = HashSet::new();
        for side in &sides {
            let mut tmp = IVec2::new(side.x + dir.1, side.y + dir.0);
            while sides.contains(&tmp) {
                remove.insert(tmp);
                tmp = IVec2::new(tmp.x + dir.1, tmp.y + dir.0);
            }
        }
        side_count += sides.len() - remove.len();
    }

    side_count
}

pub fn calculate_edge_cost(ch : char, mut value: &mut HashSet<IVec2>) -> usize {
    let mut contiguous_fields = vec![];

	let mut result = 0;

    while let Some(start_point) = pop_first(&mut value) {
        let mut contiguous_field = HashSet::from([start_point]);
        let mut region = HashSet::from([start_point]);

        loop {
            let neighbors = find_neighbors(&region, &value);

            if neighbors.is_empty() {
				let area = contiguous_field.len();
				result += area*count_region_sides(&contiguous_field);

                println!("{}: area: {} corners {}, ", ch, area, count_region_sides(&contiguous_field));

                contiguous_fields.push(contiguous_field);
				
                break;
            } else {
                for neighbor in &neighbors {
                    value.remove(neighbor); 
                    contiguous_field.insert(*neighbor);
                }
                region = neighbors;
            }
        }
    }

	return result;
}

pub fn twelve_a(){
	let mut input = read();
	
	let mut result = 0;
	for (item, value) in input.data.iter_mut() {
		result += calculate_perimeter_cost(value);
	}

    println!("{}", result);
}

pub fn twelve_b(){
	let mut input = read();
	
	let mut result = 0;
	for (item, value) in input.data.iter_mut() {
		result += calculate_edge_cost(*item, value);
	}
	
	println!("{}", result);
}