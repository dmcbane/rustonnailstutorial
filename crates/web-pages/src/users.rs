use crate::layout::Layout;
use db::User;
use dioxus::prelude::*;
use dioxus::prelude::component;
use web_assets::files::avatar_svg;

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
                                img {
                                    src: format!("/static/{}", avatar_svg.name),
                                    width: "16",
                                    height: "16"
                                }
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

            // 👇 this is our new form
            form {
                action: "/sign_up",
                method: "POST",
                label { r#for: "user_email", "Email:" }
                input { id: "user_email", name: "email", r#type: "email", required: "true" }
                button { "Submit" }
            }
        }
    }
}
