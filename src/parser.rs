use std::mem::replace;

use crate::lexer::*;
use crate::elr_pilot::*;

pub struct Parser {
    lexer: Lexer,
    lookahead: Option<Token>
}

macro_rules! expect {
    ($self:expr, $p:pat_param) => (
        {
            let tok = $self.advance();
            let Some(Token{value:$p}) = tok else {
                println!("syntax error");
                return None;
            };
            tok.unwrap()
        }
    );
    ($self:expr, $p:pat_param, $b:block) => (
        if let Some(Token{value:$p}) = $self.advance() $b else {
            println!("syntax error");
            return None;
        }
    );
}

macro_rules! accept {
    ($self:expr, $p:pat_param) => (
        if let Some(Token{value:$p}) = $self.lookahead {
            $self.advance()
        } else {
            None
        }
    );
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let lookahead = lexer.next();
        println!("{lookahead:?}");
        Parser{lexer, lookahead}
    }

    fn advance(&mut self) -> Option<Token> {
        if let Some(_) = self.lookahead {
            let new = self.lexer.next();
            println!("{new:?}");
            replace(&mut self.lookahead, new)
        } else {
            None
        }
    }

    fn parse_state(&mut self) -> Option<State> {
        expect!(self, TokenValue::KwState);
        let id = expect!(self, TokenValue::Number(num), { num });
        let mut state = State{id, transitions:vec![], is_initial:false, is_final:false};
        expect!(self, TokenValue::LBrace);
        loop {
            if let Some(_) = accept!(self, TokenValue::KwInitial) {
                expect!(self, TokenValue::Semi);
                state.is_initial = true;
            } else if let Some(_) = accept!(self, TokenValue::KwFinal) {
                expect!(self, TokenValue::Semi);
                state.is_final = true;
            } else if let Some(Token{value:TokenValue::Ident(character)}) = self.lookahead {
                self.advance();
                expect!(self, TokenValue::RArrow);
                expect!(self, TokenValue::Number(dest_id), {
                    let trans = Transition{character, dest_id};
                    state.transitions.push(trans);
                });
                expect!(self, TokenValue::Semi);
            } else {
                break;
            }
        }
        expect!(self, TokenValue::RBrace);
        Some(state)
    }

    fn parse_machine(&mut self) -> Option<Machine> {
        expect!(self, TokenValue::KwMachine);
        let name = expect!(self, TokenValue::Ident(name), {
            if !name.is_ascii_uppercase() {
                println!("machine name must be ASCII uppercase");
            }
            name
        });
        let mut machine = Machine{name, states: vec![]};
        expect!(self, TokenValue::LBrace);
        while let Some(Token{value:TokenValue::KwState}) = self.lookahead {
            if let Some(state) = self.parse_state() {
                machine.states.push(state);
            } else {
                return None;
            }
        }
        expect!(self, TokenValue::RBrace);
        Some(machine)
    }

    pub fn parse_mnet(&mut self) -> Option<MachineNet> {
        let mut machines: Vec<Machine> = Vec::new();
        expect!(self, TokenValue::KwMNet);
        expect!(self, TokenValue::LBrace);
        while let Some(Token{value:TokenValue::KwMachine}) = self.lookahead {
            if let Some(mach) = self.parse_machine() {
                machines.push(mach);
            } else {
                return None;
            }
        }
        expect!(self, TokenValue::RBrace);
        Some(MachineNet{machines})
    }
}
