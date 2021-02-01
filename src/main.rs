use std::process;

use load_covid;

fn main() {
    if let Err(err) = load_covid::run() {
        eprintln!("This program is b0rked");
        eprintln!("Error: {}", err);

        process::exit(1);
    }
}
