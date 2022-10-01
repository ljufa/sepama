use crate::Model;
use seed::{prelude::*, *};

pub fn view<Ms>(model: &Model) -> Node<Ms> {
    match &model.user {
        Some(user) => view_user(user),
        None => view_anonymous(),
    }
}

fn view_anonymous<Ms>() -> Node<Ms> {
    section![
        C!["hero", "is-medium", "ml-6"],
        div![
            C!["hero-body", "has-text-centered"],
            h2![C!["subtitle", "is-size-3"], "Please login or register!"]
        ]
    ]
}

fn view_user<Ms>(user: &crate::User) -> Node<Ms> {
    section![
        C!["hero", "is-medium", "ml-6"],
        div![
            C!["hero-body"],
            h1![
                C!["title", "is-size-1"],
                format!("Hello {}! Welcome to SepaMa.", user.nickname),
            ],
            IF!(!user.email_verified =>
                div![
                    h2![
                        C!["subtitle", "is-size-3"],
                        format!("Please verify your email address {}", user.email)
                    ],
                ]
            ),
        ]
    ]
}
