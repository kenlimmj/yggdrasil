use crate::services::{Name, Service};
use clap::{App, Arg, SubCommand};
use multimap::MultiMap;
use strum_macros::IntoStaticStr;

#[derive(PartialEq, Eq, Hash, IntoStaticStr)]
pub enum Type {
    Add,
    Link,
    List,
    Remove,
    Unknown,
}

pub struct Manager {
    service_map: MultiMap<Type, Name>,
}

impl Manager {
    pub fn new(services: &[&dyn Service]) -> Self {
        let service_map: MultiMap<Type, Name> = services
            .iter()
            .flat_map(|s| {
                s.get_supported_op_types()
                    .into_iter()
                    .map(move |op_type| (op_type, s.name()))
            })
            .collect();
        Manager { service_map }
    }

    pub fn get_op_commands(&self) -> Vec<App<'static, 'static>> {
        self.service_map
            .iter_all()
            .map(|(op_type, service_names)| {
                let name: &'static str = op_type.into();
                SubCommand::with_name(&name.to_lowercase())
                    .arg(Manager::make_command_arg(service_names))
            })
            .collect()
    }

    pub fn make_command_arg(service_names: &[Name]) -> Arg<'static, 'static> {
        let values: Vec<&'static str> = service_names.iter().map(|s| s.into()).collect();
        Arg::with_name("service")
            .required(true)
            .case_insensitive(true)
            .possible_values(values.as_slice())
            .value_name("service")
    }
}
