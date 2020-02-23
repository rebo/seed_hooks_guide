use crate::{generated::css_classes::C, Msg, Page};
use comp_state::*;
use comp_state_seed_extras::*;
use comrak::{markdown_to_html, ComrakOptions};
use seed::{prelude::*, *};

pub fn view() -> Node<Msg> {
    div![
        div![div![class![
            "absolute w-full h-full bg-gray-900 opacity-50"
        ]]],
        class![C.fixed, C.inset_0, C.z_50, C.overflow_auto, C.flex,],
        div![
            class![
                C.relative,
                C.p_8,
                C.bg_white,
                C.w_full,
                C.max_w_5xl,
                C.m_auto,
                C.flex_col,
                C.flex,
                C.rounded_sm,
                C.shadow_2xl
            ],
            div![
                class![C.flex_col],
                div![h2![class![C.font_bold, C.text_center], "Markdown Editor"]],
                hr![class![C.my_8 C.border_b_2 C.border_gray_200]],
                div![class![C.p_4], markdown_editor()],
                div![
                    class![C.flex, C.justify_end, C.pt_2],
                    a![
                        attrs![At::Href => Page::Tutorial.to_href()],
                        "Return to Tutorial"
                    ],
                ]
            ]
        ]
    ]
}

fn empty_fn() -> Node<Msg> {
    empty![]
}

fn set_scroll(textarea: web_sys::HtmlTextAreaElement, preview: web_sys::HtmlElement) {
    let scroll_percentage = (textarea.scroll_top() as f64) / (textarea.scroll_height() as f64);
    let new_scroll_top = (preview.scroll_height() as f64) * scroll_percentage;
    preview.set_scroll_top(new_scroll_top as i32);
}

#[topo::nested]
fn markdown_editor() -> Node<Msg> {
    let source = use_state(|| {
        "# Header

    ## Smaller Header
    
    **Hello** this is a *markdown* renderer.
    
    * List item
    * List item
        1. Number item
        1. Number item
        1. Number item
    * List item
    * List item
    * List item
    * List item    
    "
        .to_string()
    });
    let preview_el = use_state::<ElRef<web_sys::HtmlElement>, _>(ElRef::default);
    let textarea_el = use_state::<ElRef<web_sys::HtmlTextAreaElement>, _>(ElRef::default);

    let processed_md = markdown_to_html(&source.get(), &ComrakOptions::default());

    div![
        class!["flex flex-col"],
        div![
            class!["flex flex-row"],
            div![class!("w-1/2"), "Markdown:"],
            div![class!("w-1/2"), "Preview:"],
        ],
        div![
            class!["flex" "flex-row" "h-64"],
            textarea![
                el_ref(&textarea_el.get()),
                bind(At::Value, source),
                class!["font-mono p-2 h-full flex-none w-1/2 border-gray-200 border shadow-lg"],
                attrs![At::Type => "textbox"],
                textarea_el.input_ev(Ev::KeyUp, move |el, _| {
                    if let (Some(textarea), Some(preview)) = (el.get(), preview_el.get().get()) {
                        set_scroll(textarea, preview);
                    }
                }),
                textarea_el.input_ev(Ev::Scroll, move |el, _| {
                    if let (Some(textarea), Some(preview)) = (el.get(), preview_el.get().get()) {
                        set_scroll(textarea, preview);
                    }
                })
            ],
            div![
                class!["md-preview"],
                el_ref(&preview_el.get()),
                class!["overflow-auto p-2 pl-4 h-full flex-none w-1/2 border-gray-200 bg-indigo-100 border shadow-lg"],
                raw!(&processed_md)
            ]
        ],
        div![
            class!["flex justify-end pt-2"],
            button![
                class!["bg-green-400 rounded-lg p-4 m-2"],
                "Submit",
                mouse_ev(Ev::Click, move |_| Msg::SubmitMarkdownHtml(processed_md))
            ]
        ]
    ]
}
