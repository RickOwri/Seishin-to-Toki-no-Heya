fn fizz_buzz() -> i32 {
    let mut fizz_buzz_count = 0;

    for i in 1..=301 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizz buzz");
            fizz_buzz_count += 1
        } else if i % 3 == 0 {
            println!("fizz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", i);
        }
    }

    fizz_buzz_count
}

fn main() {
    println!("Welcome to the fizz buzz program !");

    let count = fizz_buzz();
    
    println!("Number of times 'fizz buzz' occured: {}", count);
}

