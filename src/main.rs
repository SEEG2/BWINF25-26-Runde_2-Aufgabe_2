use crate::approach1::solve;
use crate::util::interpret_input;
use chrono::{DateTime, Local};
use std::time::SystemTime;

mod util;
mod approach1;

fn main() {
    let interpreted_input = interpret_input("175
13
1 68 753
2 104 743
3 95 704
4 67 719
5 110 769
6 124 713
7 154 678
8 169 696
9 156 728
10 113 675
11 80 677
12 142 754
13 53 684
");
    let now: DateTime<Local> = SystemTime::now().into();
    println!("Start Time: {}", now.format("%H:%M:%S"));

    solve(interpreted_input.0, &interpreted_input.1);

    let now2: DateTime<Local> = SystemTime::now().into();
    println!("End Time: {}", now2.format("%H:%M:%S"));
}