use super::{Name, Service};
use crate::operations::Type;
use clap::ArgMatches;

pub struct Budget;

impl Service for Budget {
    fn name(&self) -> Name {
        Name::Budget
    }

    fn apply(&self, _op_type: Type, _arg_matches: &ArgMatches) -> bool {
        true
    }

    fn get_supported_op_types(&self) -> Vec<Type> {
        vec![Type::Add, Type::List, Type::Remove]
    }
}
