fn main() {
    let mut beers_on_the_wall = 99u32;
    while beers_on_the_wall > 2 {
        println!("{beers_on_the_wall} bottles of beer on the wall,");
        println!("{beers_on_the_wall} bottles of beer.");
        println!("Take one down, pass it around,");
        beers_on_the_wall -= 1;
        println!("{beers_on_the_wall} bottles of beer on the wall.\n");
    }

    println!("2 bottles of beer on the wall,");
    println!("2 bottles of beer.");
    println!("Take one down, pass it around,");
    println!("1 bottle of beer on the wall.\n");

    println!("1 bottle of beer on the wall,");
    println!("1 bottle of beer.");
    println!("Take one down, pass it around,");
    println!("No bottles of beer on the wall.");
}
