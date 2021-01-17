fn main() {
    let mut v : Vec<f64> = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", v);
}