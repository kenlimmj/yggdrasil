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
    let expense_matchers = expense::get_matchers();
    let matcher_registry = MatcherRegistry {
        matched_services: hashmap! {
            ServiceName::Expense => expense_matchers
        },
    };

    let app = get_app_base().subcommands(matcher_registry.get_matched_commands());
    matcher_registry.r#match(&app.get_matches());
}
