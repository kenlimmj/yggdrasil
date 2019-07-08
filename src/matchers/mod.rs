pub mod expense;

use clap::{App, Arg, ArgMatches, SubCommand};
use std::collections::HashMap;
use std::str::FromStr;

use crate::{operations::OperationType, services::ServiceName};

pub enum MatchError {
    OperationNotSupported,
    ServiceNotFound,
}

pub type MatchResult = Result<(), MatchError>;

pub type ArgMatcher = fn(&ArgMatches) -> MatchResult;

pub struct MatcherRegistry {
    pub matched_services: HashMap<ServiceName, HashMap<OperationType, ArgMatcher>>,
}

impl MatcherRegistry {
    pub fn get_matched_commands(&self) -> Vec<App<'static, 'static>> {
        let mut commands = Vec::new();
        for (service_name, op_map) in &self.matched_services {
            let values: Vec<&'static str> = op_map.keys().map(|s| s.into()).collect();
            let command_name: &'static str = service_name.into();
            let arg = Arg::with_name("service")
                .required(true)
                .case_insensitive(true)
                .possible_values(values.as_slice())
                .value_name("service");
            commands.push(SubCommand::with_name(&command_name.to_lowercase()).arg(arg));
        }
        commands
    }

    pub fn r#match(&self, arg_matches: &ArgMatches) -> MatchResult {
        let op_type_str = arg_matches.subcommand_name();
        if op_type_str.is_none() {
            return Err(MatchError::OperationNotSupported);
        }

        if let (service_name_str, Some(child_match)) = arg_matches.subcommand() {
            let service_name =
                ServiceName::from_str(service_name_str).unwrap_or_else(|_| ServiceName::Unknown);
            let op_type = OperationType::from_str(op_type_str.unwrap())
                .unwrap_or_else(|_| OperationType::Unknown);
            return self
                .matched_services
                .get(&service_name)
                .unwrap()
                .get(&op_type)
                .unwrap()(child_match);
        }

        Err(MatchError::ServiceNotFound)
    }
}
