#[macro_use]
use log::{info, warn};
use std::io::{self, Write};

fn main() {
    println!("Hello, world!");

    let x = 42;
    println!("My lucky number is {}.", x);

    // with debug repr {:?} because a vector is not printable
    let xs = vec![1, 2, 3];
    println!("The list is: {:?}", xs);

    println!("This is information"); // will write to stdout stream
    eprintln!("This is an error! :("); // will write to stderr stream

    // Printing to the terminal is surprisingly slow! If you call things like println! in a loop,
    // it can easily become a bottleneck in an otherwise fast program.
    // To speed this up, there’s two things you can do
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
    writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here

    let stdout2 = io::stdout(); // get the global stdout entity
    let mut handle = stdout2.lock(); // acquire a lock on it. This prevents the system from locking and unlocking stdout over and over again.
    writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here

    // Showing a progress bar
    let pb = indicatif::ProgressBar::new(5);
    for i in 0..5 {
        //do_hard_work();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");

    // Logingenv_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");
}
