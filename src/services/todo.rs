use super::{Name, Service};
use crate::operations::Type;
use clap::ArgMatches;

pub struct Todo;

impl Service for Todo {
    fn name(&self) -> Name {
        Name::Todo
    }

    fn apply(&self, _op_type: Type, _arg_matches: &ArgMatches) -> bool {
        true
    }

    fn get_supported_op_types(&self) -> Vec<Type> {
        vec![Type::Add, Type::List, Type::Remove]
    }
}
