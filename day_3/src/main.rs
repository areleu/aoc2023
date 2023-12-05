use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;


lazy_static! {
    static ref PARTREGEX: Regex = Regex::new(r"([0-9]+)").unwrap();
}

lazy_static! {
        static ref BOTHREGEX: Regex = Regex::new(r"\*([0-9]+)\*").unwrap();
    }
lazy_static! {
    static ref LEFTREGEX: Regex = Regex::new(r"\*([0-9]+)\.").unwrap();
}
lazy_static! {
    static ref RIGHTREGEX: Regex = Regex::new(r"\.([0-9]+)\*").unwrap();
}

lazy_static! {
    static ref TOPBOTREGEX: Regex = Regex::new(r"\*").unwrap();
}

#[derive(Debug)]
struct Caps {
    string: String,
    start: usize,
    end: usize
}

impl Default for Caps {
    fn default() ->  Caps{
        Caps {
            string: "".to_string(),
            start: 0,
            end: 0 }
    }
}

fn main() {
    let string_lines = include_str!("../input.txt")
                                    .lines() ;
    let width = string_lines.clone().next().unwrap().chars().count();
    let filler = ".".repeat(width + 2);
    // let bottom = filler.as_str();
    let mut map: HashMap<(usize,usize), Vec<String>> = HashMap::new();
    let mut part_numbers: Vec<i32> = Vec::new();
    let mut previous_numbers: Vec<Caps> = Vec::new();
    let mut top: String = filler.clone();
    for (i,line) in string_lines.clone().enumerate() {
        while !previous_numbers.is_empty() {
            let inner_buffered = ".".to_owned()  + line + "." ;
            let current_num = previous_numbers.pop().unwrap();
            match TOPBOTREGEX.find(&inner_buffered[current_num.start-1..current_num.end+1]){
                Some(mch) => {let val = map.entry((i, current_num.start-1+mch.start())).or_insert(vec![]);
                            val.extend(vec!(String::from(current_num.string.clone())));
                            part_numbers.push(current_num.string.parse::<i32>().unwrap())},
                None => ()
            }
        }
        let buffered = ".".to_owned()  + line + "." ;
        let caps = PARTREGEX.captures_iter(&buffered);
        for cap in caps {
            let cap_info = cap.get(1).unwrap();
            let start = cap_info.start() as usize;
            let end = cap_info.end() as usize;
            let this_string = cap_info.as_str();
            match TOPBOTREGEX.find(&top[start-1..end+1]){
                Some(mch) => {let val = map.entry((i-1, start-1 + mch.start())).or_insert(vec![]);
                                         val.extend(vec!(String::from(this_string)));
                                        part_numbers.push(this_string.parse::<i32>().unwrap())},
                None => match BOTHREGEX.find(&buffered[start-1..end+1]) {
                    Some(_) => {let val = map.entry((i, start-1)).or_insert(vec![]);
                                             val.extend(vec!(String::from(this_string)));
                                             let val = map.entry((i, end)).or_insert(vec![]);
                                             val.extend(vec!(String::from(this_string)));
                                             part_numbers.push(this_string.parse::<i32>().unwrap())},
                    None => match LEFTREGEX.find(&buffered[start-1..end+1]) {
                        Some(_)=> { let val = map.entry((i, start -1)).or_insert(vec![]);
                                    val.extend(vec!(String::from(this_string)));
                                    part_numbers.push(this_string.parse::<i32>().unwrap())},
                        None => match RIGHTREGEX.find(&buffered[start-1..end+1]) { 
                            Some(_) => {let val = map.entry((i, end)).or_insert(vec![]);
                                                     val.extend(vec!(String::from(this_string)));
                                                     part_numbers.push(this_string.parse::<i32>().unwrap())},
                            None => previous_numbers.push(Caps{string: String::from(this_string), end: end, start:start})}
                }
            }}
        }
        top = buffered.to_string();
        // let caps: Vec<i32> = PARTREGEX.captures(&buffered).filter_map(|digits| digits.as_str().parse().ok()).collect();
        

        

    }
    let mut final_sum: i32 = 0;
    for v in map.values().filter(|x| x.len() == 2) {
        let mut pre:i32 = 1;
        for w in v.iter().map(|y| y.parse().unwrap_or(1)).collect::<Vec<_>>().iter() {
            pre *= w
        }
        final_sum += pre;
        
    }
    println!("{:?}", map);
    println!("Final sum: {:?}",final_sum);
}
