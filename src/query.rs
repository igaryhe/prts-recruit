use crate::operator;

pub enum Query {
    Qualification(operator::Qualification),
    Position(operator::Position),
    Class(operator::Class),
    Affix(operator::Affix),
}

impl<'a> Query {

    pub fn exec(&mut self, list: &Vec<operator::Operator>) -> Vec<usize> {
        let mut result = Vec::<usize>::new();
        match &self {
            Self::Qualification(qualification) => {
                list.iter().enumerate().for_each(|(pos, operator)| {
                    if let Some(q) = operator.get_qualification() {
                        if *qualification == q { result.push(pos) }
                    }
                });
            },
            Self::Position(position) => {
                list.iter().enumerate().for_each(|(pos, operator)| {
                    if *position == operator.get_position() { result.push(pos) }
                });
            },
            Self::Class(class) => {
                list.iter().enumerate().for_each(|(pos, operator)| {
                    if *class == operator.get_class() { result.push(pos) }
                });
            },
            Self::Affix(affix) => {
                list.iter().enumerate().for_each(|(pos, operator)| {
                    operator.get_affix().iter().for_each(|a| if affix == a { result.push(pos) });
                });
            },
        }
        result
    }
}