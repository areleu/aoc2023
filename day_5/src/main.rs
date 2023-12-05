use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(r"\d+").unwrap();
}

fn main() {
    let mut chunks = include_str!("../example.txt").split("\r\n\r\n");
    let seeds: Vec<i32> = chunks.next().unwrap().split(":")
                            .collect::<Vec<&str>>().into_iter()
                            .nth(1).unwrap().trim().split(" ")
                            .collect::<Vec<_>>()
                            .into_iter().map(|x| x.parse().unwrap()).collect() ;
    println!("{:?}", seeds);
    for chunk in chunks {
        println!("{:?}", chunk)
    }
    

}
