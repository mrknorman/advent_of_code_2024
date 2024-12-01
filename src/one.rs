pub fn one_a(
		mut list_a : Vec<u32>,
		mut list_b : Vec<u32>
	) {
	
	list_a.sort();
	list_b.sort();

	let difference : Vec<i32> = list_a.into_iter().zip(list_b).map(|(a, b)| (a - b).abs()).collect();
	let sum : i32 = difference.into_iter().sum();
	println!("The sum is: {}", sum);
}

pub fn one_b(
		mut list_a : Vec<u32>,
		mut list_b : Vec<u32>
	) {

	list_a.sort();
	list_b.sort();

	let mut similarity : u32 = 0;
	let mut last : u32 = 0;

	let mut count : u32 = 0;
	for id_a in list_a {	

		if id_a == last {
			similarity += count*last;
		} else {
			count = 0;
			for (index_b, id_b) in list_b.clone().into_iter().enumerate() {
				if id_a == id_b {
					count += 1;
					similarity += id_a;
				} else if count > 1 {
					last = id_a;
					list_b.drain(0..index_b);
					break;
				}
			}
		}
	}

	println!("Similarity {}", similarity);
}