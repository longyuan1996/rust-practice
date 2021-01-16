use rand::Rng;
use rand::distributions::{Distribution, Uniform};

fn run_random() {
    let mut rng = rand::thread_rng();
    let n1 : u8 = rng.gen();
    let n2 : u16 = rng.gen();
    // let n3 = rng.gen();
    
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}

fn run_range_random_1() {
    let mut rng = rand::thread_rng();
    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0 .. 10.0));
}

fn run_range_random_2() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}

fn main() {
    run_random();
    println!("\n");

    run_range_random_1();
    println!("\n");

    run_range_random_2();
    println!("\n");
}
