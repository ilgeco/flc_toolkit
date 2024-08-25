mod elr_pilot;
mod fsm;
mod lexer;
mod parser;

use std::path::Path;

pub use crate::elr_pilot::*;
pub use crate::lexer::*;
pub use crate::parser::*;

fn main() {
    let lex = Lexer::from_path(Path::new("test_file.txt"));
    let mut pars = Parser::new(lex);
    if let Some(net) = pars.parse_mnet() {
        println!("{net:?}");
        let pilot = create_pilot(&net);
        //println!("pilot: {pilot:?}");
        println!("{}", pilot.to_dot());
        pilot.print_conflicts();
    }
}
