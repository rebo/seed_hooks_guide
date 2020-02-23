use crate::{generated::css_classes::C, Msg, Page};
use comp_state::*;
use comp_state_seed_extras::*;
use comrak::{markdown_to_html, ComrakOptions};
use seed::{prelude::*, *};

pub fn view() -> Node<Msg> {
    div![
        div![div![class![
            C.absolute C.w_full C.h_full C.bg_gray_900 C.opacity_50
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
        class![C.flex C.flex_col],
        div![
            class![C.flex C.flex_row],
            div![class!(C.w_1of2), "Markdown:"],
            div![class!(C.w_1of2), "Preview:"],
        ],
        div![
            class!["flex" "flex-row" C.h_64],
            textarea![
                el_ref(&textarea_el.get()),
                bind(At::Value, source),
                class![C.font_mono C.p_2 C.h_full C.flex_none C.w_1of2 C.border_gray_200 C.border C.shadow_lg],
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
                class![C.overflow_auto C.p_2 C.pl_4 C.h_full C.flex_none C.w_1of2 C.border_gray_200 C.bg_indigo_100 C.border C.shadow_lg],
                raw!(&processed_md)
            ]
        ],
        div![
            class![C.flex C.justify_end C.pt_2],
            button![
                class![C.bg_green_400 C.rounded_lg C.p_4 C.m_2],
                "Submit",
                mouse_ev(Ev::Click, move |_| Msg::SubmitMarkdownHtml(processed_md))
            ]
        ]
    ]
}
