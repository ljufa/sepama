use crate::{page::view_validation_icon, api_client};

use api_models::{
    models::{BankAccount, Mandate, Status},
    validator::Validate,
};
use seed::{prelude::*, *};
use uuid::Uuid;

// ------ ------~
//     Model
// ------ ------
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Confirmation {
    Uncomfirmed,
    ConfirmedYes,
    ConfirmedNo,
}

pub struct Model {
    mandates: Vec<Mandate>,
    selected_mandate: Option<Mandate>,
    unsaved_changes_confirmation: Option<Confirmation>,
    remote_call_in_progress: bool,
}
impl Model {
    fn get_bank_accounts(&self) -> Vec<BankAccount> {
        let mut all_bank_accounts: Vec<BankAccount> = self
            .mandates
            .iter()
            .map(|m| m.bank_account.clone())
            .collect();
        all_bank_accounts.sort();
        all_bank_accounts.dedup();
        all_bank_accounts
    }
    fn find_mandate_by_id(&self, id: Uuid) -> Option<&Mandate> {
        self.mandates.iter().find(|m| m.api_id == id)
    }

    fn is_selected_mandate_modified(&self) -> bool {
        if let Some(sm) = &self.selected_mandate {
            self.find_mandate_by_id(sm.api_id)
                .map_or(false, |m| m != sm)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub enum Msg {
    MandatesFetched(fetch::Result<Vec<Mandate>>),
    MandateItemSelected(Uuid),
    DisplayNameChanged(String),
    MandateReferenceChanged(String),
    BankAccountChanged(String),
    CreditorNameChanged(String),
    CreditorSepaIdentChanged(String),
    CreditorStreetChanged(String),
    CreditorHouseNoChanged(String),
    CreditorZipChanged(String),
    CreditorPlaceChanged(String),
    SaveSelectedMandate(Status),
    SelectedMandateSaved(Mandate, fetch::Result<Response>),
    DebtorBankAccountInstitutionChanged(String),
    DebtorBankAccountIbanChanged(String),
    DebtorBankAccountBicChanged(String),
    NewMandateClicked,
    ConfirmUnsavedChanges(Confirmation),
}

//--------
//     Init
// ------ ------

pub fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    log!("Init manage");
    orders.perform_cmd(async { Msg::MandatesFetched(api_client::request_mandates().await) });
    Model {
        mandates: Vec::new(),
        selected_mandate: None,
        remote_call_in_progress: true,
        unsaved_changes_confirmation: None,
    }
}

// ------ ------
//     Update
// ------ ------
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    log!("Update manage", msg);
    match msg {
        Msg::MandatesFetched(result) => {
            model.remote_call_in_progress = false;
            model.mandates = result.map_or(Vec::new(), |r| r)
        }

        Msg::MandateItemSelected(mid) => {
            if model
                .unsaved_changes_confirmation
                .map_or(true, |f| f == Confirmation::Uncomfirmed)
                && model.is_selected_mandate_modified()
            {
                model.unsaved_changes_confirmation = Some(Confirmation::Uncomfirmed)
            } else if let Some(mandate) = model.find_mandate_by_id(mid) {
                model.selected_mandate = Some(mandate.clone());
                // model.unsaved_changes_confirmation = Some(Confirmation::Uncomfirmed);
            }
        }

        Msg::SaveSelectedMandate(status) => {
            model.selected_mandate.as_mut().map(|sm| sm.status = status);
            
            model.selected_mandate.clone().map(|sm| {
                orders.perform_cmd(async move {
                    Msg::SelectedMandateSaved(sm.clone(), api_client::save_selected_mandate(sm).await)
                });
            });
        }

        Msg::SelectedMandateSaved(mandate, m) => match m {
            Ok(_) => {
                model
                    .mandates
                    .iter_mut()
                    .filter(|i| i.api_id == mandate.api_id)
                    .next()
                    .map(|m| {
                        *m = mandate;
                    });
            }
            Err(e) => log!(e),
        },

        Msg::NewMandateClicked => {
            let mut new_mandate = Mandate::default();
            new_mandate.api_id = Uuid::new_v4();
            let id = new_mandate.api_id;
            new_mandate.display_name = "new...".to_string();
            model.mandates.push(new_mandate);
            orders.perform_cmd(async move { Msg::MandateItemSelected(id.clone()) });
        }

        Msg::DisplayNameChanged(dname) => {
            model
                .selected_mandate
                .as_mut()
                .map(|sm| sm.display_name = dname);
        }

        Msg::MandateReferenceChanged(value) => {
            model
                .selected_mandate
                .as_mut()
                .map(|sm| sm.unique_reference = Some(value));
        }

        Msg::BankAccountChanged(selected) => {
            model.selected_mandate.as_mut().map(|sm| {
                if selected == "ADDNEW" {
                    sm.bank_account = BankAccount::default();
                } else {
                    model
                        .mandates
                        .iter()
                        .filter(|a| a.bank_account.iban == selected)
                        .map(|f| f.bank_account.clone())
                        .next()
                        .map(|bac| sm.bank_account = bac);
                }
            });
        }

        Msg::CreditorNameChanged(value) => {
            model
                .selected_mandate
                .as_mut()
                .map(|sm| sm.creditor.name = value);
        }

        Msg::CreditorSepaIdentChanged(value) => {
            model
                .selected_mandate
                .as_mut()
                .map(|sm| sm.creditor.sepa_identifier = Some(value));
        }

        Msg::CreditorStreetChanged(value) => {
            model
                .selected_mandate
                .as_mut()
                .map(|sm| sm.creditor.address.street = value);
        }

        Msg::CreditorHouseNoChanged(value) => {
            model
                .selected_mandate
                .as_mut()
                .map(|sm| sm.creditor.address.house_number = value);
        }

        Msg::CreditorZipChanged(value) => {
            model
                .selected_mandate
                .as_mut()
                .map(|sm| sm.creditor.address.zip = value);
        }

        Msg::CreditorPlaceChanged(value) => {
            model
                .selected_mandate
                .as_mut()
                .map(|sm| sm.creditor.address.place = value);
        }

        Msg::DebtorBankAccountInstitutionChanged(value) => {
            model
                .selected_mandate
                .as_mut()
                .map(|sm| sm.bank_account.institution = value);
        }

        Msg::DebtorBankAccountIbanChanged(value) => {
            model
                .selected_mandate
                .as_mut()
                .map(|sm| sm.bank_account.iban = value);
        }

        Msg::DebtorBankAccountBicChanged(value) => {
            model
                .selected_mandate
                .as_mut()
                .map(|sm| sm.bank_account.bic = Some(value));
        }
        Msg::ConfirmUnsavedChanges(conf) => model.unsaved_changes_confirmation = Some(conf),
    };
}

// ------ ------~
//     View
// ------ ------

pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["columns"],
        view_mandate_list(model),
        view_selected_mandate_details(model)
    ]
}

fn view_mandate_list(model: &Model) -> Node<Msg> {
    div![
        C!["column", "is-two-fifths"],
        nav![
            C!["panel is-info"],
            // search mandates input field
            div![
                C!["panel-block"],
                p![
                    C!["control has-icons-left"],
                    input![
                        C!["input"],
                        attrs! {At::Value => "", At::Type => "text", At::Placeholder => "Search mandates"},
                    ],
                    span![C!["icon", "is-left"], i![C!["fas", "fa-search"]]],
                ],
            ],
            div![
                C!["scrollable-list"],
                if model.remote_call_in_progress {
                    progress![C!["progress", "is-link", "mt-6"]].into_nodes()
                } else {
                    model
                        .mandates
                        .iter()
                        .map(|mandate| view_mandate_list_item(model, mandate))
                        .collect()
                }
            ],
            div![
                C!["panel-block"],
                button![
                    C!["button", "is-link", "is-outlined", "is-fullwidth"],
                    ev(Ev::Click, |_| Msg::NewMandateClicked),
                    "Create New Mandate"
                ]
            ]
        ]
    ]
}

fn view_mandate_list_item(model: &Model, mandate: &Mandate) -> Node<Msg> {
    let selected_mandate = model.selected_mandate.as_ref();
    let active = selected_mandate.map_or(false, |sm| mandate.api_id == sm.api_id);
    let api_id = mandate.api_id.clone();
    a![
        C![
            "panel-block",
            IF!(active => "has-text-weight-semibold is-italic")
        ],
        span![C!["panel-icon"], i![C!["fas"]]],
        ev(Ev::Click, move |_| Msg::MandateItemSelected(api_id)),
        &mandate.display_name,
    ]
}

fn view_selected_mandate_details(model: &Model) -> Node<Msg> {
    if let Some(mandate) = &model.selected_mandate {
        div![
            C!["column"],
            div![
                C![
                    "modal",
                    IF!(model.unsaved_changes_confirmation.as_ref().map_or(false, |f| f.eq(&Confirmation::Uncomfirmed)) => "is-active")
                ],
                div![C!["modal-background"]],
                div![
                    C!["modal-content"],
                    div![
                        C!["block"],
                        div![
                            C!["notification"],
                            p![C!["title"], "Your changes will be lost! Are you sure?"]
                        ]
                    ],
                ],
                div![
                    C!["buttons"],
                    button![
                        C!["button", "is-warning"],
                        "Yes",
                        ev(Ev::Click, |_| Msg::ConfirmUnsavedChanges(
                            Confirmation::ConfirmedYes
                        ))
                    ],
                    button![
                        C!["button", "is-success"],
                        "No",
                        ev(Ev::Click, |_| Msg::ConfirmUnsavedChanges(
                            Confirmation::ConfirmedNo
                        ))
                    ],
                ]
            ],
            div![
                C!["field"],
                label![C!["label"], "Display Name"],
                div![
                    C!["control", "has-icons-right"],
                    input![
                        C!["input", "is-success"],
                        attrs! {
                            At::Value => mandate.display_name
                        },
                        input_ev(Ev::Input, move |dname| { Msg::DisplayNameChanged(dname) }),
                    ],
                    view_validation_icon(mandate, "display_name"),
                ]
            ],
            div![
                C!["field"],
                label![C!["label"], "Unique mandate reference (optional)"],
                div![
                    C!["control", "has-icons-right"],
                    input![
                        C!["input", "is-success"],
                        attrs! {
                            At::Value => mandate.unique_reference.clone().unwrap_or_default()
                        },
                        input_ev(Ev::Input, move |value| {
                            Msg::MandateReferenceChanged(value)
                        }),
                    ],
                    view_validation_icon(mandate, "unique_reference"),
                ]
            ],
            // Creditor
            div![
                C!["box"],
                label![C!["label"], "Creditor information"],
                nav![
                    C!["level"],
                    div![
                        C!["level-left"],
                        // Name
                        div![
                            C!["level-item"],
                            div![
                                C!["field"],
                                label![C!["label"], "Name"],
                                div![
                                    C!["control", "has-icons-left has-icons-right"],
                                    input![
                                        C!["input", "is-success"],
                                        attrs! {
                                            At::Value => mandate.creditor.name
                                        },
                                        input_ev(Ev::Input, move |value| {
                                            Msg::CreditorNameChanged(value)
                                        }),
                                    ],
                                    span![
                                        C!["icon", "is-small", "is-left"],
                                        i![C!["fas", "fa-university"]]
                                    ],
                                    view_validation_icon(&mandate.creditor, "name"),
                                ]
                            ],
                        ],
                        // creditor identifier
                        div![
                            C!["level-item"],
                            div![
                                C!["field"],
                                label![C!["label"], "Sepa identifier (Optional)"],
                                div![
                                    C!["control"],
                                    input![
                                        C!["input"],
                                        attrs! {
                                            At::Value => mandate.creditor.sepa_identifier.as_ref().map_or("",|m|m)
                                        }
                                    ],
                                    input_ev(Ev::Input, move |value| {
                                        Msg::CreditorSepaIdentChanged(value)
                                    }),
                                ]
                            ]
                        ]
                    ]
                ],
                // address
                nav![
                    C!["level"],
                    div![
                        C!["level-left"],
                        // Street
                        div![
                            C!["level-item"],
                            div![
                                C!["field"],
                                label![C!["label"], "Street"],
                                div![
                                    C!["control", "has-icons-left has-icons-right"],
                                    input![
                                        C!["input", "is-success"],
                                        attrs! {
                                            At::Value => mandate.creditor.address.street
                                        },
                                        input_ev(Ev::Input, move |value| {
                                            Msg::CreditorStreetChanged(value)
                                        }),
                                    ],
                                    span![
                                        C!["icon", "is-small", "is-left"],
                                        i![C!["fas", "fa-map-marker-alt"]]
                                    ],
                                    view_validation_icon(&mandate.creditor.address, "street"),
                                ]
                            ],
                        ],
                        // Hause number
                        div![
                            C!["level-item"],
                            div![
                                C!["field"],
                                label![C!["label"], "House number"],
                                div![
                                    C!["control", "has-icons-left has-icons-right"],
                                    input![
                                        C!["input"],
                                        attrs! {
                                            At::Value => mandate.creditor.address.house_number
                                        },
                                        input_ev(Ev::Input, move |value| {
                                            Msg::CreditorHouseNoChanged(value)
                                        }),
                                    ],
                                    span![
                                        C!["icon", "is-small", "is-left"],
                                        i![C!["fas", "fa-map-marker-alt"]]
                                    ],
                                    view_validation_icon(&mandate.creditor.address, "house_number"),
                                ]
                            ]
                        ]
                    ]
                ],
                nav![
                    C!["level"],
                    div![
                        C!["level-left"],
                        // Zip code
                        div![
                            C!["level-item"],
                            div![
                                C!["field"],
                                label![C!["label"], "Zip"],
                                div![
                                    C!["control", "has-icons-left has-icons-right"],
                                    input![
                                        C!["input", "is-success"],
                                        attrs! {
                                            At::Value => mandate.creditor.address.zip
                                        },
                                        input_ev(Ev::Input, move |value| {
                                            Msg::CreditorZipChanged(value)
                                        }),
                                    ],
                                    span![
                                        C!["icon", "is-small", "is-left"],
                                        i![C!["fas", "fa-map-marker-alt"]]
                                    ],
                                    view_validation_icon(&mandate.creditor.address, "zip"),
                                ]
                            ],
                        ],
                        // Place
                        div![
                            C!["level-item"],
                            div![
                                C!["field"],
                                label![C!["label"], "Place"],
                                div![
                                    C!["control", "has-icons-left has-icons-right"],
                                    input![
                                        C!["input"],
                                        attrs! {
                                            At::Value => mandate.creditor.address.place
                                        },
                                        input_ev(Ev::Input, move |value| {
                                            Msg::CreditorPlaceChanged(value)
                                        }),
                                    ],
                                    span![
                                        C!["icon", "is-small", "is-left"],
                                        i![C!["fas", "fa-map-marker-alt"]]
                                    ],
                                    view_validation_icon(&mandate.creditor.address, "place"),
                                ]
                            ]
                        ]
                    ]
                ],
            ],
            // Debtor bank account selection
            div![
                C!["box"],
                div![
                    C!["field"],
                    label![C!["label"], "Bank account"],
                    div![
                        C!["control"],
                        div![
                            C!["select"],
                            select![
                                model.get_bank_accounts().iter().map(|m| {
                                    option![
                                        if m.iban == mandate.bank_account.iban {
                                            attrs! {At::Selected => ""}
                                        } else {
                                            attrs! {At::Alt => ""}
                                        },
                                        attrs! {At::Value => m.iban.clone()},
                                        format!("{} - {}", m.institution, m.iban)
                                    ]
                                }),
                                // option![attrs! {At::Value=>""}, ""],
                                option![
                                    attrs! {At::Value=>"ADDNEW"},
                                    "--- Add New Bank Account ---"
                                ],
                                input_ev(Ev::Change, move |selected| Msg::BankAccountChanged(
                                    selected
                                )),
                            ],
                        ],
                    ],
                ],
                div![
                    div![
                        C!["field"],
                        label![C!["label"], "Bank/Institution"],
                        div![
                            C!["control", "has-icons-right"],
                            input![
                                C!["input", "is-success"],
                                attrs! {
                                    At::Value => mandate.bank_account.institution
                                },
                                input_ev(Ev::Input, move |value| {
                                    Msg::DebtorBankAccountInstitutionChanged(value)
                                }),
                            ],
                            view_validation_icon(&mandate.bank_account, "insitution"),
                        ]
                    ],
                    div![
                        C!["field"],
                        label![C!["label"], "IBAN"],
                        div![
                            C!["control", "has-icons-right"],
                            input![
                                C!["input", "is-success"],
                                attrs! {
                                    At::Value => mandate.bank_account.iban,
                                    At::Placeholder => "DE____________________"
                                },
                                input_ev(Ev::Input, move |value| {
                                    Msg::DebtorBankAccountIbanChanged(value)
                                }),
                            ],
                            view_validation_icon(&mandate.bank_account, "iban"),
                        ]
                    ],
                    div![
                        C!["field"],
                        label![C!["label"], "BIC (Optional)"],
                        div![
                            C!["control", "has-icons-right"],
                            input![
                                C!["input", "is-success"],
                                attrs! {
                                    At::Value => mandate.bank_account.bic.clone().unwrap_or_default(),

                                },
                                input_ev(Ev::Input, move |value| {
                                    Msg::DebtorBankAccountBicChanged(value)
                                }),
                            ],
                            view_validation_icon(&mandate.bank_account, "bic"),
                        ]
                    ],
                ],
            ],
            div![
                C!["field"],
                label![
                    C!["label"],
                    format!(
                        "Date: {}",
                        mandate.date_created.as_ref().unwrap_or(&String::new())
                    )
                ],
            ],
            div![
                C!["field"],
                label![C!["label"], format!("Status: {:?}", mandate.status)],
            ],
            // buttons
            div![
                C!["field", "is-grouped", "section"],
                div![
                    C!["control"],
                    button![
                        IF!(mandate.validate().is_err() => attrs!{ At::Disabled => ""}),
                        C!["button", "is-success"],
                        "Save",
                        ev(Ev::Click, |_| Msg::SaveSelectedMandate(Status::ACTIVE)),
                    ]
                ],
                div![
                    C!["control"],
                    button![
                        C!["button", "is-danger"],
                        "Delete",
                        ev(Ev::Click, |_| Msg::SaveSelectedMandate(Status::DELETED)),
                    ]
                ],
                div![
                    C!["control"],
                    button![
                        C!["button", "is-warning"],
                        "Print cancelation form",
                        ev(Ev::Click, |_| Msg::SaveSelectedMandate(Status::CANCELED)),
                    ]
                ],
            ]
        ]
    } else {
        section![
            C!["hero", "is-medium", "ml-6"],
            div![
                C!["hero-body", "has-text-centered"],
                h2![
                    C!["subtitle", "is-size-3"],
                    "Please select or create mandate."
                ]
            ]
        ]
    }
}

