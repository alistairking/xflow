use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // TODO
}

fn main() {
    let args = Args::parse();

    println!("Hello, world! {:?}", args);
}
