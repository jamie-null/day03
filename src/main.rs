use std::error::Error;
use std::fs::File;
use std::io::{self,BufRead};

 fn main() -> Result<(),Box<dyn Error>>{
    let input_handle = File::open("./input.txt")?;
     let lines = io::BufReader::new(input_handle).lines();
     let mut map  = Vec::new();
     for line in lines {
         let line: String = line.unwrap();
         map.push(line.chars().map(|x| x == '#').collect::<Vec<bool>>());
     }
     println!("{:#?}",map);
     let mut product: u64 = 1;
     for (right,down) in vec![(1,1),(3,1),(5,1),(7,1),(1,2)] {
         let mut trees = 0;
         for n in 0..map.len() {
             if down * n < map.len() && map[n * down][(n*right) % map[0].len()] {
                 trees += 1
             }
         }
         println!("{}",trees);
         product *= trees;
     }
     println!("{}",product);
     return Ok(());
}
