extern crate clap;

use clap::{arg_enum, crate_authors, crate_version, App, AppSettings, Arg, SubCommand};

arg_enum! {
    // TODO(kenlimmj): Read this out of the protos.
    #[derive(PartialEq, Debug)]
    enum ItemType {
        EXPENSE,
        BUDGET,
        TODO,
    }
}

fn build_cli() -> App<'static, 'static> {
    let item = Arg::with_name("item")
        .required(true)
        .short("i")
        .long("")
        .case_insensitive(true)
        .possible_values(&ItemType::variants())
        .value_name("ITEM");

    let subcommands = vec![
        SubCommand::with_name("add").arg(item.clone()),
        SubCommand::with_name("list").arg(item.clone()),
        SubCommand::with_name("link").arg(item.clone()),
    ];

    App::new("Yggdrasil")
        .alias("ygg")
        .author(crate_authors!())
        .version(crate_version!())
        .settings(&[
            AppSettings::ColoredHelp,
            AppSettings::SubcommandRequiredElseHelp,
            AppSettings::UnifiedHelpMessage,
        ])
        .subcommands(subcommands)
}

fn main() {
    color_backtrace::install();

    match build_cli().get_matches().subcommand() {
        ("add", Some(sub_m)) => {}
        ("link", Some(sub_m)) => {}
        ("list", Some(sub_m)) => {}
        ("", None) => unreachable!(),
        _ => unreachable!(),
    }
}
