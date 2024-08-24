mod elr_pilot;
mod fsm;

pub use crate::elr_pilot::*;

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
    /*
    // 2024-07-04
    let mach_s = Machine{name:'S', states:vec![
        State{id:0, transitions:vec![Transition{character:'a', dest_id:1}], is_initial:true, is_final:false},
        State{id:1, transitions:vec![Transition{character:'a', dest_id:1}, Transition{character:'A', dest_id:2}], is_initial:false, is_final:false},
        State{id:2, transitions:vec![], is_initial:false, is_final:true},
    ]};
    let mach_a = Machine{name:'A', states:vec![
        State{id:0, transitions:vec![Transition{character:'b', dest_id:1}, Transition{character:'B', dest_id:2}], is_initial:true, is_final:true},
        State{id:1, transitions:vec![Transition{character:'B', dest_id:2}], is_initial:false, is_final:false},
        State{id:2, transitions:vec![], is_initial:false, is_final:true},
    ]};
    let mach_b = Machine{name:'B', states:vec![
        State{id:0, transitions:vec![Transition{character:'A', dest_id:1}], is_initial:true, is_final:false},
        State{id:1, transitions:vec![Transition{character:'c', dest_id:2}], is_initial:false, is_final:false},
        State{id:2, transitions:vec![], is_initial:false, is_final:true},
    ]};
    let net = MachineNet{machines:vec![mach_s, mach_a, mach_b]};
    */
    let pilot = create_pilot(&net);
    //println!("pilot: {pilot:?}");
    println!("{}", pilot.to_dot());
    pilot.print_conflicts();
}
