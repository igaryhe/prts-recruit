pub mod operator;
pub mod bot;

use operator::{Tag, Operator};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn query(tag: Tag, ops: &Vec<operator::Operator>) -> HashSet<usize> {
    let mut result = HashSet::<usize>::new();
        match &tag {
            Tag::Starter | Tag::Senior | Tag::Top => {
                ops.into_iter().enumerate().for_each(|(pos, operator)| {
                    if let Some(q) = operator.get_qualification() {
                        if tag == q { result.insert(pos); }
                    }
                });
            },
            Tag::Melee | Tag::Ranged => {
                ops.into_iter().enumerate().for_each(|(pos, operator)| {
                    if tag == operator.get_position() { result.insert(pos); }
                });
            },
            Tag::Defender | Tag::Guard | Tag::Vanguard | Tag::Sniper |
            Tag::Caster | Tag::Medic | Tag::Supporter | Tag::Specialist => {
                ops.iter().enumerate().for_each(|(pos, operator)| {
                    if tag == operator.get_class() { result.insert(pos); }
                });
            },
            _ => {
                ops.iter().enumerate().for_each(|(pos, operator)| {
                    operator.get_affix().iter().for_each(|a| if tag == *a { result.insert(pos); });
                });
            },
        }
        result
}

fn comb(list: Vec<Tag>) -> Vec<Vec<Tag>> {
    match list.len() {
        1 => {
            list.into_iter().combinations(1).collect()
        },
        2 => {
            let mut one: Vec<_> = list.clone().into_iter().combinations(1).collect();
            let mut two: Vec<_> = list.into_iter().combinations(2).collect();
            two.append(&mut one);
            two
        },
        3..=5 => {
            let mut one: Vec<_> = list.clone().into_iter().combinations(1).collect();
            let mut two: Vec<_> = list.clone().into_iter().combinations(2).collect();
            let mut three: Vec<_> = list.into_iter().combinations(3).collect();
            two.append(&mut one);
            three.append(&mut two);
            three
        },
        _ => { vec![] },
    }
}

fn get_list(list: &Vec<Tag>, ops: &Vec<Operator>) -> HashMap<Tag, HashSet<usize>> {
    let mut result: HashMap<Tag, HashSet<usize>> = HashMap::new();
    list.into_iter().for_each(|tag| {
        let set = query(tag.clone(), ops);
        result.insert(tag.clone(), set);
    });
    result
}

fn join(tags: &Vec<Tag>, map: &HashMap<Tag, HashSet<usize>>) -> Vec<usize> {
    let mut top_included = false;
    let mut result: HashSet<usize> = map.get(&tags[0]).unwrap().clone();
    tags.into_iter().enumerate().for_each(|(pos, tag)| {
        if *tag == operator::Tag::Top {
            top_included = true;
        }
        if pos != 0 {
            result = result.intersection(map.get(&tag).unwrap()).copied().collect();
        }
    });
    if !top_included {
        result = result.into_iter().filter(|op| *op <= 70).collect();
    }
    let mut result_vec: Vec<usize> = result.into_iter().collect();
    result_vec.sort();
    result_vec.reverse();
    result_vec
}

fn filter(comb: Vec<Vec<Tag>>, list: HashMap<Tag, HashSet<usize>>, ops: &Vec<operator::Operator>) -> Vec<(Vec<Tag>, Vec<usize>)> {
    let mut result = vec![];
    comb.iter().for_each(|tags| {
        let joined = join(tags, &list);
        if !joined.is_empty() {
            result.push((tags.clone(), joined));   
        }
    });
    result.sort_by(|a, b| avg(&a.1, ops).partial_cmp(&avg(&b.1, ops)).unwrap());
    result.reverse();
    result.into_iter().filter(|(_, op)| avg(op, ops) > 4.1).collect()
}

pub fn process(tag_list: Vec<Tag>, ops: &Vec<operator::Operator>) -> Vec<(Vec<Tag>, Vec<usize>)> {
    let op_list = get_list(&tag_list, &ops);
    let comb = comb(tag_list);
    filter(comb, op_list, ops)
}

fn avg(list: &Vec<usize>, ops: &Vec<operator::Operator>) -> f32 {
    let mut sum = 0;
    let mut count = 0;
    list.into_iter().for_each(|i| {
        let r = ops[*i].get_rarity();
        if r >= 3 {
            sum = sum + ops[*i].get_rarity();
            count = count + 1;
        }
    });
    sum as f32 / count as f32
}

pub fn get_ops() -> Vec<operator::Operator> {
    let file = include_str!("resources/operators.json");
    serde_json::from_str(file).unwrap()
}

pub fn format_result(result: Vec<(Vec<Tag>, Vec<usize>)>, operators: &Vec<operator::Operator>) -> String {
    let mut output = String::new();
    result.into_iter().for_each(|(key, val)| {
        output.push_str(format!("{:?}: ", key).as_str());
        val.into_iter().for_each(|i| output.push_str(format!("{} ", operators[i]).as_str()));
        output.push_str(format!("\n").as_str());
    });
    output
}

pub fn parse_string(input: String) -> Vec<Tag> {
    let tag_list: Vec<_> = input.split_whitespace()
        .map(|word| serde_json::from_str(format!("\"{}\"", word).as_str()))
        .collect::<Vec<Result<Tag, _>>>();
    tag_list.into_iter().filter(|option| match option {
        Ok(_) => true,
        _ => false,
    }).map(|option| option.unwrap()).collect::<Vec<Tag>>()
}
