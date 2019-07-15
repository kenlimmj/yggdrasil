pub mod expense;

use clap::{App, Arg, ArgMatches, SubCommand};
use std::collections::HashMap;
use std::str::FromStr;
use strum::IntoEnumIterator;

use crate::{operations::OperationType, services::ServiceName};

const ARG_NAME_SERVICE: &str = "service";

pub type MatchHandler = fn();

pub struct MatcherRegistry {
    pub matched_services: HashMap<ServiceName, HashMap<OperationType, MatchHandler>>,
}

impl MatcherRegistry {
    pub fn get_matched_commands(&self) -> Vec<App<'static, 'static>> {
        OperationType::iter().fold(Vec::new(), |mut acc, op_type| {
            let values: Vec<&'static str> = self
                .matched_services
                .iter()
                .filter(|(_, op_map)| op_map.contains_key(&op_type))
                .map(|(service_name, _)| service_name.into())
                .collect();
            if values.is_empty() {
                return acc;
            }

            let arg = Arg::with_name(ARG_NAME_SERVICE)
                .required(true)
                .case_insensitive(true)
                .possible_values(values.as_slice())
                .value_name(ARG_NAME_SERVICE);
            acc.push(SubCommand::with_name(op_type.into()).arg(arg));
            acc
        })
    }

    pub fn get_match_handler(&self, arg_matches: &ArgMatches) -> Option<&MatchHandler> {
        let (op_type_str, service_name_matches) = arg_matches.subcommand();

        self.matched_services
            .get(&ServiceName::from_str(service_name_matches?.value_of(ARG_NAME_SERVICE)?).ok()?)?
            .get(&OperationType::from_str(op_type_str).ok()?)
    }
}
