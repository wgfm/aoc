use std::any::Any;
use std::collections::HashMap;

use num::integer::lcm;

use crate::{problem::Solution, solution};

solution!(2023, 20);

impl Solution for Problem {
    fn part_a(&self) -> anyhow::Result<String> {
        let mut modules = parse(&self.input);

        let mut high_pulses = 0;
        let mut low_pulses = 0;

        dbg!(&modules);

        for _ in 0..1000 {
            let mut to_process: Vec<Box<dyn Module>> = vec![Box::new(Button::new())];

            while to_process.len() > 0 {
                let mut next_to_process: Vec<Box<dyn Module>> = vec![];
                for module in to_process.iter() {
                    if let Some(pulse) = module.pulse() {
                        match pulse {
                            Pulse::High => high_pulses += module.outputs().len(),
                            Pulse::Low => low_pulses += module.outputs().len(),
                        }

                        for output_name in module.outputs().iter() {
                            println!("{} -{:?}-> {}", module.name(), pulse, output_name);
                            if let Some(output) = modules.get_mut(output_name) {
                                output.receive_pulse(&module.name(), pulse);
                                next_to_process.push(output.clone());
                            }
                        }
                    }
                }

                to_process = next_to_process;
            }
        }

        dbg!(high_pulses, low_pulses);

        Ok((high_pulses * low_pulses).to_string())
    }

    fn part_b(&self) -> anyhow::Result<String> {
        let mut modules = parse(&self.input);

        let subsystems = vec!["bh", "jf", "sh", "mz"];

        let mut subsolves = vec![];

        for system in subsystems {
            let (total, loop_len) = solve(&mut modules, system);
            println!("{}: total {}, loop len {}", system, total, loop_len);
            subsolves.push(loop_len);
        }

        let answer = subsolves.iter().fold(1, |acc, x| lcm(acc, *x));

        Ok(answer.to_string())
    }
}

fn solve(modules: &mut HashMap<String, Box<dyn Module>>, target: &str) -> (u64, u64) {
    let mut button_presses = 0;
    let mut counting_loop = false;
    let mut loop_count = 0;
    loop {
        button_presses += 1;
        if counting_loop {
            loop_count += 1;
        }
        let mut to_process: Vec<Box<dyn Module>> = vec![Box::new(Button::new())];

        while to_process.len() > 0 {
            let mut next_to_process: Vec<Box<dyn Module>> = vec![];
            for module in to_process.iter() {
                if let Some(pulse) = module.pulse() {
                    if module.name() == target && pulse == Pulse::High {
                        if !counting_loop {
                            counting_loop = true;
                        } else {
                            return (button_presses, loop_count);
                        }
                    }

                    for output_name in module.outputs().iter() {
                        if let Some(output) = modules.get_mut(output_name) {
                            output.receive_pulse(&module.name(), pulse);
                            next_to_process.push(output.clone());
                        }
                    }
                }
            }

            to_process = next_to_process;
        }
    }
}

impl Clone for Box<dyn Module> {
    fn clone(&self) -> Self {
        if let Some(conjunction) = self.as_any().downcast_ref::<Conjunction>() {
            Box::new(conjunction.clone())
        } else if let Some(flip_flop) = self.as_any().downcast_ref::<FlipFlop>() {
            Box::new(flip_flop.clone())
        } else if let Some(broadcaster) = self.as_any().downcast_ref::<Broadcaster>() {
            Box::new(broadcaster.clone())
        } else if let Some(button) = self.as_any().downcast_ref::<Button>() {
            Box::new(button.clone())
        } else {
            unreachable!()
        }
    }
}

/*
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

trait Module: std::fmt::Debug {
    fn receive_pulse(&mut self, from: &str, pulse: Pulse);
    fn pulse(&self) -> Option<Pulse>;

    fn name(&self) -> String;
    fn outputs(&self) -> &[String];

    // blech
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Clone)]
struct Conjunction {
    name: String,
    num_inputs: usize,
    outputs: Vec<String>,
    incoming_pulses: HashMap<String, Pulse>,
}

impl std::fmt::Debug for Conjunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pulse_info = if self
            .incoming_pulses
            .values()
            .any(|pulse| pulse == &Pulse::Low)
        {
            "all pulses high"
        } else {
            "some pulses low"
        };

        f.write_fmt(format_args!("Conjunction {}, {}", self.name, pulse_info))
    }
}

impl Module for Conjunction {
    fn pulse(&self) -> Option<Pulse> {
        if self
            .incoming_pulses
            .values()
            .all(|pulse| pulse == &Pulse::High)
            && self.incoming_pulses.len() == self.num_inputs
        {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }

    fn receive_pulse(&mut self, from: &str, pulse: Pulse) {
        self.incoming_pulses.insert(from.to_string(), pulse);
    }
    fn outputs(&self) -> &[String] {
        &self.outputs
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Clone)]
struct FlipFlop {
    name: String,
    is_on: bool,
    outputs: Vec<String>,
    received: Pulse,
}

impl Module for FlipFlop {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn pulse(&self) -> Option<Pulse> {
        if self.received == Pulse::Low {
            let p = if self.is_on { Pulse::High } else { Pulse::Low };

            Some(p)
        } else {
            None
        }
    }

    fn receive_pulse(&mut self, _from: &str, pulse: Pulse) {
        self.received = pulse;
        if pulse == Pulse::Low {
            self.is_on = !self.is_on;
        }
    }

    fn outputs(&self) -> &[String] {
        &self.outputs
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl std::fmt::Debug for FlipFlop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "FlipFlop {}, currently {}",
            self.name,
            if self.is_on { "on" } else { "off" },
        ))
    }
}

#[derive(Clone)]
struct Broadcaster {
    name: String,
    outputs: Vec<String>,
}

impl Module for Broadcaster {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn receive_pulse(&mut self, _from: &str, _pulse: Pulse) {}

    fn pulse(&self) -> Option<Pulse> {
        Some(Pulse::Low)
    }

    fn outputs(&self) -> &[String] {
        &self.outputs
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl std::fmt::Debug for Broadcaster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Broadcaster")
    }
}

#[derive(Clone, Debug)]
struct Button {
    outputs: Vec<String>,
}

impl Button {
    fn new() -> Self {
        Self {
            outputs: vec!["broadcaster".to_string()],
        }
    }
}

impl Module for Button {
    fn receive_pulse(&mut self, _from: &str, _pulse: Pulse) {
        unreachable!()
    }

    fn pulse(&self) -> Option<Pulse> {
        Some(Pulse::Low)
    }

    fn name(&self) -> String {
        "button".to_string()
    }

    fn outputs(&self) -> &[String] {
        &self.outputs
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn parse(input: &str) -> HashMap<String, Box<dyn Module>> {
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    let mut conjs = vec![];
    for line in input.lines() {
        let (name, rest) = line.split_once(" -> ").unwrap();

        let outputs: Vec<_> = rest.split(", ").map(str::to_string).collect();

        match name.chars().next().unwrap() {
            'b' => {
                modules.insert(
                    "broadcaster".to_string(),
                    Box::new(Broadcaster {
                        name: "broadcaster".to_string(),
                        outputs,
                    }),
                );
            }
            '%' => {
                modules.insert(
                    name[1..].to_string(),
                    Box::new(FlipFlop {
                        name: name[1..].to_string(),
                        is_on: false,
                        outputs,
                        received: Pulse::Low,
                    }),
                );
            }
            '&' => {
                conjs.push(&name[1..]);
                modules.insert(
                    name[1..].to_string(),
                    Box::new(Conjunction {
                        name: name[1..].to_string(),
                        num_inputs: 0,
                        outputs,
                        incoming_pulses: HashMap::new(),
                    }),
                );
            }
            _ => unreachable!(),
        }
    }

    for conj in conjs {
        let mut count = 0;
        for module in modules.values() {
            if module.outputs().contains(&conj.to_string()) {
                count += 1;
            }
        }

        if let Some(module) = modules.get_mut(conj) {
            if let Some(conj) = module.as_any_mut().downcast_mut::<Conjunction>() {
                conj.num_inputs = count;
            }
        }
    }

    modules
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_ONE: &'static str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const INPUT_TWO: &'static str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    //    #[test]
    fn test_a_one() {
        let p = Problem::with_input(INPUT_ONE);
        assert_eq!(p.part_a().unwrap(), "32000000");
    }

    #[test]
    fn test_a_two() {
        let p = Problem::with_input(INPUT_TWO);
        assert_eq!(p.part_a().unwrap(), "11687500");
    }
}
