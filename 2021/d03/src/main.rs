fn set_bit(num: u32, index: usize, bit: u8) -> u32 {
  // index will count from least to most sig bit
  match bit {
    0 => num,
    1 => num | (1 << index),
    _ => panic!("invalid bit: {}", bit),
  }
}

fn get_bit(num: u32, index: usize) -> u8 {
  // index will count from least to most sig bit
  ((num >> index) & 1).try_into().unwrap()
}

fn count_bits_at_index(nums: &Vec<u32>, index: usize) -> (u32, u32) {
  nums.into_iter().fold((0, 0), |(zeros, ones), num| {
    let new_bit = get_bit(*num, index);
    match new_bit {
      0 => (zeros+1, ones),
      1 => (zeros, ones+1),
      bit => panic!("invalid bit: {}", bit)
    }
  })
}

fn filter_nums_by_fn(nums: &Vec<u32>, max_bits: usize, compare: fn((u32,u32)) -> u8 ) -> u32 {
  let mut remaining = nums.to_vec();
  for i in 0..max_bits {
    // need to check most -> least significant, so invert the index
    let inv_index = (max_bits-1) - i;
    let count = count_bits_at_index(&remaining, inv_index);
    let preferred_bit = compare(count);
    remaining = remaining.into_iter().filter(|num| {
      get_bit(*num, inv_index) == preferred_bit
    }).collect();

    if remaining.len() == 0 {
      return 0;
    }

    if remaining.len() == 1 {
      return remaining[0];
    }
  }

  if remaining.len() == nums.len() {
    // we recursively called, and we didn't filter any additional
    // all of our numbers are the same
    return remaining[0]
  }

  // keep going until we have one number (or a list of the same number)
  return filter_nums_by_fn(&remaining, max_bits, compare)
}

fn pt1(nums: &Vec<u32>, max_bits: usize) -> u32 {
  let (gamma, epsilon) = (0..max_bits).fold((0,0), |(gamma, epsilon), i| {
    let count = count_bits_at_index(nums, i);
    let most_common = if count.0 > count.1 {0} else {1};

    (set_bit(gamma, i, most_common), set_bit(epsilon, i, most_common^1))
  });

  gamma * epsilon
}

fn pt2(nums: &Vec<u32>, max_bits: usize) -> u32 {
  let o2_fn = {|count|
    match count {
      (zeros, ones) if zeros > ones => 0,
      (zeros, ones) if zeros < ones => 1,
      (_, _) => 1, // prefer 1 over 0 for o2
    }
  };
  let o2 = filter_nums_by_fn(nums, max_bits, o2_fn);

  let co2_fn = {|count|
    match count {
      (zeros, ones) if zeros < ones => 0,
      (zeros, ones) if zeros > ones => 1,
      (_, _) => 0, // prefer 0 over 1 for co2
    }
  };
  let co2 = filter_nums_by_fn(nums, max_bits, co2_fn);

  o2 * co2
}

fn main() {
  let text = std::fs::read_to_string("./assets/input.txt")
    .expect("Unable to read input file");

  let mut max_bits = 0;

  let nums: Vec<u32> = text.lines().map(|str| {
    let str_len = str.len();
    max_bits = if str_len > max_bits { str_len } else { max_bits };
    u32::from_str_radix(str, 2).unwrap()
  }).collect();

  println!("pt1: {}", pt1(&nums, max_bits));
  println!("pt2: {}", pt2(&nums, max_bits));
}