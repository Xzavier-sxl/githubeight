#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("outer");

        'inner: loop {
            println!("inner");

           
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}
