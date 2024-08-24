use crate::elr_pilot::*;

struct MergedCandidate {
    machine: char,
    state: i32,
    lookaheads: Vec<char>,
    is_seed: bool,
    is_final: bool
}

impl MergedCandidate {
    fn to_dot_label_html(&self) -> String {
        let look_strs: Vec<_> = self.lookaheads.iter().map(|c| c.to_string()).collect();
        let look_str = look_strs.join(", ");
        let state = format!("{}<sub>{}</sub>", self.state, self.machine);
        let state = if self.is_final { format!("({})", state) } else { state };
        format!("<tr><td sides=\"ltb\">{state}</td><td sides=\"trb\">{look_str}</td></tr>")
    }
}

impl PilotState {
    fn merged_candidates(&self) -> Vec<MergedCandidate> {
        let mut states: Vec<(char, i32)> = self.candidates.iter().map(|c| {
            (c.machine, c.state)
        }).collect();
        states.sort();
        states.dedup();
        states.into_iter().map(|(machine, state)| {
            let raw_candidates: Vec<_> = self.candidates.iter().filter(|c| {
                c.machine == machine && c.state == state
            }).collect();
            let mut lookaheads: Vec<char> = raw_candidates.iter().map(|c| {
                if c.lookahead == '$' { '⊣' } else { c.lookahead }
            }).collect();
            lookaheads.sort();
            let is_seed = raw_candidates.iter().fold(false, |v, c| {
                v || c.is_seed
            });
            let is_final: bool = raw_candidates[0].is_final;
            MergedCandidate{machine, state, lookaheads, is_seed, is_final}
        }).collect()
    }

    fn to_dot(&self) -> String {
        let mut res: Vec<String> = Vec::new();
        res.push(format!("  i{} [label=<", self.id));

        let merged = self.merged_candidates();
        let base_sets: Vec<_> = merged.iter().filter_map(|c| {
            if c.is_seed {
                Some(format!("    {}", c.to_dot_label_html()))
            } else {
                None
            }
        }).collect();
        let others: Vec<_> = merged.iter().filter_map(|c| {
            if !c.is_seed {
                Some(format!("    {}", c.to_dot_label_html()))
            } else {
                None
            }
        }).collect();

        let sep_border_top = if base_sets.is_empty() { "t" } else { "" };
        let sep_border_bot = if others.is_empty() { "b" } else { "" };
        let sep_border_sides = if sep_border_bot != "" || sep_border_top != "" {
            format!("sides=\"{}{}\"", sep_border_top, sep_border_bot)
        } else {
            "border=\"0\"".to_string()
        };
        let sep_border = format!("    <tr><td colspan=\"2\" {}></td></tr>", sep_border_sides);
        
        res.push("    <table border=\"0\" cellborder=\"1\" cellspacing=\"0\">".to_string());
        res.extend(base_sets);
        res.push(sep_border);
        res.extend(others);
        res.push("    </table>".to_string());

        let node_id = format!("I<sub>{}</sub>", self.id);
        res.push(format!("  >, xlabel=<{}>];", node_id));

        let transitions: Vec<_> = self.transitions.iter().map(|t| {
            format!("  i{} -> i{} [label=\"{}\"];", self.id, t.dest_id, t.character)
        }).collect();
        res.extend(transitions);

        res.join("\n")
    }
}

impl Pilot {
    pub fn to_dot(&self) -> String {
        let header = "digraph {\n  node [shape=\"plain\", forcelabels=true];\n";
        let states = self.states.iter().map(|s| {
            s.to_dot()
        }).collect::<Vec<_>>().join("\n");
        let trailer = "\n}";
        format!("{}{}{}", header, states, trailer)
    }
}
