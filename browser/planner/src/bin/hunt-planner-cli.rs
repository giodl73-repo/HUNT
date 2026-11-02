use std::io::{self, Read};
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("UTF-8 stdin");
    println!("{}", hunt_planner::forecast_request(&input));
}
