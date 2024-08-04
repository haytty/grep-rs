mod cli;
mod grep;

fn main() {
    match cli::run() {
        Ok(_) => (),
        Err(err) => println!("{}", err)
    }
}
