use atty::Stream; // is this a tty?
use serde_json::json;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// Output JSON instead of human readable messages
    #[structopt(long = "json")]
    json: bool,
}

fn main() {
    // if it's a tty or not
    if atty::is(Stream::Stdout) {
        println!("I'm a terminal");
    } else {
        println!("I'm not");
    }

    // if arg is json prints in json formats, else human readable
    // example human readeabl : cargo run -q
    // example json format: cargo run -q -- --json
    let args = Cli::from_args();
    if args.json {
        println!(
            "{}",
            json!({
                "type": "message",
                "content": "Hello world",
            })
        );
    } else {
        println!("Hello world");
    }
}
