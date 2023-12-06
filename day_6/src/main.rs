use regex::Regex;
use lazy_static::lazy_static;
use std::cmp;

lazy_static! {
    static ref RE: Regex = Regex::new(r"\d+").unwrap();
}

pub fn run(hold: i32, time: i32) -> i32 {
    let distance = cmp::max((time - hold) * hold,0);
    distance
}

pub fn bigrun(hold: i64, time: i64) -> i64 {
    let distance = cmp::max((time - hold) * hold,0);
    distance
}

fn main() {
    let mut lines = include_str!("../input.txt").lines();
    let times: Vec<i32>  = RE.find_iter(lines.next().unwrap().split(":")
                             .collect::<Vec<&str>>().into_iter()
                             .nth(1).unwrap().trim())
                             .filter_map(|digits| digits.as_str()
                             .parse().ok()).collect();
    let distances: Vec<i32> = RE.find_iter(lines.next().unwrap().split(":")
                            .collect::<Vec<&str>>().into_iter()
                            .nth(1).unwrap().trim())
                            .filter_map(|digits| digits.as_str()
                            .parse().ok()).collect();
    let bigtime: i64 = times.iter().fold("".to_string(), |acc, x| acc + &x.to_string()).parse::<i64>().unwrap();
    let bigdistance: i64 = distances.iter().fold("".to_string(), |acc, x| acc + &x.to_string()).parse::<i64>().unwrap();
    let races = times.iter().zip(distances.iter());
    let mut counts: Vec<i32> = Vec::new();
    for (race, (time, distance)) in races.enumerate() {
        let mut ways: Vec<i32> = Vec::new();
        for hold in 0..*time+1 {
            let my_distance = run(hold, *time);
            if my_distance > *distance {
                ways.push(my_distance)
            }
           
        }
        let count: i32 = ways.len().try_into().unwrap();
        counts.push(count);
        println!("Race: {}, time: {}, distance: {}, counts: {:?}", race, time, distance, count)

    }
    let mut i = 1;
    for c in counts {
        i *= c;
        
    } 
    println!("{}",i);

    // part b
    let mut new_ways: Vec<i64> = Vec::new();
    for hold in 0..bigtime+1 {
        let my_distance = bigrun(hold, bigtime);
        if my_distance > bigdistance {
            new_ways.push(my_distance.into())
        }
       
    }
    let count_on_me: i64 = new_ways.len().try_into().unwrap();
    println!("{:?}", count_on_me)
}
