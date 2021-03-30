mod cocktail;

fn main() {
    let mut input = [5, 4, 2, 6, 7, 1, 12, 10, 13];

    println!("PRE COCKTAIL SORT: {:?}", input);
    cocktail::run(&mut input);
    println!("POST COCKTAIL SORT: {:?}", input);
}
