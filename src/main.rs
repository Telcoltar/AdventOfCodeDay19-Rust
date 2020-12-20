use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use log::{debug, info};
use itertools::Itertools;

fn return_split(rule_line: &str) -> (i32, Vec<Vec<String>>) {
    let mut split_str = rule_line.split(":");
    let index = split_str.next().unwrap().parse::<i32>().unwrap();
    let body_str: String = split_str.next().unwrap().parse().unwrap();
    let rule_parts = body_str.split("|");
    let mut rules: Vec<Vec<String>> = Vec::new();
    for part in rule_parts {
        rules.push(part.trim().split(" ").
            map(|s| s.replace("\"", "").parse().unwrap()).collect::<Vec<String>>())
    }
    return (index, rules);
}

fn get_input_data(filename: &str) -> (HashMap<i32, Vec<Vec<String>>>, Vec<String>) {
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    let mut contrains: HashMap<i32, Vec<Vec<String>>> = HashMap::new();
    let mut messages: Vec<String> = Vec::new();

    let mut messages_part = false;

    for r_line in f.lines() {
        let line = r_line.unwrap();
        if messages_part {
            messages.push(line);
        } else {
            if line.trim() == "" {
                messages_part = true;
                continue;
            }
            let (index, body) = return_split(line.trim());
            contrains.insert(index, body);
        }
    }
    return (contrains, messages);
}

fn product(lists: Vec<Vec<Vec<String>>>) -> Vec<Vec<String>> {
    debug!("List: {:?}", lists);
    let iters = lists.iter();
    let v = iters.multi_cartesian_product();
    let mut result: Vec<Vec<String>> = Vec::new();
    for el in v {
        let el_iter = el.into_iter().flatten();
        result.push(el_iter.map(|s| String::from(s)).collect());
    }
    debug!("List: {:?}", result);
    return result;
}

fn compile_rules(rules: &HashMap<i32, Vec<Vec<String>>>,
                 num: i32) -> Vec<Vec<String>> {
    if rules.get(&num).unwrap() == &vec![vec![String::from("a")]] ||
        rules.get(&num).unwrap() == &vec![vec![String::from("b")]] {
        let result = rules.get(&num).unwrap().clone();
        return result;
    }
    let mut new_rules: Vec<Vec<String>> = Vec::new();
    for l in rules.get(&num).unwrap() {
        let mut l_rules: Vec<Vec<Vec<String>>> = Vec::new();
        for d in l {
            l_rules.push(compile_rules(rules, d.parse::<i32>().unwrap()));
        }
        let mut product = product(l_rules);
        debug!("Product: {:?}", product);
        new_rules.append(&mut product);
    }
    return new_rules;
}

fn prepare_data(filename: &str) -> (Vec<String>, HashMap<i32, Vec<String>>) {
    let (rules, messages) = get_input_data(filename);
    let comp_42 = compile_rules(&rules, 42);
    let comp_31 = compile_rules(&rules, 31);
    let comp_42_flat = comp_42.iter().map(|s| s.join("")).collect();
    let comp_31_flat = comp_31.iter().map(|s| s.join("")).collect();
    let mut comps: HashMap<i32, Vec<String>> = HashMap::new();
    comps.insert(42, comp_42_flat);
    comps.insert(31, comp_31_flat);
    return (messages, comps);
}

fn solution_part_1(filename: &str) -> i32 {
    let (messages, comps) = prepare_data(filename);
    let mut count = 0;
    for m in messages {
        if message_valid_part_1(&m, &comps) {
            count += 1;
        }
    }
    return count;
}

fn solution_part_2(filename: &str) -> i32 {
    let (messages, comps) = prepare_data(filename);
    let mut count = 0;
    for m in messages {
        if message_valid_part_2(&m, &comps) {
            count += 1;
        }
    }
    return count;
}

fn starts_with(patterns: &Vec<String>, message: &str, start: usize) -> bool {
    for p in patterns {
        if message[start..].starts_with(p) {
            return true;
        }
    }
    return false;
}


fn message_valid_part_1(message: &str, comps: &HashMap<i32, Vec<String>>) -> bool {
    let part_len = comps.get(&31).unwrap().get(0).unwrap().len();
    if message.len() != 3*part_len {
        return false;
    }
    let mut i = 0;
    if !starts_with(comps.get(&42).unwrap(), message, i) {
        return false;
    }
    i = part_len;
    if !starts_with(comps.get(&42).unwrap(), message, i) {
        return false;
    }
    i = 2*part_len;
    if !starts_with(comps.get(&31).unwrap(), message, i) {
        return false;
    }
    return true;
}

fn message_valid_part_2(message: &str, comps: &HashMap<i32, Vec<String>>) -> bool {
    if message.len() % 8 != 0 {
        return false;
    }
    let mut i = 0;
    let mut count_42 = 0;
    let mut count_31 = 0;
    if !starts_with(comps.get(&42).unwrap(), message, i) {
        return false;
    }
    i = 8;
    while starts_with(comps.get(&42).unwrap(), message, i) {
        count_42 += 1;
        i += 8;
    }
    while starts_with(comps.get(&31).unwrap(), message, i) {
        count_31 += 1;
        i += 8;
    }
    if message.len() != i {
        return false;
    }
    if count_31 < 1 || count_42 < 1 {
        return false;
    }
    if count_31 > count_42 {
        return false;
    }
    return true;
}

fn main() {
    env_logger::init();
    info!("Start");
    info!("{:?}", solution_part_1("inputData.txt"));
    info!("{:?}", solution_part_2("inputData.txt"));
}
