//! All site copy lives here so editing the site never means touching component code.

pub struct Link {
    pub label: &'static str,
    pub href: &'static str,
}

pub struct Role {
    pub company: &'static str,
    pub title: &'static str,
    pub location: &'static str,
    pub period: &'static str,
    pub blurb: &'static str,
    pub tags: &'static [&'static str],
}

pub struct Project {
    pub name: &'static str,
    pub blurb: &'static str,
    pub tags: &'static [&'static str],
    pub href: Option<&'static str>,
}

pub const NAME: &str = "John J. Park";
pub const ROLE: &str = "Software Engineer";
pub const LOCATION: &str = "Ithaca, NY";
pub const EMAIL: &str = "jjp282@cornell.edu";
pub const PHONE: &str = "(646) 461 0807";

// The <meta> description and share-card text live in `index.html`, not here:
// crawlers read them before the WebAssembly bundle ever runs, so they have to
// be present in the served HTML rather than injected at hydration.

pub const INTRO: &str = "I build software for the web — mostly backend systems and the \
    occasional interface to go with them. I studied Computer Science at Cornell.";

pub const ABOUT: &[&str] = &[
    "I care about the unglamorous parts of engineering: clear data models, systems that fail \
     loudly instead of quietly, and code the next person can read without a walkthrough.",
    "This site is written in Rust and compiled to WebAssembly with Dioxus — partly because it \
     is a static site that did not need a framework at all, and partly because that made it a \
     more interesting thing to build.",
];

pub const SOCIALS: &[Link] = &[
    Link {
        label: "GitHub",
        href: "https://github.com/parkjmjohn",
    },
    Link {
        label: "LinkedIn",
        href: "https://www.linkedin.com/in/john-j-park-836914145/",
    },
    Link {
        label: "Twitter",
        href: "https://twitter.com/jpsportsgeek",
    },
];

pub const ROLES: &[Role] = &[Role {
    company: "Asymm Developers",
    title: "Technical Product Manager Intern",
    location: "San Diego, CA",
    period: "Internship",
    blurb: "Worked with the engineering team to scope and ship client web projects, \
                translating product requirements into technical work.",
    tags: &["Product", "Web"],
}];

// TODO(john): replace these with real projects — they render as-is on the live site.
pub const PROJECTS: &[Project] = &[Project {
    name: "This site",
    blurb: "A portfolio written in Rust, compiled to WebAssembly, and served from \
                GitHub Pages with no JavaScript framework in the bundle.",
    tags: &["Rust", "Dioxus", "WebAssembly"],
    href: Some("https://github.com/parkjmjohn/parkjmjohn.github.io"),
}];
