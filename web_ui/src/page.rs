use api_models::validator::Validate;
use seed::{prelude::*, *};

pub mod anonimous;
pub mod home;
pub mod not_found;
pub mod sepa_management;
pub mod user_profile;

fn view_validation_icon<Ms>(val: &impl Validate, key: &str) -> Node<Ms> {
    let class = if let Err(errors) = val.validate() {
        if errors.errors().contains_key(key) {
            "fa-exclamation-triangle"
        } else {
            "fa-check"
        }
    } else {
        "fa-check"
    };

    span![C!["icon", "is-small", "is-right"], i![C!["fas", class]]]
}
