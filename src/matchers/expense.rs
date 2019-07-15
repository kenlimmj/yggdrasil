use chrono::{Duration, Local, NaiveDate};
use console::Style;
use dialoguer::{
    theme::{ColorfulTheme, Theme},
    Confirmation, Input, Select,
};
use maplit::hashmap;
use rust_decimal::Decimal;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, IntoStaticStr};

use crate::{matchers::MatchHandler, operations::OperationType};

#[derive(PartialEq, Eq, Hash, IntoStaticStr, EnumIter, Display, Copy, Clone)]
enum Currency {
    #[strum(disabled = "true")]
    Unknown,

    SGD,
    USD,
}

#[derive(PartialEq, Eq, Hash, IntoStaticStr, EnumIter, Display, Copy, Clone)]
enum Category {
    #[strum(disabled = "true")]
    Unknown,

    Food,
    Healthcare,
    Retirement,
    Savings,
    Shock,
    Transportation,
    Utilities,
}

#[derive(PartialEq, Eq, Hash, IntoStaticStr, EnumIter, Display, Copy, Clone)]
enum Curve {
    #[strum(disabled = "true")]
    Unknown,

    #[strum(serialize = "Straight line")]
    StraightLine,

    #[strum(serialize = "Double declining")]
    DoubleDeclining,

    #[strum(serialize = "Annuity")]
    Annuity,

    #[strum(serialize = "Sum of Years' Digits")]
    SumOfYearsDigits,

    #[strum(serialize = "Units of Production")]
    UnitsOfProduction,

    #[strum(serialize = "Units of Time")]
    UnitsOfTime,
}

struct Entry {
    category: Category,
    depreciation: Option<Depreciation>,
    recurrence_frequency: Duration,
    value: Value,
}

impl Default for Entry {
    fn default() -> Self {
        Entry {
            category: Category::Unknown,
            depreciation: Option::None,
            recurrence_frequency: Duration::zero(),
            value: Value::default(),
        }
    }
}

struct Value {
    amount: Decimal,
    currency: Currency,
    datetime: NaiveDate,
}

impl Default for Value {
    fn default() -> Self {
        Value {
            amount: Decimal::new(0, 0),
            currency: Currency::USD,
            datetime: Local::today().naive_local(),
        }
    }
}

struct Depreciation {
    curve: Curve,
    impairment_loss: Decimal,
    residual: Decimal,
    window: Duration,
}

impl Default for Depreciation {
    fn default() -> Self {
        Depreciation {
            curve: Curve::Unknown,
            impairment_loss: Decimal::new(0, 0),
            residual: Decimal::new(0, 0),
            window: Duration::zero(),
        }
    }
}

pub fn get_match_handlers() -> HashMap<OperationType, MatchHandler> {
    hashmap! {
        OperationType::Add => handle_add as MatchHandler,
        OperationType::List => handle_list as MatchHandler
    }
}

fn handle_add() {
    let theme = ColorfulTheme {
        yes_style: Style::new().green().dim(),
        no_style: Style::new().red().dim(),
        ..ColorfulTheme::default()
    };
    get_all_add_values(theme);
}

fn get_simple_add_values(theme: impl Theme) -> Option<Entry> {
    let amount = Input::<Decimal>::with_theme(&theme)
        .with_prompt("Item amount")
        .interact()
        .ok()?;

    let category_items: Vec<Category> = Category::iter().collect();
    let category_index = Select::with_theme(&theme)
        .with_prompt("Category")
        .default(0)
        .items(&category_items)
        .interact()
        .ok()?;

    Some(Entry {
        category: category_items[category_index],
        value: Value {
            amount,
            ..Value::default()
        },
        ..Entry::default()
    })
}

fn get_all_add_values(theme: impl Theme) -> Option<Entry> {
    let amount = Input::<Decimal>::with_theme(&theme)
        .with_prompt("Item amount")
        .interact()
        .ok()?;

    let currency_items: Vec<Currency> = Currency::iter().collect();
    let usd_index = currency_items.iter().position(|c| c == &Currency::USD)?;
    let currency_index = Select::with_theme(&theme)
        .with_prompt("Currency")
        .default(usd_index)
        .items(&currency_items)
        .interact()
        .ok()?;

    let date_incurred = Input::<NaiveDate>::with_theme(&theme)
        .with_prompt("Date incurred")
        .interact()
        .ok()?;

    let category_items: Vec<Category> = Category::iter().collect();
    let category_index = Select::with_theme(&theme)
        .with_prompt("Category")
        .default(0)
        .items(&category_items)
        .interact()
        .ok()?;

    let recurrence_frequency_days = Input::<i64>::with_theme(&theme)
        .with_prompt("Recurrence frequency (days)")
        .interact()
        .ok()?;

    Some(Entry {
        category: category_items[category_index],
        depreciation: get_depreciation_value(theme),
        recurrence_frequency: Duration::days(recurrence_frequency_days),
        value: Value {
            amount,
            currency: currency_items[currency_index],
            datetime: date_incurred,
        },
    })
}

fn get_depreciation_value(theme: impl Theme) -> Option<Depreciation> {
    let has_depreciation = Confirmation::with_theme(&theme)
        .with_text("Has depreciation?")
        .interact()
        .ok()?;
    if !has_depreciation {
        return Option::None;
    }

    let curve_items: Vec<Curve> = Curve::iter().collect();
    let curve_index = Select::with_theme(&theme)
        .with_prompt("Curve type")
        .default(0)
        .items(&curve_items)
        .interact()
        .ok()?;

    let impairment_loss = Input::<Decimal>::with_theme(&theme)
        .with_prompt("Impairment loss")
        .interact()
        .ok()?;

    let residual = Input::<Decimal>::with_theme(&theme)
        .with_prompt("Residual value")
        .interact()
        .ok()?;

    let window_days = Input::<i64>::with_theme(&theme)
        .with_prompt("Depreciation window (days)")
        .interact()
        .ok()?;

    Some(Depreciation {
        curve: curve_items[curve_index],
        impairment_loss,
        residual,
        window: Duration::days(window_days),
    })
}

fn handle_list() {}
