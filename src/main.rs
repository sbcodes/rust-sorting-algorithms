use std::io::Read;
use std::fs;
use std::string;

mod cocktail;

fn main() {
    let filename: String = "input.txt".to_string();
    let input = read_file(filename);
    println!("PRE COCKTAIL SORT: {:?}", input);
    let output = cocktail::run(input);
    println!("POST COCKTAIL SORT: {:?}", output);
}

fn read_file(filename: String) -> std::vec::Vec<i32> {
    let mut file = fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut content_vec = Vec::new();
    for c in contents.split(","){
        content_vec.push(c.trim().parse::<i32>().unwrap())
    }
    content_vec
}
