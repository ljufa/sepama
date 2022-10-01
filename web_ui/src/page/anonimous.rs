use seed::{prelude::*, *};

pub fn view<Ms>() -> Node<Ms> {
    section![
        C!["hero", "is-medium", "ml-6"],
        div![
            C!["hero-body", "has-text-centered"],
            h2![C!["subtitle", "is-size-3"], "Please login or register!"]
        ]
    ]
}
