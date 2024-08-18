use core::cmp::max;
use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Transition {
    character: char,
    dest_id: i32
}

impl Transition {
    fn is_nonterminal(&self) -> bool {
        self.character.is_ascii_uppercase()
    }
}

struct State {
    id: i32,
    transitions: Vec<Transition>,
    is_initial: bool,
    is_final: bool
}

struct Machine {
    name: char,
    states: Vec<State>
}

impl Machine {
    fn lookup_state(&self, id: i32) -> &State {
        for s in &self.states {
            if s.id == id {
                return &s;
            }
        }
        panic!("state {id} does not exist")
    }
}

struct MachineNet {
    machines: Vec<Machine>
}

impl MachineNet {
    fn lookup_machine(&self, machine: char) -> &Machine {
        for m in &self.machines {
            if m.name == machine {
                return &m;
            }
        }
        panic!("machine {machine} does not exist")
    }

    fn lookup_state(&self, machine: char, id: i32) -> &State {
        self.lookup_machine(machine).lookup_state(id)
    }

    fn followers_impl<'a>(&'a self, machine: char, id: i32, visited: &mut HashSet<&'a Transition>, next: &HashSet<char>) -> HashSet<char> {
        let state = self.lookup_state(machine, id);
        let mut res: HashSet<char> = HashSet::new();
        if state.is_final {
            res.extend(next);
        }
        for t in &state.transitions {
            if visited.contains(t) {
                continue;
            }
            visited.insert(t);
            if !t.is_nonterminal() {
                res.insert(t.character);
            } else {
                let nextnext = self.followers_impl(machine, t.dest_id, visited, next);
                let rec_fol = self.followers_impl(t.character, 0, visited, &nextnext);
                res.extend(rec_fol);
            }
        }
        return res;
    }

    fn followers(&self, machine: char, id: i32, next: HashSet<char>) -> HashSet<char> {
        let mut visited: HashSet<&Transition> = HashSet::new();
        return self.followers_impl(machine, id, &mut visited, &next);
    }
}


#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Candidate {
    machine: char,
    state: i32,
    lookahead: char,
    is_seed: bool
}

#[derive(Debug, Clone)]
struct PilotState {
    id: i32,
    candidates: HashSet<Candidate>,
    transitions: Vec<Transition>
}

impl PilotState {
    fn seeds(&self) -> HashSet<&Candidate> {
        self.candidates.iter().filter(|x| x.is_seed).collect::<HashSet<_>>()
    }
    
    fn is_equivalent(&self, other: &PilotState) -> bool {
        let my_seeds = self.seeds();
        let other_seeds = other.seeds();
        return my_seeds == other_seeds;
    }
}

#[derive(Debug)]
struct Pilot {
    states: Vec<PilotState>
}

impl Pilot {
    fn lookup_state(&mut self, id: i32) -> &mut PilotState {
        for s in &mut self.states {
            if s.id == id {
                return s;
            }
        }
        panic!("state {id} does not exist");
    }

    fn insert(&mut self, mut new: PilotState) -> i32 {
        let mut last_id = 0;
        for s in &self.states {
            if s.is_equivalent(&new) {
                return s.id;
            }
            last_id = max(last_id, s.id);
        }
        last_id += 1;
        new.id = last_id;
        self.states.push(new);
        return last_id;
    }
}


fn closure(state: &mut PilotState, net: &MachineNet) {
    let mut worklist: VecDeque<Candidate> = VecDeque::new();
    for c in &state.candidates {
        worklist.push_back(*c);
    }
    while !worklist.is_empty() {
        let c = worklist.pop_front().unwrap();
        let mstate = net.lookup_state(c.machine, c.state);
        for t in &mstate.transitions {
            if !t.is_nonterminal() {
                continue;
            }
            let ini = net.followers(c.machine, t.dest_id, HashSet::from([c.lookahead]));
            for ch in ini {
                let c2 = Candidate{machine:t.character, state:0, lookahead:ch, is_seed:false};
                if state.candidates.insert(c2) {
                    worklist.push_back(c2);
                }
            }
        }
    }
}

fn collect_transitions(state: &PilotState, net: &MachineNet) -> HashSet<char> {
    let mut res: HashSet<char> = HashSet::new();
    for c in &state.candidates {
        let mstate = net.lookup_state(c.machine, c.state);
        for t in &mstate.transitions {
            res.insert(t.character);
        }
    }
    return res;
}

fn shift_candidate(c: &Candidate, net: &MachineNet, next: char) -> Option<Candidate> {
    let mstate = net.lookup_state(c.machine, c.state);
    for t in &mstate.transitions {
        if t.character == next {
            return Some(Candidate{machine:c.machine, state:t.dest_id, lookahead:c.lookahead, is_seed:true});
        }
    }
    return None;
}

fn shift(state: &PilotState, net: &MachineNet, next: char) -> PilotState {
    let mut res = PilotState{id:-1, candidates:HashSet::new(), transitions:vec![]};
    for c in &state.candidates {
        let shifted = shift_candidate(c, net, next);
        if let Some(new_candidate) = shifted {
            res.candidates.insert(new_candidate);
        }
    }
    return res;
}

fn create_pilot(net: &MachineNet) -> Pilot {
    let mut pilot = Pilot{states: vec![]};
    let init_candidate = Candidate{machine:'S', state:0, lookahead:'$', is_seed:true};
    let init_state = PilotState{id:0, candidates:HashSet::from([init_candidate]), transitions:vec![]};
    pilot.states.push(init_state);

    let mut worklist = VecDeque::from([0]);
    let mut visited: HashSet<i32> = HashSet::new();
    while !worklist.is_empty() {
        let state_id = worklist.pop_front().unwrap();
        if visited.contains(&state_id) {
            continue;
        }
        visited.insert(state_id);
        let state = pilot.lookup_state(state_id);
        closure(state, net);

        let future_xions = collect_transitions(&state, net);
        for xion in future_xions {
            let state = pilot.lookup_state(state_id);
            let maybe_new_state = shift(&state, net, xion);
            let id = pilot.insert(maybe_new_state);
            worklist.push_back(id);
            let state = pilot.lookup_state(state_id);
            state.transitions.push(Transition{character:xion, dest_id:id});
        }
    }

    return pilot;
}


fn main() {
    let mach_s = Machine{name:'S', states:vec![
        State{id:0, transitions:vec![Transition{character:'d', dest_id:1}], is_initial:true, is_final:true},
        State{id:1, transitions:vec![Transition{character:'B', dest_id:2}], is_initial:false, is_final:false},
        State{id:2, transitions:vec![], is_initial:false, is_final:true}
    ]};
    let mach_b = Machine{name:'B', states:vec![
        State{id:0, transitions:vec![Transition{character:'c', dest_id:1}, Transition{character:'b', dest_id:3}], is_initial:true, is_final:false},
        State{id:1, transitions:vec![Transition{character:'A', dest_id:2}], is_initial:false, is_final:false},
        State{id:2, transitions:vec![Transition{character:'b', dest_id:3}], is_initial:false, is_final:false},
        State{id:3, transitions:vec![], is_initial:false, is_final:true}
    ]};
    let mach_a = Machine{name:'A', states:vec![
        State{id:0, transitions:vec![Transition{character:'a', dest_id:1}], is_initial:true, is_final:false},
        State{id:1, transitions:vec![Transition{character:'a', dest_id:1}, Transition{character:'B', dest_id:2}], is_initial:false, is_final:false},
        State{id:2, transitions:vec![], is_initial:false, is_final:true}
    ]};
    let net = MachineNet{machines:vec![mach_s, mach_b, mach_a]};
    /*
    let mach_s = Machine{name:'S', states:vec![
        State{id:0, transitions:vec![Transition{character:'A', dest_id:1}], is_initial:true, is_final:false},
        State{id:1, transitions:vec![Transition{character:'C', dest_id:1}], is_initial:false, is_final:true},
    ]};
    let mach_a = Machine{name:'A', states:vec![
        State{id:0, transitions:vec![Transition{character:'a', dest_id:1}], is_initial:true, is_final:true},
        State{id:1, transitions:vec![Transition{character:'C', dest_id:2}], is_initial:false, is_final:false},
        State{id:2, transitions:vec![Transition{character:'b', dest_id:3}], is_initial:false, is_final:false},
        State{id:3, transitions:vec![], is_initial:false, is_final:true},
    ]};
    let mach_c = Machine{name:'C', states:vec![
        State{id:0, transitions:vec![Transition{character:'c', dest_id:1}], is_initial:true, is_final:true},
        State{id:1, transitions:vec![Transition{character:'A', dest_id:2}], is_initial:false, is_final:true},
        State{id:2, transitions:vec![Transition{character:'d', dest_id:3}], is_initial:false, is_final:false},
        State{id:3, transitions:vec![], is_initial:false, is_final:true},
    ]};
    let net = MachineNet{machines:vec![mach_s, mach_a, mach_c]};
    */
    let pilot = create_pilot(&net);
    println!("pilot: {pilot:?}");
}


