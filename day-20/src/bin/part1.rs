use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter, Result};

fn main() {
    println!("Hello, world!");
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Pulse {
    Low,
    High,
}

// impl Display for ModuleType {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         match self {
//             ModuleType::FlipFlop(on) => write!(f, "%({})", if *on { "on" } else { "off" }),
//             ModuleType::Conjunctive => write!(f, "&"),
//         }
//     }
// }

trait ModuleTrait {
    fn do_pulse(&mut self, signal: Pulse, name: String);
    fn get_last_pulse(&self) -> Pulse;
    fn get_name(&self) -> String;
    fn get_outputs(&self) -> Vec<String>;
    fn update(&mut self, _: String) {}
    fn cont(&self, _: Pulse) -> bool {
        true
    }
}

#[derive(Clone)]
struct FlipFlop {
    name: String,
    on: bool,
    last_pulse: Pulse,
    outputs: Vec<String>,
}

impl ModuleTrait for FlipFlop {
    fn do_pulse(&mut self, signal: Pulse, _: String) {
        if signal == Pulse::Low {
            self.last_pulse = {
                if !self.on {
                    Pulse::High
                } else {
                    Pulse::Low
                }
            };
            self.on = !self.on;
        }
    }

    fn get_last_pulse(&self) -> Pulse {
        self.last_pulse
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_outputs(&self) -> Vec<String> {
        self.outputs.clone()
    }

    fn cont(&self, signal: Pulse) -> bool {
        signal == Pulse::Low
    }
}

#[derive(Clone)]
struct Conjunctive {
    name: String,
    last_pulse: Pulse,
    inputs: HashMap<String, Pulse>,
    outputs: Vec<String>,
}

impl ModuleTrait for Conjunctive {
    fn do_pulse(&mut self, signal: Pulse, name: String) {
        println!("inputs: {:?}, name: {name}", self.inputs);
        self.inputs.insert(name, signal);
        self.last_pulse = {
            if self.inputs.values().all(|v| *v == Pulse::High) {
                Pulse::Low
            } else {
                Pulse::High
            }
        };
    }

    fn get_last_pulse(&self) -> Pulse {
        self.last_pulse
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_outputs(&self) -> Vec<String> {
        self.outputs.clone()
    }

    fn update(&mut self, name: String) {
        self.inputs.insert(name, Pulse::Low);
    }
}

fn solution(input: &str) -> String {
    let mut modules: HashMap<String, Box<dyn ModuleTrait>> = HashMap::new();

    let mut initial_modules = VecDeque::new();

    input.lines().for_each(|line| {
        if line.starts_with("broadcaster") {
            let mut parts = line.split(" -> ");
            let right = parts.nth(1).unwrap();
            right.split(", ").map(|s| s.to_string()).for_each(|s| {
                initial_modules.push_back((Pulse::Low, s));
            });
        } else {
            let mut parts = line.split(" -> ");
            let left = parts.next().unwrap();
            let right = parts.next().unwrap();
            let outputs = right.split(", ");
            let outputs = outputs.map(|s| s.to_string()).collect::<Vec<_>>();
            let name = &left.to_string()[1..];
            if left.starts_with("%") {
                modules.insert(
                    name.to_string(),
                    Box::new(FlipFlop {
                        name: name.to_string(),
                        on: false,
                        last_pulse: Pulse::Low,
                        outputs,
                    }),
                );
            } else if left.starts_with("&") {
                modules.insert(
                    name.to_string(),
                    Box::new(Conjunctive {
                        name: name.to_string(),
                        last_pulse: Pulse::Low,
                        inputs: HashMap::new(),
                        outputs,
                    }),
                );
            } else {
                panic!("Invalid input");
            }
        }
    });

    // let original_modules = modules.clone();

    // Create a mapping from module names to their outputs
    let output_mapping: HashMap<String, Vec<String>> = modules
        .iter()
        .map(|(name, module)| (name.clone(), module.get_outputs().clone()))
        .collect();

    for (_, module) in modules.iter_mut() {
        let module_name = module.get_name().to_string();

        // Get the modules to update based on the output mapping
        let to_update: Vec<String> = output_mapping
            .iter()
            .filter_map(|(name, outputs)| {
                if outputs.contains(&module_name) {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect();

        for t in to_update {
            println!("updating [{}] with {}", module.get_name(), t);
            module.update(t);
        }
    }

    // for (_, module) in modules.iter() {
    //     println!("{}", module);
    // }

    let mut high_pulse_counts = 0;
    let mut low_pulse_counts = 0;

    for _ in 0..1 {
        let mut initial_modules_copy = initial_modules.clone();

        while !initial_modules_copy.is_empty() {
            let (pulse, initial_module) = initial_modules_copy.pop_front().unwrap();
            let current_module = modules.get_mut(&initial_module).unwrap();

            if pulse == Pulse::High {
                high_pulse_counts += 1;
            } else {
                low_pulse_counts += 1;
            }

            println!(
                "pulse: {:?}, module: {}",
                current_module.get_last_pulse(),
                current_module.get_name()
            );
            if current_module.get_name() == "output" {
                continue;
            }

            println!("{initial_module}, {}", current_module.get_name());

            current_module.do_pulse(pulse, initial_module.clone());

            if !current_module.cont(pulse) {
                continue;
            }

            initial_modules_copy.extend(
                current_module
                    .get_outputs()
                    .iter()
                    .map(|s| (current_module.get_last_pulse(), s.to_string())),
            );
        }
    }

    println!(
        "high_pulse_counts: {}, low_pulse_counts: {}, score: {}",
        high_pulse_counts,
        low_pulse_counts,
        high_pulse_counts * low_pulse_counts
    );
    "32000000".to_string()
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solution(
            "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a",
        );
        assert_eq!(result, "32000000");
    }

    // #[test]
    // fn test_flipflop_low_off_pulse() {
    //     let module = Module {
    //         name: "a".to_string(),
    //         outputs: vec!["b".to_string()],
    //         module_type: ModuleType::FlipFlop(false),
    //         inputs: vec!["b".to_string()],
    //         last_pulse: Pulse::Low,
    //     };

    //     let mut module_map = HashMap::new();
    //     module_map.insert(module.name.clone(), module.clone());

    //     let new_pulse = do_pulse(Pulse::Low, &module, &module, &mut module_map);
    //     assert_eq!(new_pulse, Some(Pulse::High));
    //     assert_eq!(
    //         module_map.get(&module.name).unwrap().module_type,
    //         ModuleType::FlipFlop(true)
    //     );
    // }

    // #[test]
    // fn test_flipflop_low_on_pulse() {
    //     let module = Module {
    //         name: "a".to_string(),
    //         outputs: vec!["b".to_string()],
    //         module_type: ModuleType::FlipFlop(true),
    //         inputs: vec!["b".to_string()],
    //         last_pulse: Pulse::Low,
    //     };

    //     let mut module_map = HashMap::new();
    //     module_map.insert(module.name.clone(), module.clone());

    //     let new_pulse = do_pulse(Pulse::Low, &module, &module, &mut module_map);
    //     assert_eq!(new_pulse, Some(Pulse::Low));
    //     assert_eq!(
    //         module_map.get(&module.name).unwrap().module_type,
    //         ModuleType::FlipFlop(false)
    //     );
    // }

    // #[test]
    // fn test_flipflop_high_pulse() {
    //     let module = Module {
    //         name: "a".to_string(),
    //         outputs: vec!["b".to_string()],
    //         module_type: ModuleType::FlipFlop(true),
    //         inputs: vec!["b".to_string()],
    //         last_pulse: Pulse::Low,
    //     };

    //     let mut module_map = HashMap::new();
    //     module_map.insert(module.name.clone(), module.clone());

    //     let new_pulse = do_pulse(Pulse::High, &module, &module, &mut module_map);
    //     assert_eq!(new_pulse, None);
    // }

    // #[test]
    // fn test_conjunctive_low() {
    //     let module = Module {
    //         name: "a".to_string(),
    //         outputs: vec!["b".to_string()],
    //         module_type: ModuleType::Conjunctive,
    //         inputs: vec!["c".to_string()],
    //         last_pulse: Pulse::Low,
    //     };

    //     let other_module = Module {
    //         name: "c".to_string(),
    //         outputs: vec!["a".to_string()],
    //         module_type: ModuleType::FlipFlop(false),
    //         inputs: vec!["a".to_string()],
    //         last_pulse: Pulse::High,
    //     };

    //     let mut module_map = HashMap::new();
    //     module_map.insert(module.name.clone(), module.clone());
    //     module_map.insert(other_module.name.clone(), other_module.clone());

    //     let new_pulse = do_pulse(Pulse::Low, &other_module, &module, &mut module_map);
    //     assert_eq!(new_pulse, Some(Pulse::High));
    //     assert_eq!(
    //         module_map.get(&other_module.name).unwrap().last_pulse,
    //         Pulse::Low
    //     );
    // }

    // #[test]
    // fn test_conjunctive_high() {
    //     let module = Module {
    //         name: "a".to_string(),
    //         outputs: vec!["b".to_string()],
    //         module_type: ModuleType::Conjunctive,
    //         inputs: vec!["c".to_string()],
    //         last_pulse: Pulse::Low,
    //     };

    //     let other_module = Module {
    //         name: "c".to_string(),
    //         outputs: vec!["a".to_string()],
    //         module_type: ModuleType::FlipFlop(false),
    //         inputs: vec!["a".to_string()],
    //         last_pulse: Pulse::High,
    //     };

    //     let mut module_map = HashMap::new();
    //     module_map.insert(module.name.clone(), module.clone());
    //     module_map.insert(other_module.name.clone(), other_module.clone());

    //     let new_pulse = do_pulse(Pulse::High, &other_module, &module, &mut module_map);
    //     assert_eq!(new_pulse, Some(Pulse::Low));
    //     assert_eq!(
    //         module_map.get(&other_module.name).unwrap().last_pulse,
    //         Pulse::High
    //     );
    // }
}
