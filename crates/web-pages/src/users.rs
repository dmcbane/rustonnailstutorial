use crate::layout::Layout;
use db::User;
use dioxus::prelude::*;
use dioxus::prelude::component;

// Define the properties for IndexPage
#[derive(Props, Clone, PartialEq)]  // Add Clone and PartialEq here
pub struct IndexPageProps {
    pub users: Vec<User>,
}

// Define the IndexPage component
#[component]
pub fn IndexPage(props: IndexPageProps) -> Element {
    rsx! {
        Layout {    // <-- Use our layout
            title: "Users Table",
            table {
                thead {
                    tr {
                        th { "ID" }
                        th { "Email" }
                    }
                }
                tbody {
                    for user in props.users {
                        tr {
                            td {
                                strong {
                                    "{user.id}"
                                }
                            }
                            td {
                                "{user.email}"
                            }
                        }
                    }
                }
            }
        }
    }
}