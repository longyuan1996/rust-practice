use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (x, y) = rng.gen();
        Point { x, y }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let rand_tuple : (i32, bool, f64)= rng.gen();
    let rand_point : Point= rng.gen();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);
}
