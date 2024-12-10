#[derive(Debug, Clone, Copy)]
enum Block {
    File(usize, usize),
    Free(usize),
}

#[allow(dead_code)]
fn debug(blocks: &Vec<Block>) {
    blocks.iter().for_each(|b| match b {
        Block::File(id, size) => print!("{}", id.to_string().repeat(*size)),
        Block::Free(size) => print!("{}", ".".repeat(*size)),
    });
    println!();
}

fn parse(input: &str, pt2: bool) -> Result<Vec<Block>, Box<dyn std::error::Error>> {
    let mut blocks = Vec::new();
    for (i, c) in input.chars().enumerate() {
        let n: usize = c.to_string().parse::<usize>()?;
        let blk = if i % 2 == 0 {
            let id = i / 2;
            Block::File(id, if pt2 { n } else { 1 })
        } else {
            Block::Free(if pt2 { n } else { 1 })
        };

        if pt2 {
            blocks.push(blk);
        } else {
            blocks.extend(std::iter::repeat(blk).take(n));
        }
    }
    Ok(blocks)
}

fn solve(blocks: &Vec<Block>, pt2: bool) -> usize {
    let mut blocks = blocks.clone();
    let mut ptr = 0;
    for i in (0..blocks.len()).rev() {
        if pt2 {
            // reset the pointer to the beginning of the blocks for part 2 for full O(n) scans
            ptr = 0;
        }
        match blocks[i] {
            Block::File(_, b_size) => loop {
                if ptr > i {
                    break;
                }

                if let Block::Free(f_size) = blocks[ptr] {
                    if f_size == b_size {
                        blocks.swap(i, ptr);
                        break;
                    } else if f_size > b_size {
                        blocks[ptr] = blocks[i];
                        // an unfortunate O(n) insert, but i don't wanna refactor for a linked list
                        blocks.insert(ptr + 1, Block::Free(f_size - b_size));
                        blocks[i + 1] = Block::Free(b_size);
                        break;
                    }
                }

                ptr += 1;
            },
            Block::Free(_) => {
                continue;
            }
        }
    }

    let mut checksum = 0;
    let mut i = 0;
    for block in blocks.iter() {
        match block {
            Block::File(id, size) => {
                for _ in 0..*size {
                    checksum += i * id;
                    i += 1;
                }
            }
            Block::Free(size) => {
                i += size;
            }
        }
    }

    checksum
}

fn pt1(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let blocks = parse(input, false)?;
    Ok(solve(&blocks, false))
}

fn pt2(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let blocks = parse(input, true)?;
    Ok(solve(&blocks, true))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("../assets/input.txt");

    println!("pt1: {}", pt1(&input)?);
    println!("pt2: {}", pt2(&input)?);

    Ok(())
}
