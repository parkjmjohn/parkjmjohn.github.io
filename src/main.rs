mod content;
mod icons;

use content::{ABOUT, EMAIL, INTRO, LOCATION, NAME, PHONE, PROJECTS, ROLE, ROLES, SOCIALS};
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const PROF_JPG: Asset = asset!("/assets/prof.jpg");
const RESUME: Asset = asset!("/assets/resume.pdf");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // The favicon link lives in index.html so it is present before hydration.
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div { class: "shell",
            TopBar {}
            Hero {}
            About {}
            Experience {}
            Projects {}
            Footer {}
        }
    }
}

/// Small header strip: wordmark on the left, theme switch on the right.
#[component]
fn TopBar() -> Element {
    // `None` means "follow the OS setting", which is also the pre-hydration default.
    let mut theme = use_signal(|| Option::<&'static str>::None);

    let flip = move |_| {
        // Anything that is not already dark flips to dark, including the
        // pre-hydration `None` state where the OS setting is in charge.
        let next = if matches!(theme(), Some("dark")) {
            "light"
        } else {
            "dark"
        };
        theme.set(Some(next));
        document::eval(&format!(
            "document.documentElement.dataset.theme = '{next}';
             try {{ localStorage.setItem('theme', '{next}'); }} catch (e) {{}}"
        ));
    };

    rsx! {
        header { class: "topbar",
            span { class: "topbar-mark", "{NAME}" }
            button {
                class: "theme-toggle",
                r#type: "button",
                aria_label: "Switch between light and dark theme",
                onclick: flip,
                "theme"
            }
        }
    }
}

#[component]
fn Hero() -> Element {
    rsx! {
        section { class: "hero",
            div {
                h1 { class: "hero-name", "{NAME}" }
                p { class: "hero-role", "{ROLE}" }
                p { class: "hero-intro", "{INTRO}" }
                div { class: "actions",
                    a {
                        class: "btn btn-primary",
                        href: RESUME,
                        target: "_blank",
                        rel: "noopener noreferrer",
                        icons::Download {}
                        "Résumé"
                    }
                    a { class: "btn", href: "mailto:{EMAIL}",
                        icons::Mail {}
                        "Get in touch"
                    }
                }
            }
            img {
                class: "hero-photo",
                src: PROF_JPG,
                alt: "{NAME}",
                width: "880",
                height: "880",
            }
        }
    }
}

#[component]
fn About() -> Element {
    rsx! {
        section { class: "section",
            h2 { class: "section-label", "About" }
            div { class: "prose",
                for para in ABOUT {
                    p { "{para}" }
                }
            }
            ul { class: "contact-list", style: "margin-top: 1.25rem;",
                li {
                    icons::Mail {}
                    a { href: "mailto:{EMAIL}", "{EMAIL}" }
                }
                li {
                    icons::Phone {}
                    a { href: "tel:+16464610807", "{PHONE}" }
                }
                li {
                    icons::Pin {}
                    span { "{LOCATION}" }
                }
            }
        }
    }
}

#[component]
fn Experience() -> Element {
    rsx! {
        section { class: "section",
            h2 { class: "section-label", "Experience" }
            for role in ROLES {
                div { class: "entry",
                    div { class: "entry-head",
                        h3 { class: "entry-title", "{role.company}" }
                        span { class: "entry-meta", "{role.location} · {role.period}" }
                    }
                    p { class: "entry-sub", "{role.title}" }
                    p { class: "entry-blurb", "{role.blurb}" }
                    Tags { items: role.tags }
                }
            }
        }
    }
}

#[component]
fn Projects() -> Element {
    rsx! {
        section { class: "section",
            h2 { class: "section-label", "Projects" }
            for project in PROJECTS {
                match project.href {
                    Some(href) => rsx! {
                        a {
                            class: "entry",
                            href,
                            target: "_blank",
                            rel: "noopener noreferrer",
                            div { class: "entry-head",
                                h3 { class: "entry-title", "{project.name}" }
                                span { class: "entry-meta", icons::Arrow {} }
                            }
                            p { class: "entry-blurb", "{project.blurb}" }
                            Tags { items: project.tags }
                        }
                    },
                    None => rsx! {
                        div { class: "entry",
                            div { class: "entry-head",
                                h3 { class: "entry-title", "{project.name}" }
                            }
                            p { class: "entry-blurb", "{project.blurb}" }
                            Tags { items: project.tags }
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn Tags(items: &'static [&'static str]) -> Element {
    if items.is_empty() {
        return rsx! {};
    }
    rsx! {
        div { class: "tags",
            for tag in items {
                span { class: "tag", "{tag}" }
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        footer { class: "footer",
            div {
                for (i, link) in SOCIALS.iter().enumerate() {
                    if i > 0 {
                        span { " · " }
                    }
                    a {
                        href: link.href,
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "{link.label}"
                    }
                }
            }
            span { "Rust + Dioxus \u{2192} WebAssembly" }
        }
    }
}
