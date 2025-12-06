use std::collections::BTreeMap;

pub struct Inventory {
    ranges: BTreeMap<u64, u64>,
    ids: Vec<u64>,
}

impl Inventory {
    pub fn pt1(&self) -> usize {
        self.ids.iter().filter(|id| self.is_fresh(**id)).count()
    }

    pub fn pt2(&self) -> u64 {
        self.ranges
            .iter()
            .map(|(&start, &end)| end - start + 1)
            .sum()
    }

    pub fn is_fresh(&self, id: u64) -> bool {
        for (&start, &end) in self.ranges.iter() {
            if id >= start && id <= end {
                return true;
            }
        }
        false
    }
}

impl std::fmt::Debug for Inventory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Inventory")
            .field("ranges", &self.ranges)
            .field("ids", &self.ids)
            .finish()
    }
}

impl TryFrom<&str> for Inventory {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut raw_ranges: Vec<(u64, u64)> = Vec::new();
        let mut ids = Vec::new();

        for line in input.lines() {
            match line {
                "" => continue,
                line if line.contains("-") => {
                    let (start, end) = line.split_once("-").ok_or("invalid range")?;
                    let start: u64 = start.parse().map_err(|_| "invalid start")?;
                    let end: u64 = end.parse().map_err(|_| "invalid end")?;
                    raw_ranges.push((start, end));
                }
                line => ids.push(line.parse().map_err(|_| "invalid id")?),
            }
        }

        raw_ranges.sort_by_key(|r| r.0);
        let mut ranges = BTreeMap::new();
        for (start, end) in raw_ranges {
            if ranges.is_empty() {
                ranges.insert(start, end);
            } else {
                let (&last_start, &last_end) = ranges.iter().next_back().unwrap();

                if start <= last_end + 1 {
                    ranges.insert(last_start, last_end.max(end));
                } else {
                    ranges.insert(start, end);
                }
            }
        }

        Ok(Self { ranges, ids })
    }
}

fn main() {
    let input = include_str!("../assets/input.txt");
    let inv = Inventory::try_from(input).unwrap();
    println!("pt1: {:?}", inv.pt1());
    println!("pt2: {:?}", inv.pt2());
}
