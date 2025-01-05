use cli::UserInterface;

mod cemu;
mod cli;
mod lesson;
mod parser;
mod tools;

fn main() {
    let mut cli = UserInterface::new().unwrap();

    cli.run()
}
