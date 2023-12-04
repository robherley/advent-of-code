#[derive(Debug)]
struct Game {
    id: i32,
    rounds: Vec<Cubes>,
}

impl Game {
    pub fn from_str(input: &str) -> Game {
        let (game, rounds) = input.split_once(": ").unwrap();
        Game {
            id: game.split_once(" ").unwrap().1.parse::<i32>().unwrap(),
            rounds: rounds
                .split("; ")
                .map(|raw_cubes| Cubes::from_str(raw_cubes))
                .collect(),
        }
    }

    pub fn power(&self) -> i32 {
        self.maximum().power()
    }

    pub fn maximum(&self) -> Cubes {
        let mut max = Cubes { r: 0, g: 0, b: 0 };
        self.rounds.iter().for_each(|round| {
            max.r = max.r.max(round.r);
            max.g = max.g.max(round.g);
            max.b = max.b.max(round.b);
        });
        max
    }

    pub fn is_possible(&self, cubes: &Cubes) -> bool {
        !self.rounds.iter().any(|round| !round.is_possible(cubes))
    }
}

#[derive(Debug)]
struct Cubes {
    r: i32,
    g: i32,
    b: i32,
}

impl Cubes {
    pub fn from_str(input: &str) -> Cubes {
        let mut cubes = Cubes { r: 0, g: 0, b: 0 };
        input.split(", ").for_each(|cube| {
            let (amount, color) = cube.split_once(" ").unwrap();
            let amount = amount.parse::<i32>().unwrap();
            match color {
                "red" => cubes.r += amount,
                "green" => cubes.g += amount,
                "blue" => cubes.b += amount,
                _ => panic!("invalid color"),
            }
        });
        cubes
    }

    pub fn is_possible(&self, max: &Cubes) -> bool {
        self.r <= max.r && self.g <= max.g && self.b <= max.b
    }

    pub fn power(&self) -> i32 {
        let mut power = 1;

        if self.r > 0 {
            power *= self.r;
        }
        if self.g > 0 {
            power *= self.g;
        }
        if self.b > 0 {
            power *= self.b;
        }

        power
    }
}

fn pt1(games: &Vec<Game>, cubes: &Cubes) -> i32 {
    games
        .into_iter()
        .filter(|game| game.is_possible(cubes))
        .map(|game| game.id)
        .sum()
}

fn pt2(games: &Vec<Game>) -> i32 {
    games.into_iter().map(|game| game.power()).sum()
}

fn main() {
    let input = include_str!("../assets/input.txt");
    let games: Vec<Game> = input.lines().map(|line| Game::from_str(line)).collect();
    println!(
        "pt1: {}",
        pt1(
            &games,
            &Cubes {
                r: 12,
                g: 13,
                b: 14,
            }
        )
    );
    println!("pt2: {}", pt2(&games));
}
