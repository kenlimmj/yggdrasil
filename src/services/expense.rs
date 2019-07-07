use super::{Name, Service};
use crate::operations::Type;
use clap::ArgMatches;

pub struct Expense;

impl Service for Expense {
    fn name(&self) -> Name {
        Name::Expense
    }

    fn apply(&self, _op_type: Type, _arg_matches: &ArgMatches) -> bool {
        true
    }

    fn get_supported_op_types(&self) -> Vec<Type> {
        vec![Type::Add, Type::List, Type::Remove]
    }
}
