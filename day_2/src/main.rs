use regex::Regex;
use lazy_static::lazy_static;


lazy_static! {
    static ref GAMEREGEX: Regex = Regex::new(r"Game\s+([0-9]+)").unwrap();
}
lazy_static! {
    static ref REDREGEX: Regex = Regex::new(r"([0-9]+)\s+red").unwrap();
}
lazy_static! {
    static ref BLUEREGEX: Regex = Regex::new(r"([0-9]+)\s+blue").unwrap();
}
lazy_static! {
    static ref GREENREGEX: Regex = Regex::new(r"([0-9]+)\s+green").unwrap();
}

#[derive(Debug)]
struct Game {
    number: i32,
    rounds: Vec<Round>
}

#[derive(Debug,Clone)]
struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

impl Default for Round {
    fn default() -> Round {
        Round {
            red: 0,
            green: 0,
            blue: 0 }
    }
}

fn build_round(raw: &str) -> Round {

    let red: i32 =  match REDREGEX.captures(raw.trim()) {
        Some(x) => x.get(1).unwrap().as_str().parse().unwrap(),
        None => 0
    };
    let blue: i32 =  match BLUEREGEX.captures(raw.trim()) {
        Some(x) => x.get(1).unwrap().as_str().parse().unwrap(),
        None => 0
    };
    let green: i32 =  match GREENREGEX.captures(raw.trim()) {
        Some(x) => x.get(1).unwrap().as_str().parse().unwrap(),
        None => 0
    };

    Round {
        red: red,
        blue: blue,
        green: green
    }
}

impl Game {
    pub fn is_possible(&self, red: i32, green: i32, blue: i32) -> bool {
        let mut poss_vec: Vec<bool> = Vec::new();
        for r in self.rounds.iter() {
           let r_clone = r.clone();
           poss_vec.push(r_clone.is_possible(red, green, blue));
        }
        poss_vec.into_iter().all(|x| !!x)
    }

    pub fn sum_if_possible(&self, red: i32, green: i32, blue: i32) -> i32 {
        let val: i32 = if self.is_possible(red, green, blue) {
            self.number
        } else { 0};
        val
    }

    pub fn find_minimum(&self) -> (i32,i32,i32) {
        let mut red_power: i32 = 0; 
        let mut green_power: i32 = 0; 
        let mut blue_power: i32 = 0; 

        for r in self.rounds.iter() {
            let r_clone = r.clone();
            let current_red: i32 = r_clone.red;
            red_power = if current_red > red_power {
                current_red
            } else {
                red_power
            };
            let current_green: i32 = r_clone.green;
            green_power = if current_green > green_power {
                current_green
            } else {
                green_power
            };
            let curent_blue: i32 = r_clone.blue;
            blue_power = if curent_blue > blue_power {
                curent_blue
            } else {
                blue_power
            };
         }

        (red_power,green_power,blue_power)
    }

    pub fn calculate_power(&self) -> i32 {

        let ind = self.find_minimum();
        // println!("red:{:?}, green:{:?}, blue:{:?}", ind.0, ind.1, ind.2);
        // println!("{:?}",ind.0 * ind.1 * ind.2);
        ind.0 * ind.1 * ind.2
    }

}

impl Round {
    pub fn is_possible(&self, red: i32, green: i32, blue: i32) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue 
    }

}
fn build_game(line: &str) -> Game {
    let v: Vec<&str> = line.split(':').collect();
    let number = GAMEREGEX.captures(v.get(0).unwrap()).unwrap().get(1).unwrap().as_str().parse().unwrap();
    let rounds_raw: &str = v.into_iter().nth(1).unwrap().trim();
    let rounds: Vec<&str> = rounds_raw.split(';').collect();
    let mut round_vec : Vec<Round> = Vec::new();
    for r in rounds.into_iter() {
        let current_round = build_round(r);
        round_vec.push(current_round);

    }
    Game { number: number, rounds: round_vec }

    }

fn main() {
    let string_lines = include_str!("../input.txt").lines();
    let mut games: Vec<Game> = Vec::new();
    for line in string_lines {
        let this_game: Game = build_game(line);
        games.push(this_game);
    }
    // Part 1
    let sum: i32 = games.iter_mut().map(|g| g.sum_if_possible(12, 13, 14)).sum();
    println!("The sum is {}", sum);
    // Part 2
    let power: i32 = games.iter_mut().map(|g| g.calculate_power()).sum();
    println!("The power is {}", power);


}
