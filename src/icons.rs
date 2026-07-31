//! Inline SVG icons, so the page pulls no icon font from a CDN.

use dioxus::prelude::*;

#[component]
pub fn Mail() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.7",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            aria_hidden: "true",
            rect { x: "2", y: "4", width: "20", height: "16", rx: "2" }
            path { d: "m2 7 10 6 10-6" }
        }
    }
}

#[component]
pub fn Phone() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.7",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            aria_hidden: "true",
            path { d: "M22 16.9v3a2 2 0 0 1-2.2 2 19.8 19.8 0 0 1-8.6-3.1 19.5 19.5 0 0 1-6-6A19.8 19.8 0 0 1 2.1 4.2 2 2 0 0 1 4.1 2h3a2 2 0 0 1 2 1.7c.1 1 .4 1.9.7 2.8a2 2 0 0 1-.5 2.1L8.1 9.9a16 16 0 0 0 6 6l1.3-1.3a2 2 0 0 1 2.1-.4c.9.3 1.8.6 2.8.7a2 2 0 0 1 1.7 2z" }
        }
    }
}

#[component]
pub fn Pin() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.7",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            aria_hidden: "true",
            path { d: "M20 10c0 6-8 12-8 12s-8-6-8-12a8 8 0 0 1 16 0z" }
            circle { cx: "12", cy: "10", r: "3" }
        }
    }
}

#[component]
pub fn Download() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.7",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            aria_hidden: "true",
            path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
            path { d: "m7 10 5 5 5-5" }
            path { d: "M12 15V3" }
        }
    }
}

#[component]
pub fn Arrow() -> Element {
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.7",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            aria_hidden: "true",
            path { d: "M7 17 17 7" }
            path { d: "M7 7h10v10" }
        }
    }
}
