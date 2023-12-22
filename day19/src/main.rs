use std::{cmp::Ordering, collections::HashMap, future::IntoFuture, ops::Range};

fn main1() {
    let input = std::fs::read_to_string("input").unwrap();
    let (workflows_str, parts_str) = input.split_once("\n\n").unwrap();
    let mut workflows = HashMap::new();
    fn attr_index(attr: char) -> usize {
        match attr {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => panic!(),
        }
    }
    struct Rule<'a> {
        attr: usize,
        operator: Ordering,
        value: i32,
        dest: &'a str,
    }
    for line in workflows_str.lines() {
        let (name, rules) = line.split_once('{').unwrap();
        let (main_rules, last_rule) = rules.rsplit_once(',').unwrap();
        let last_rule = last_rule.strip_suffix('}').unwrap();
        let rules = workflows.entry(name).or_insert_with(Vec::new);
        for rule in main_rules.split(',') {
            let (cond, dest) = rule.split_once(':').unwrap();
            let attr = attr_index(cond.chars().next().unwrap());
            let operator = match cond.chars().nth(1) {
                Some('<') => Ordering::Less,
                Some('>') => Ordering::Greater,
                _ => panic!(),
            };
            let value = cond[2..].parse().unwrap();
            rules.push(Rule {
                attr,
                operator,
                value,
                dest,
            });
        }
        rules.push(Rule {
            attr: 0,
            operator: Ordering::Less,
            value: i32::MAX,
            dest: last_rule,
        });
    }
    let mut sum = 0;
    for line in parts_str.lines() {
        let body = line.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
        let mut values = [0; 4];
        for item in body.split(',') {
            let attr = attr_index(item.chars().next().unwrap());
            values[attr] = item[2..].parse().unwrap();
        }
        let mut wf = "in";
        loop {
            let rules = &workflows[wf];
            for rule in rules {
                let value = values[rule.attr];
                if rule.operator == value.cmp(&rule.value) {
                    wf = rule.dest;
                    break;
                }
            }
            if wf == "R" {
                break;
            } else if wf == "A" {
                sum += values.iter().sum::<i32>();
                break;
            }
        }
    }
    dbg!(sum);
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let (workflows_str, parts_str) = input.split_once("\n\n").unwrap();
    let mut workflows = HashMap::new();
    fn attr_index(attr: char) -> usize {
        match attr {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => panic!(),
        }
    }
    #[derive(Debug)]
    struct Rule<'a> {
        attr: usize,
        operator: Ordering,
        value: i32,
        dest: &'a str,
    }
    for line in workflows_str.lines() {
        let (name, rules) = line.split_once('{').unwrap();
        let (main_rules, last_rule) = rules.rsplit_once(',').unwrap();
        let last_rule = last_rule.strip_suffix('}').unwrap();
        let rules = workflows.entry(name).or_insert_with(Vec::new);
        for rule in main_rules.split(',') {
            let (cond, dest) = rule.split_once(':').unwrap();
            let attr = attr_index(cond.chars().next().unwrap());
            let operator = match cond.chars().nth(1) {
                Some('<') => Ordering::Less,
                Some('>') => Ordering::Greater,
                _ => panic!(),
            };
            let value = cond[2..].parse().unwrap();
            rules.push(Rule {
                attr,
                operator,
                value,
                dest,
            });
        }
        rules.push(Rule {
            attr: 0,
            operator: Ordering::Less,
            value: i32::MAX,
            dest: last_rule,
        });
    }
    struct Trace<'a> {
        rule: &'a Rule<'a>,
        applies: bool,
        prev: Option<&'a Trace<'a>>,
    }
    fn resolve(
        workflows: &HashMap<&str, Vec<Rule<'_>>>,
        rules: &[Rule<'_>],
        values: [Range<i32>; 4],
        trace: Option<&Trace<'_>>,
    ) -> usize {
        match rules {
            [last_rule] => match last_rule.dest {
                "A" => {
                    let [x, m, a, s] = &values;
                    let product = x.len() * m.len() * a.len() * s.len();
                    println!("Accept {values:?} = {product}");
                    let mut next = trace;
                    while let Some(node) = next {
                        //println!("{:?} ({:?})", node.rule, node.applies);
                        next = node.prev;
                    }
                    //println!();
                    product
                }
                "R" => {
                    let [x, m, a, s] = &values;
                    let product = x.len() * m.len() * a.len() * s.len();
                    println!("Reject {values:?} = {product}");
                    let mut next = trace;
                    while let Some(node) = next {
                        //println!("{:?} ({:?})", node.rule, node.applies);
                        next = node.prev;
                    }
                    //println!();
                    0
                }
                dest => {
                    let recurse_rules = &workflows[dest][..];
                    let trace = Trace {
                        rule: last_rule,
                        applies: true,
                        prev: trace,
                    };
                    resolve(workflows, recurse_rules, values, Some(&trace))
                }
            },
            [rule, rest @ ..] => {
                let mut includes = values.clone();
                let mut excludes = values.clone();
                if rule.operator == Ordering::Less {
                    includes[rule.attr].end = includes[rule.attr].end.min(rule.value);
                    excludes[rule.attr].start = excludes[rule.attr].start.max(rule.value);
                } else {
                    includes[rule.attr].start = includes[rule.attr].start.max(rule.value + 1);
                    excludes[rule.attr].end = excludes[rule.attr].end.min(rule.value + 1);
                }
                let trace = Trace {
                    rule,
                    applies: true,
                    prev: trace,
                };
                let include_product = {
                    let [x, m, a, s] = &includes;
                    x.len() * m.len() * a.len() * s.len()
                };
                let included_count = match rule.dest {
                    "A" => {
                        println!("Accept {includes:?} = {include_product}");
                        let mut next = Some(&trace);
                        while let Some(node) = next {
                            //println!("{:?} ({:?})", node.rule, node.applies);
                            next = node.prev;
                        }
                        //println!();
                        let [x, m, a, s] = includes;
                        x.len() * m.len() * a.len() * s.len()
                    }
                    "R" => {
                        println!("Reject {includes:?} = {include_product}");
                        let mut next = Some(&trace);
                        while let Some(node) = next {
                            //println!("{:?} ({:?})", node.rule, node.applies);
                            next = node.prev;
                        }
                        //println!();
                        0
                    }
                    dest => {
                        let included_rules = &workflows[dest][..];
                        resolve(workflows, included_rules, includes, Some(&trace))
                    }
                };
                let trace = Trace {
                    applies: false,
                    ..trace
                };
                let excluded_rules = rest;
                included_count + resolve(workflows, excluded_rules, excludes, Some(&trace))
            }
            [] => panic!(),
        }
    }
    let result = resolve(
        &workflows,
        &workflows["in"][..],
        [1..4001, 1..4001, 1..4001, 1..4001],
        None,
    );
    dbg!(result);
}
