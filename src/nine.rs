
use itertools::Itertools;
trait Decompress {
    fn decompress(&self) -> Vec<Option<u64>>;
}

impl Decompress for str {
    fn decompress(&self) -> Vec<Option<u64>> {
        self.chars().enumerate().fold(Vec::new(), |mut output, (index, ch)| {
            if let Some(digit) = ch.to_digit(10) {
                if index % 2 == 0 {
                    output.extend(std::iter::repeat(Some((index / 2) as u64)).take(digit as usize));
                } else {
                    output.extend(std::iter::repeat(None).take(digit as usize));
                }
            }
            output
        })
    }
}

trait CompressFrag {
	fn compress_frag(self) -> Vec<Option<u64>>;
	fn compress_defrag(self) -> Vec<Option<u64>>;
}

impl CompressFrag for Vec<Option<u64>> {
	fn compress_frag(self) -> Vec<Option<u64>> {
		let mut output = self.into_iter();
		let mut compressed: Vec<Option<u64>> = Vec::new();
		while let Some(front) = output.next() {
			match front {
				Some(value) => compressed.push(Some(value)),
				None => {
					while let Some(back) = output.next_back() {
						compressed.push(back);
						break;
					}
				}
			}
		}	
		compressed
	}

	fn compress_defrag(self) -> Vec<Option<u64>> {
		let mut chunked: Vec<Vec<Option<u64>>> = self
			.into_iter()
			.chunk_by(|elt| *elt)
			.into_iter()
			.map(|(_, group)| group.collect())
			.collect();
	
		let mut i = 0;
	
		while i < chunked.len() {
			if chunked[i][0].is_none() {
				let free_space_len = chunked[i].len();
				let mut j = chunked.len() - 1;
				let mut file_moved = false;
	
				while i < j {
					if chunked[j][0].is_some() {
						let file_len = chunked[j].len();
	
						if file_len <= free_space_len {
							chunked[i] = std::mem::take(&mut chunked[j]);	
							chunked[j] = vec![None; file_len];
	
							if j > 0 && chunked[j - 1][0].is_none() {
								let left_chunk = chunked.swap_remove(j);
								chunked[j - 1].extend(left_chunk);
								j -= 1;
							}
	
							if j + 1 < chunked.len() && chunked[j + 1][0].is_none() {
								let right_chunk = chunked.swap_remove(j + 1);
								chunked[j].extend(right_chunk);
							}
	
							file_moved = true;
							break;
						}
					}
					j -= 1;
				}
	
				if !file_moved {
					i += 1;
				}
			} else {
				i += 1;
			}
		}
		
		chunked.into_iter().flatten().collect()
	}
}

trait CheckSum {
	fn checksum(&self) -> u64;
}

impl CheckSum for Vec<Option<u64>> {
	fn checksum(&self) -> u64 {
		self.into_iter().enumerate().fold(0, |result, (index, num)| {
			let mut add = 0;
			if let Some(num) = num {
				add = index as u64 * num;
			}
			result + add
		})
	}
}

pub fn nine_a() {

	let input = include_str!("../res/09_input.txt").to_string();
	let compressed = input.decompress().compress_frag();
	println!("{}", compressed.checksum());
}

pub fn nine_b() {

	let input = include_str!("../res/09_input.txt").to_string();

	let decompressed: Vec<Option<u64>> = input.decompress().compress_defrag();

	println!("{}", decompressed.checksum());
}