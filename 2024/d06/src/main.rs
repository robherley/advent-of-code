use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entity {
    Obstacle,
    Guard,
    Seen,
    Empty,
}

impl TryFrom<char> for Entity {
    type Error = String;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '#' => Ok(Entity::Obstacle),
            '^' => Ok(Entity::Guard),
            '.' => Ok(Entity::Empty),
            'X' => Ok(Entity::Seen),
            _ => Err(format!("unexpected character: {}", ch)),
        }
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ch = match self {
            Entity::Obstacle => '#',
            Entity::Guard => '^',
            Entity::Seen => 'X',
            Entity::Empty => '.',
        };
        write!(f, "{}", ch)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Clone)]
struct Map {
    grid: Vec<Vec<Entity>>,
    guard: (isize, isize),
    start: (isize, isize),
    direction: Direction,
    is_infinite: bool,
    breadcrumbs: HashSet<(isize, isize, Direction)>,
}

impl Map {
    fn set(&mut self, pos: (isize, isize), entity: Entity) {
        self.grid[pos.0 as usize][pos.1 as usize] = entity;
    }

    fn get(&self, pos: (isize, isize)) -> Option<&Entity> {
        if pos.0.is_negative() || pos.1.is_negative() {
            return None;
        }

        self.grid.get(pos.0 as usize)?.get(pos.1 as usize)
    }

    fn seen(&self) -> Vec<(isize, isize)> {
        let mut seen = vec![self.start];

        for (x, row) in self.grid.iter().enumerate() {
            for (y, entity) in row.iter().enumerate() {
                if *entity == Entity::Seen {
                    seen.push((x as isize, y as isize));
                }
            }
        }

        seen
    }

    fn solve(&mut self) {
        loop {
            let inserted = self
                .breadcrumbs
                .insert((self.guard.0, self.guard.1, self.direction));
            if !inserted {
                self.is_infinite = true;
                break;
            }
            if !&self.next() {
                break;
            }
        }
    }

    fn next(&mut self) -> bool {
        let next = (
            self.guard.0 + self.direction.delta().0,
            self.guard.1 + self.direction.delta().1,
        );

        match self.get(next) {
            Some(Entity::Obstacle) => {
                self.direction = self.direction.turn();
                return true;
            }
            Some(Entity::Empty) | Some(Entity::Seen) => {
                self.set(self.guard, Entity::Seen);
                self.guard = next;
                return true;
            }
            None => {
                self.guard = next;
                return false;
            }
            _ => panic!("unexpected entity"),
        }
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[s: {:?}, g: {:?}, d: {:?}]",
            self.start, self.guard, self.direction
        )?;
        for row in &self.grid {
            for entity in row {
                write!(f, "{}", entity)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl TryFrom<&str> for Map {
    type Error = &'static str;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut start: Option<(isize, isize)> = None;

        let grid: Result<Vec<Vec<Entity>>, _> = input
            .lines()
            .enumerate()
            .map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .map(|(y, ch)| match ch {
                        '#' => Ok(Entity::Obstacle),
                        '^' => {
                            if start.is_some() {
                                return Err("multiple guards found");
                            }
                            start = Some((x as isize, y as isize));
                            Ok(Entity::Guard)
                        }
                        '.' => Ok(Entity::Empty),
                        _ => Err("unexpected character"),
                    })
                    .collect::<Result<Vec<Entity>, _>>()
            })
            .collect();

        let guard = start.ok_or("no guard found")?;

        Ok(Map {
            grid: grid?,
            guard,
            start: guard,
            direction: Direction::Up,
            is_infinite: false,
            breadcrumbs: HashSet::new(),
        })
    }
}

fn pt1(mut map: Map) -> usize {
    map.solve();
    map.seen().len()
}

// this is garbo
fn pt2(template: Map) -> usize {
    let mut inifinite = Vec::new();
    for (x, row) in template.grid.iter().enumerate() {
        for (y, entity) in row.iter().enumerate() {
            let pos = (x as isize, y as isize);
            if pos == template.start {
                continue;
            }

            match entity {
                Entity::Obstacle | Entity::Guard => continue,
                Entity::Empty | Entity::Seen => {
                    let mut candidate = template.clone();
                    candidate.set(pos, Entity::Obstacle);
                    candidate.solve();
                    if candidate.is_infinite {
                        inifinite.push(candidate);
                    }
                }
            }
        }
    }

    inifinite.len()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map = Map::try_from(include_str!("../assets/input.txt"))?;

    println!("pt1: {}", pt1(map.clone()));
    println!("pt2: {}", pt2(map.clone()));

    Ok(())
}
