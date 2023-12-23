use std::collections::{HashMap, HashSet, VecDeque};

fn main1() {
    let input = std::fs::read_to_string("input").unwrap();
    struct Module<'a> {
        kind: u8,
        dests: Vec<&'a str>,
    }
    let mut modules = HashMap::new();
    for line in input.lines() {
        let (src, dest) = line.split_once(" -> ").unwrap();
        if src == "broadcaster" {
            modules.insert(
                src,
                Module {
                    kind: b'b',
                    dests: dest.split(", ").collect(),
                },
            );
        } else {
            let kind = src.as_bytes()[0];
            let src = &src[1..];
            modules.insert(
                src,
                Module {
                    kind,
                    dests: dest.split(", ").collect(),
                },
            );
        }
    }
    let mut flip_flop_mem = HashSet::new();
    let mut conj_mem: HashMap<&str, _> = modules
        .iter()
        .filter(|(_, m)| m.kind == b'&')
        .map(|(&name, _)| (name, HashMap::new()))
        .collect();
    for (conj_name, mem) in &mut conj_mem {
        for (&mod_name, module) in &modules {
            if module.dests.contains(conj_name) {
                mem.insert(mod_name, false);
            }
        }
    }
    struct Pulse<'a> {
        src: &'a str,
        high: bool,
        dest: &'a str,
    }
    impl std::fmt::Debug for Pulse<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{} -{}-> {}",
                self.src,
                if self.high { "high" } else { "low" },
                self.dest
            )
        }
    }
    let mut total_low = 0_usize;
    let mut total_high = 0_usize;
    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        queue.push_back(Pulse {
            src: "button",
            high: false,
            dest: "broadcaster",
        });
        while let Some(pulse) = queue.pop_front() {
            println!("{pulse:?}");
            if pulse.high {
                total_high += 1;
            } else {
                total_low += 1;
            }
            let current_name = pulse.dest;
            let Some(current_mod) = modules.get(current_name) else {
                continue;
            };
            match current_mod.kind {
                b'%' => {
                    if !pulse.high {
                        let high = if flip_flop_mem.remove(current_name) {
                            false
                        } else {
                            flip_flop_mem.insert(current_name);
                            true
                        };
                        for dest in &current_mod.dests {
                            queue.push_back(Pulse {
                                src: current_name,
                                high,
                                dest,
                            });
                        }
                    }
                }
                b'&' => {
                    let mem = conj_mem.get_mut(current_name).unwrap();
                    mem.insert(pulse.src, pulse.high);
                    let high = !mem.values().all(|&m| m);
                    for dest in &current_mod.dests {
                        queue.push_back(Pulse {
                            src: current_name,
                            high,
                            dest,
                        });
                    }
                }
                b'b' => {
                    for dest in &current_mod.dests {
                        queue.push_back(Pulse {
                            src: current_name,
                            high: false,
                            dest,
                        });
                    }
                }
                _ => panic!(),
            }
        }
        println!();
    }
    dbg!(total_high * total_low);
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    struct Module<'a> {
        kind: u8,
        dests: Vec<&'a str>,
    }
    let mut modules = HashMap::new();
    for line in input.lines() {
        let (src, dest) = line.split_once(" -> ").unwrap();
        if src == "broadcaster" {
            modules.insert(
                src,
                Module {
                    kind: b'b',
                    dests: dest.split(", ").collect(),
                },
            );
        } else {
            let kind = src.as_bytes()[0];
            let src = &src[1..];
            modules.insert(
                src,
                Module {
                    kind,
                    dests: dest.split(", ").collect(),
                },
            );
        }
    }
    let mut flip_flop_mem: HashMap<&str, bool> = modules
        .iter()
        .filter(|(_, m)| m.kind == b'%')
        .map(|(&name, _)| (name, false))
        .collect();
    let mut conj_mem: HashMap<&str, _> = modules
        .iter()
        .filter(|(_, m)| m.kind == b'&')
        .map(|(&name, _)| (name, HashMap::new()))
        .collect();
    for (conj_name, mem) in &mut conj_mem {
        for (&mod_name, module) in &modules {
            if module.dests.contains(conj_name) {
                mem.insert(mod_name, false);
            }
        }
    }
    struct Pulse<'a> {
        src: &'a str,
        high: bool,
        dest: &'a str,
    }
    impl std::fmt::Debug for Pulse<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{} -{}-> {}",
                self.src,
                if self.high { "high" } else { "low" },
                self.dest
            )
        }
    }
    let terminal_conj = *modules
        .iter()
        .find(|(_, v)| v.dests.contains(&"rx"))
        .unwrap()
        .0;
    assert!(conj_mem.contains_key(terminal_conj));
    dbg!(terminal_conj);
    let mut first_high_pulses = HashMap::new();
    let mut count = 0_u128;
    loop {
        //for v in flip_flop_mem
        //    .values()
        //    .copied()
        //    .chain(conj_mem.values().flat_map(|m| m.values().copied()))
        //{
        //    if v {
        //        print!("X");
        //    } else {
        //        print!(" ");
        //    }
        //}
        //println!();
        //std::thread::sleep(std::time::Duration::from_millis(50));
        //println!("{conj_mem:?}");
        let mut queue = VecDeque::new();
        queue.push_back(Pulse {
            src: "button",
            high: false,
            dest: "broadcaster",
        });
        count += 1;
        while let Some(pulse) = queue.pop_front() {
            //println!("{pulse:?}");
            let current_name = pulse.dest;
            let Some(current_mod) = modules.get(current_name) else {
                continue;
            };
            if pulse.high && pulse.dest == terminal_conj {
                first_high_pulses.entry(pulse.src).or_insert(count);
                if first_high_pulses.len() == conj_mem.get(terminal_conj).unwrap().len() {
                    dbg!(&first_high_pulses);
                    dbg!(first_high_pulses.values().product::<u128>());
                    return;
                }
            }
            match current_mod.kind {
                b'%' => {
                    if !pulse.high {
                        let state = flip_flop_mem.get_mut(current_name).unwrap();
                        let high = !*state;
                        *state = !*state;
                        for dest in &current_mod.dests {
                            queue.push_back(Pulse {
                                src: current_name,
                                high,
                                dest,
                            });
                        }
                    }
                }
                b'&' => {
                    let mem = conj_mem.get_mut(current_name).unwrap();
                    mem.insert(pulse.src, pulse.high);
                    let high = !mem.values().all(|&m| m);
                    // if !high && ["bx", "bc", "gj", "qq"].contains(&current_name) {
                    //     println!("{current_name} @ {count}");
                    // }
                    for dest in &current_mod.dests {
                        queue.push_back(Pulse {
                            src: current_name,
                            high,
                            dest,
                        });
                    }
                }
                b'b' => {
                    for dest in &current_mod.dests {
                        queue.push_back(Pulse {
                            src: current_name,
                            high: pulse.high,
                            dest,
                        });
                    }
                }
                _ => panic!(),
            }
        }
        //println!();
    }
}
