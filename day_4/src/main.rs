use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::collections::HashMap;

lazy_static! {
    static ref RE: Regex = Regex::new(r"\d+").unwrap();
}

fn main() {
    let string_lines = include_str!("../input.txt")
                                    .lines() ;
    let mut scores: Vec<usize> = Vec::new();
    let mut instances: HashMap<usize, i32> = HashMap::new();
    for (i,line) in string_lines.enumerate() {
        let current_card: usize = i + 1;
        *instances.entry(current_card).or_insert(0) +=1; 
        let v: Vec<&str> = line.split(':').collect();
        let numbers_raw: &str = v.into_iter().nth(1).unwrap().trim();
        let separated: Vec<&str> = numbers_raw.split('|').collect();
        let i_have: Vec<i32> =  RE.find_iter(separated.clone().into_iter().nth(0).unwrap().trim())
                                  .filter_map(|digits| digits.as_str().parse().ok())
                                  .collect() ;
        let winners: Vec<i32>  =  RE.find_iter(separated.into_iter().nth(1).unwrap().trim())
                                  .filter_map(|digits| digits.as_str().parse().ok())
                                  .collect() ;
        let winner_set: HashSet<_> = winners.iter().copied().collect();
        let matching: Vec<&i32> = i_have.iter().filter(|item| winner_set.contains(item)).collect();
        let matches: usize = matching.len().try_into().unwrap();
        for n in current_card+1..current_card+matches+1 {
            let added: i32 = *instances.get(&current_card).unwrap_or(&0);
            *instances.entry(n).or_insert(0) += added; 
        }
        // let val = map.entry((i, start-1)).or_insert(vec![]);
        // val.extend(vec!(String::from(this_string)));
        if matches != 0 {
            let score: usize = 2_usize.pow((matches - 1).try_into().unwrap()).try_into().unwrap();
            scores.push(score)
            }
        
    }
    let mut sum: i32 = 0;
    for (key, value) in instances.into_iter() {
        sum += value;
    }
    println!("the total sum is: {}", sum);
}
