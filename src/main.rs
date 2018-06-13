//use std::io;
extern crate euler;
extern crate time;

use euler::problems::problem_083;
use time::PreciseTime;

use std::fs::{/*File, */self};
use std::env;
use std::path::Path;

use problem::Problem;
use solution::Solution;

mod problem;
mod solution;

fn main() {
    let start = PreciseTime::now();

    println!("E83: {}", problem_083::main());

    let end = PreciseTime::now();

    let duration = start.to(end);
    let seconds = duration.num_seconds();
    let milliseconds = duration.num_milliseconds();
    let microseconds = duration.num_microseconds().unwrap();
    let nanoseconds = duration.num_nanoseconds().unwrap();

    if seconds < 1 as i64 {
        if milliseconds < 1 as i64 {
            if microseconds < 1 as i64 {
                println!("Time: {} nanoseconds", nanoseconds);
            } else {
                println!("Time: {} microseconds", microseconds);
            }
        } else {
            println!("Time: {} milliseconds", milliseconds);
        }
    } else {
        println!("Time: {} seconds", seconds);
    }

    // let mut number = String::new();

    // io::stdin().read_line(&mut number)
    // 	.expect("Failed to read line");

    // println!("You entered {}", number);
}

//#![warn(bad_style, unused, unused_extern_crates, unused_import_braces, unused_qualifications,
//unused_results)]
////extern crate euler;
//
////use euler::problems::problem_001;
//#[macro_use]
//extern crate log;
//extern crate simple_logger;
//
//use std::fs::{/*File, */self};
//use std::env;
//use std::path::Path;
//
//use problem::Problem;
//use solution::Solution;
//
//mod problem;
//mod solution;
//
//fn main() {
//  simple_logger::init().unwrap();
//
//  for problem in fs::read_dir(&Path::new("src/problems")).unwrap().filter_map(|dir| {
//    Problem::new(dir.unwrap().path())
//  }) {
//    let solution = Solution::new(&problem);
//
//    match Executable::new(&solution) {
//      Some(executable) => if executable.validate() == Some(true) {
//        executable.bench();
//        processed += 1;
//      },
//      None => {},
//    }
//
////    info!("{:?}", problem);
//  }
//
//  let /*mut*/ path = env::current_exe();
////  let _ = writeln!(&mut io::stdout(), "{:?}", path);
//  match path {
//    Ok(ref p) => info!("{:?}", p),
//    Err(e) => error!("{:?}", e),
//  }
//
////    println!("E1: {}", problem_001::main());
//
//  // let mut number = String::new();
//
//  // io::stdin().read_line(&mut number)
//  // 	.expect("Failed to read line");
//
//  // println!("You entered {}", number);
//}
