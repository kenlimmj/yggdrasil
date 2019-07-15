use clap::{crate_authors, crate_version, App, AppSettings};
use maplit::hashmap;
use yggdrasil::{
    matchers::{expense, MatcherRegistry},
    services::ServiceName,
};

const SETTINGS_DEFAULT: &[AppSettings] = &[
    AppSettings::ColoredHelp,
    AppSettings::SubcommandRequiredElseHelp,
    AppSettings::UnifiedHelpMessage,
];
const NAME_DISPLAY: &str = "Yggdrasil";
const NAME_BINARY: &str = "ygg";

fn get_app_base() -> App<'static, 'static> {
    App::new(NAME_DISPLAY)
        .alias(NAME_BINARY)
        .author(crate_authors!())
        .bin_name(NAME_BINARY)
        .settings(SETTINGS_DEFAULT)
        .version(crate_version!())
}

fn main() {
    let expense_match_handlers = expense::get_match_handlers();
    let matcher_registry = MatcherRegistry {
        matched_services: hashmap! {
            ServiceName::Expense => expense_match_handlers
        },
    };

    let app = get_app_base().subcommands(matcher_registry.get_matched_commands());
    match matcher_registry.get_match_handler(&app.get_matches()) {
        Some(handler_fn) => handler_fn(),
        None => ::std::process::exit(0),
    }
}
