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


#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct Candidate {
    machine: char,
    state: i32,
    lookahead: char,
    is_seed: bool,
    is_final: bool
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct PilotTransition {
    character: char,
    dest_id: i32,
    multiplicity: i32
}

struct ShiftReduceConflict {
    state_id: i32,
    candidate_idx: usize
}

struct ReduceReduceConflict {
    state_id: i32,
    candidate_1_idx: usize,
    candidate_2_idx: usize
}

struct ConvergenceConflict {
    state_1_id: i32,
    transition_char: char,
    state_2_id: i32
}

#[derive(Debug, Clone)]
struct PilotState {
    id: i32,
    candidates: Vec<Candidate>,
    transitions: Vec<PilotTransition>
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

    fn shift_reduce_conflicts(&self) -> Vec<ShiftReduceConflict> {
        let outgoing: HashSet<char> = self.transitions.iter().map(|trans| {
            trans.character
        }).collect();
        self.candidates.iter().enumerate().filter_map(|(i, cand)| {
            if cand.is_final && outgoing.contains(&cand.lookahead) {
                Some(ShiftReduceConflict{state_id:self.id, candidate_idx:i})
            } else {
                None
            }
        }).collect()
    }

    fn reduce_reduce_conflicts(&self) -> Vec<ReduceReduceConflict> {
        let mut res: Vec<ReduceReduceConflict> = Vec::new();
        for i in 0 .. self.candidates.len() {
            for j in i+1 .. self.candidates.len() {
                let ci = &self.candidates[i];
                let cj = &self.candidates[j];
                if ci.is_final && cj.is_final && ci.lookahead == cj.lookahead {
                    res.push(ReduceReduceConflict{state_id: self.id, candidate_1_idx:i, candidate_2_idx:j});
                }
            }
        }
        res
    }
}

#[derive(Debug)]
struct Pilot {
    states: Vec<PilotState>
}

impl Pilot {
    fn lookup_state_mut(&mut self, id: i32) -> &mut PilotState {
        for s in &mut self.states {
            if s.id == id {
                return s;
            }
        }
        panic!("state {id} does not exist");
    }

    fn lookup_state(&self, id: i32) -> &PilotState {
        for s in &self.states {
            if s.id == id {
                return s;
            }
        }
        panic!("state {id} does not exist");
    }

    fn insert(&mut self, mut new: PilotState, net: &MachineNet) -> i32 {
        for s in &self.states {
            if s.is_equivalent(&new) {
                return s.id;
            }
        }
        let id = self.states.len() as i32;
        new.id = id;
        closure(&mut new, net);
        self.states.push(new);
        return id;
    }

    fn convergence_conflicts(&self) -> Vec<ConvergenceConflict> {
        let mut res: Vec<ConvergenceConflict> = Vec::new();
        for (i, state) in self.states.iter().enumerate() {
            for trans in &state.transitions {
                let dest_state = self.lookup_state(trans.dest_id);
                let n_seeds = dest_state.seeds().len();
                if n_seeds != trans.multiplicity as usize {
                    res.push(ConvergenceConflict{state_1_id:i as i32, transition_char:trans.character, state_2_id:trans.dest_id});
                }
            }
        }
        res
    }

    fn print_shift_reduce_conflict(&self, c: &ShiftReduceConflict) {
        let s = c.state_id;
        let c1 = c.candidate_idx;
        println!("shift-reduce conflict in state {s}, candidate {c1} is final");
    }

    fn print_reduce_reduce_conflict(&self, c: &ReduceReduceConflict) {
        let s = c.state_id;
        let c1 = c.candidate_1_idx;
        let c2 = c.candidate_2_idx;
        println!("reduce-reduce conflict in state {s}, candidates {c1} and {c2}");
    }
    
    fn print_convergence_conflict(&self, c: &ConvergenceConflict) {
        let s1 = c.state_1_id;
        let ts = c.transition_char;
        let s2 = c.state_2_id;
        println!("convergence conflict: multiple transition from state {s1} character {ts} leads to merged base set in state {s2}");
    }

    fn print_conflicts(&self) -> () {
        let mut n_confl = 0;
        for state in &self.states {
            let sr_confl = state.shift_reduce_conflicts();
            for confl in &sr_confl {
                self.print_shift_reduce_conflict(confl);
                n_confl += 1;
            }
            let rr_confl = state.reduce_reduce_conflicts();
            for confl in &rr_confl {
                self.print_reduce_reduce_conflict(confl);
                n_confl += 1;
            }
        }
        let c_confl = self.convergence_conflicts();
        for confl in &c_confl {
            self.print_convergence_conflict(confl);
            n_confl += 1;
        }
        if n_confl == 0 {
            println!("no conflicts");
        }
    }
}


fn closure(state: &mut PilotState, net: &MachineNet) {
    let mut candidate_id: usize = 0;
    while candidate_id < state.candidates.len() {
        let c = state.candidates[candidate_id];
        let mstate = net.lookup_state(c.machine, c.state);
        for t in &mstate.transitions {
            if !t.is_nonterminal() {
                continue;
            }
            let ini = net.followers(c.machine, t.dest_id, HashSet::from([c.lookahead]));
            for ch in ini {
                let dest_state = net.lookup_state(t.character, 0);
                let c2 = Candidate{machine:t.character, state:0, lookahead:ch, is_seed:false, is_final:dest_state.is_final};
                if !state.candidates.contains(&c2) {
                    state.candidates.push(c2);
                }
            }
        }
        candidate_id += 1;
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
            let dest_state = net.lookup_state(c.machine, t.dest_id);
            return Some(Candidate{machine:c.machine, state:t.dest_id, lookahead:c.lookahead, is_seed:true, is_final:dest_state.is_final});
        }
    }
    return None;
}

fn shift(state: &PilotState, net: &MachineNet, next: char) -> (char, PilotState, i32) {
    let mut new_cand: Vec<Candidate> = state.candidates.iter().filter_map(|c| {
        shift_candidate(c, net, next)
    }).collect();
    let mult = new_cand.len() as i32;
    new_cand.sort();
    new_cand.dedup();
    (next, PilotState{id:-1, candidates:new_cand, transitions:vec![]}, mult)
}

fn create_pilot(net: &MachineNet) -> Pilot {
    let first_state = net.lookup_state('S', 0);
    let init_candidate = Candidate{machine:'S', state:0, lookahead:'$', is_seed:false, is_final:first_state.is_final};
    let init_state = PilotState{id:0, candidates:vec![init_candidate], transitions:vec![]};
    let mut pilot = Pilot{states: vec![]};

    let mut worklist = VecDeque::from([pilot.insert(init_state, net)]);
    let mut visited: HashSet<i32> = HashSet::new();
    while !worklist.is_empty() {
        let state_id = worklist.pop_front().unwrap();
        if visited.contains(&state_id) {
            continue;
        }
        visited.insert(state_id);

        let state = pilot.lookup_state(state_id);
        let future_xions = collect_transitions(state, net);
        let shifts: Vec<_> = future_xions.into_iter().map(|c| {
            shift(&state, net, c)
        }).collect();
        let xions: Vec<_> = shifts.into_iter().map(|(c, maybe_new_state, mult)| {
            let id = pilot.insert(maybe_new_state, net);
            PilotTransition{character:c, dest_id:id, multiplicity:mult}
        }).collect();
        worklist.extend(xions.iter().map(|xion| xion.dest_id));
        pilot.lookup_state_mut(state_id).transitions = xions;
    }

    return pilot;
}


fn main() {
    /*
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
    */
    // 2024-02-13
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
        State{id:0, transitions:vec![Transition{character:'c', dest_id:1}], is_initial:true, is_final:false},
        State{id:1, transitions:vec![Transition{character:'A', dest_id:2}], is_initial:false, is_final:true},
        State{id:2, transitions:vec![Transition{character:'d', dest_id:3}], is_initial:false, is_final:false},
        State{id:3, transitions:vec![], is_initial:false, is_final:true},
    ]};
    let net = MachineNet{machines:vec![mach_s, mach_a, mach_c]};
    /*
    let mach_s = Machine{name:'S', states:vec![
        State{id:0, transitions:vec![Transition{character:'d', dest_id:1}], is_initial:true, is_final:false},
        State{id:1, transitions:vec![Transition{character:'A', dest_id:2}], is_initial:false, is_final:true},
        State{id:2, transitions:vec![Transition{character:'c', dest_id:1}], is_initial:false, is_final:false},
    ]};
    let mach_a = Machine{name:'A', states:vec![
        State{id:0, transitions:vec![Transition{character:'a', dest_id:1}], is_initial:true, is_final:false},
        State{id:1, transitions:vec![Transition{character:'S', dest_id:2}], is_initial:false, is_final:true},
        State{id:2, transitions:vec![Transition{character:'b', dest_id:3}], is_initial:false, is_final:false},
        State{id:3, transitions:vec![], is_initial:false, is_final:true},
    ]};
    let net = MachineNet{machines:vec![mach_s, mach_a]};
    */
    let pilot = create_pilot(&net);
    println!("pilot: {pilot:?}");
    pilot.print_conflicts();
}
