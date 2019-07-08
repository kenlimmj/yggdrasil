use clap::ArgMatches;
use std::collections::HashMap;

use crate::{
    matchers::{ArgMatcher, MatchResult},
    operations::OperationType,
};

pub fn get_matchers() -> HashMap<OperationType, ArgMatcher> {
    let mut matcher_map = HashMap::new();
    matcher_map.insert(OperationType::Add, add_matcher as ArgMatcher);
    matcher_map.insert(OperationType::List, list_matcher as ArgMatcher);
    matcher_map
}

fn add_matcher(arg_matches: &ArgMatches) -> MatchResult {
    Ok(())
}

fn list_matcher(arg_matches: &ArgMatches) -> MatchResult {
    Ok(())
}
