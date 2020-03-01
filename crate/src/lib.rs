#![feature(track_caller)]
// @TODO: uncomment once https://github.com/rust-lang/rust/issues/54726 stable
//#![rustfmt::skip::macros(class)]
#![allow(clippy::used_underscore_binding)]
#![allow(clippy::non_ascii_literal)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::must_use_candidate)]

mod generated;
mod page;

use comp_state::{
    clone_state_with_topo_id, execute_and_remove_drop_types, topo, StateAccess,
    StateAccessDropType,
};
use comp_state_seed_extras::handle_drop_types;
use fixed_vec_deque::FixedVecDeque;
use generated::css_classes::C;
use seed::{prelude::*, *};
use wasm_bindgen::JsCast;
const TITLE_SUFFIX: &str = "seedhooks";
// https://mailtolink.me/
// const MAIL_TO_REBO: &str = "mailto:rebotfc@gmail.com";
// const USER_AGENT_FOR_PRERENDERING: &str = "ReactSnap";
const STATIC_PATH: &str = "static";
const IMAGES_PATH: &str = "static/images";
// use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct Logger {
    // ...
}

// ------ ------
// Before Mount
// ------ ------

fn before_mount(_: Url) -> BeforeMount {
    BeforeMount::new().mount_type(MountType::Takeover)
}

// ------ ------
//     Model
// ------ ------}

// We need at least 3 last values to detect scroll direction,
// because neighboring ones are sometimes equal.
type ScrollHistory = FixedVecDeque<[i32; 3]>;

pub struct Model {
    pub page: Page,
    pub scroll_history: ScrollHistory,
}

// ------ Page ------

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Page {
    Home,
    ApiRef,
    About,
    Tutorial,
    TutorialExample,
    NotFound,
}

impl Page {
    pub fn to_href(self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::ApiRef => "/api_ref",
            Self::About => "/about",
            Self::Tutorial => "/tutorial",
            Self::TutorialExample => "/tutorial_example",
            Self::NotFound => "/404",
        }
    }
}

impl From<Url> for Page {
    fn from(url: Url) -> Self {
        match url.path.first().map(String::as_str) {
            None | Some("") => Self::Home,
            Some("about") => Self::About,
            Some("api_ref") => Self::ApiRef,
            Some("tutorial") => Self::Tutorial,
            Some("tutorial_example") => Self::TutorialExample,
            _ => Self::NotFound,
        }
    }
}

// ------ ------
//  After Mount
// ------ ------

fn after_mount(url: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.send_msg(Msg::UpdatePageTitle);

    let model = Model {
        page: url.into(),
        scroll_history: ScrollHistory::new(),
    };

    AfterMount::new(model).url_handling(UrlHandling::None)
}

// fn is_in_prerendering() -> bool {
//     let user_agent =
//         window().navigator().user_agent().expect("cannot get user agent");

//     user_agent == USER_AGENT_FOR_PRERENDERING
// }

// ------ ------
//    Routes
// ------ ------

pub fn routes(url: Url) -> Option<Msg> {
    // Urls which start with `static` are files => treat them as external links.
    if url.path.starts_with(&[STATIC_PATH.into()]) {
        return None;
    }
    Some(Msg::RouteChanged(url))
}

// ------ ------
// Window Events
// ------ ------

pub fn window_events(_: &Model) -> Vec<EventHandler<Msg>> {
    vec![ev(Ev::Scroll, |_| {
        // Some browsers use `document.body.scrollTop`
        // and other ones `document.documentElement.scrollTop`.
        let mut position = body().scroll_top();
        if position == 0 {
            position = document()
                .document_element()
                .expect("cannot get document element")
                .scroll_top()
        }
        Msg::Scrolled(position)
    })]
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    RouteChanged(Url),
    UpdatePageTitle,
    ScrollToTop,
    Scrolled(i32),
    SubmitMarkdownHtml(String),
    NoOp,
}

impl std::default::Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::RouteChanged(url) => {
            model.page = url.into();
            orders.send_msg(Msg::UpdatePageTitle);
        },
        Msg::UpdatePageTitle => {
            let title = match model.page {
                Page::Home => TITLE_SUFFIX.to_owned(),
                Page::ApiRef => format!("Api Reference - {}", TITLE_SUFFIX),
                Page::About => format!("About - {}", TITLE_SUFFIX),
                Page::Tutorial => format!("Tutorial - {}", TITLE_SUFFIX),
                Page::TutorialExample => {
                    format!("MarkdownEditor - {}", TITLE_SUFFIX)
                },
                Page::NotFound => format!("404 - {}", TITLE_SUFFIX),
            };
            document().set_title(&title);
        },
        Msg::ScrollToTop => window().scroll_to_with_scroll_to_options(
            web_sys::ScrollToOptions::new().top(0.),
        ),
        Msg::Scrolled(position) => {
            *model.scroll_history.push_back() = position;
        },
        Msg::SubmitMarkdownHtml(html) => log!(html),
        Msg::NoOp => {},
    }
}

// ------ ------
//     View
// ------ ------

// Notes:
// - \u{00A0} is the non-breaking space
//   - https://codepoints.net/U+00A0
//
// - "▶\u{fe0e}" - \u{fe0e} is the variation selector, it prevents ▶ to change to emoji in some browsers
//   - https://codepoints.net/U+FE0E

#[topo::nested]
pub fn view(model: &Model) -> impl View<Msg> {
    div![
        class![C.fade_in, C.min_h_screen, C.flex, C.flex_col,],
        match model.page {
            Page::Home => page::home::view().els(),
            Page::ApiRef => page::api_ref::view().els(),
            Page::About => page::about::view().els(),
            Page::Tutorial => page::tutorial::view().els(),
            Page::TutorialExample => page::tutorial_example::view().els(),
            Page::NotFound => page::not_found::view().els(),
        },
        page::partial::header::view(model).els(),
        page::partial::footer::view().els(),
        handle_drop_types()
    ]
}

pub fn image_src(image: &str) -> String {
    format!("{}/{}", IMAGES_PATH, image)
}

pub fn asset_path(asset: &str) -> String {
    format!("{}/{}", STATIC_PATH, asset)
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn run() {
    log!("Starting app...");

    App::builder(update, view)
        .before_mount(before_mount)
        .after_mount(after_mount)
        .routes(routes)
        .window_events(window_events)
        .build_and_start();

    log!("App started.");
}
