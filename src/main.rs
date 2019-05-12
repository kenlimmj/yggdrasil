extern crate clap;

use clap::{arg_enum, App, Arg, SubCommand};

arg_enum! {
    // TODO(kenlimmj): Read this out of the protos.
    #[derive(PartialEq, Debug)]
    enum ItemType {
        EXPENSE,
        BUDGET,
        TODO,
    }
}

fn main() {
    let item_type = Arg::with_name("item_type")
        .required(true)
        .case_insensitive(true)
        .possible_values(&ItemType::variants())
        .value_name("ITEM TYPE");

    let app_m = App::new("Yggdrasil")
        .subcommand(SubCommand::with_name("add").arg(item_type.clone()))
        .subcommand(SubCommand::with_name("list").arg(item_type.clone()))
        .get_matches();

    match app_m.subcommand() {
        ("add", Some(sub_m)) => println!("{:#?}", sub_m.value_of("item_type")),
        ("list", Some(sub_m)) => println!("{:#?}", sub_m.value_of("item_type")),
        _ => {}
    }
}
