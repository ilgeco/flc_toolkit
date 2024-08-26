mod elr_pilot;
mod fsm;
mod lexer;
mod parser;

use std::env::args;
use std::path::Path;
use std::process::exit;

pub use crate::elr_pilot::*;
pub use crate::lexer::*;
pub use crate::parser::*;

fn generate_pilot(path: impl AsRef<Path>) {
    let lex = Lexer::from_path(Path::new(path.as_ref()));
    let mut pars = Parser::new(lex);
    if let Some(net) = pars.parse_mnet() {
        if net.validate() {
            let pilot = create_pilot(&net);
            //println!("pilot: {pilot:?}");
            println!("{}", pilot.to_dot());
            pilot.print_conflicts();
        }
    }
}

fn main() {
    let args: Vec<_> = args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} [file]", args[0]);
        exit(1);
    }
    generate_pilot(&args[1]);
}

#[cfg(test)]
mod test {
    use crate::generate_pilot;

    #[test]
    fn test_generate_pilot() {
        generate_pilot(r"./tests/cursed.txt");
        generate_pilot(r"./tests/dangling_else.txt");
        generate_pilot(r"./tests/elr_mnet_2013-02-05.txt");
        generate_pilot(r"./tests/elr_mnet_2020-01-14.txt");
        generate_pilot(r"./tests/elr_mnet_2024-02-13.txt");
        generate_pilot(r"./tests/elr_mnet_2024-06-13.txt");
        generate_pilot(r"./tests/elr_mnet_2024-07-04.txt");
        generate_pilot(r"./tests/elr_mnet_book-4.15.txt");
        generate_pilot(r"./tests/elr_mnet_book-4.16.txt");
    }
}
