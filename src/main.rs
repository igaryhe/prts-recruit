use arknights_tags_bot::{operator, query};
use serde_json;

fn main() {
    let file = include_str!("resources/operators.json");
    let operators: Vec<operator::Operator> = serde_json::from_str(file).unwrap();
    let mut query = query::Query::Affix(operator::Affix::DPS);
    let mut query_two = query::Query::Affix(operator::Affix::Defense);
    let result = query.exec(&operators);
    let result_two = query_two.exec(&operators);
    let joined = join(&result, &result_two);
    joined.iter().for_each(|i| println!("{}", operators[*i].get_name()));
}

fn join(query_one: &Vec<usize>, query_two: &Vec<usize>) -> Vec<usize> {
    let mut result = Vec::<usize>::new();
    let mut i = 0;
    let mut j = 0;
    while i < query_one.len() && j < query_two.len() {
        if query_one[i] == query_two[j] {
            result.push(query_one[i]);
            i = i + 1;
            j = j + 1;
        } else if query_one[i] < query_two[j] {
            i = i + 1;
        } else {
            j = j + 1;
        }
    }
    result
}