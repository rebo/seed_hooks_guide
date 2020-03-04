use crate::{generated::css_classes::C, Msg, Page};
use seed_hooks::*;
use seed::{prelude::*, *};
use web_sys::{HtmlElement, HtmlTextAreaElement};

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

fn markdown_editor() -> Node<Msg>
{
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
".to_string()});

    let preview_el = use_state(ElRef::<HtmlElement>::default);
    let textarea_el = use_state(ElRef::<HtmlTextAreaElement>::default);

    div![
        class!["flex flex-col"],
        div![
            class!["flex flex-row"],
            div![class!["w-1/2"], "Markdown:"],
            div![class!["w-1/2"], "Preview:"],
        ],
        div![
            class!["flex flex-row h-64"],
            textarea![
                el_ref(&textarea_el.get()),
                bind(At::Value, source),
                class!["font-mono p-2 h-full flex-none w-1/2 border-gray-200 border shadow-lg"],
                attrs![At::Type => "textbox"],
                scroll_event_handler(Ev::KeyUp ,textarea_el, preview_el),
                scroll_event_handler(Ev::Scroll, textarea_el, preview_el),
            ],
            div![
                el_ref(&preview_el.get()),
                class!["markdown-body"],
                class!["overflow-auto p-2 pl-4 h-full flex-none w-1/2 border-gray-200 bg-indigo-100 border shadow-lg"],
                md!(&source.get())
            ]
        ],
        div![
            class!["flex justify-end pt-2"],
            button![
                class!["bg-green-400 p-4 m-2"],
                "Submit (See console log)",
                mouse_ev(Ev::Click, move |_| {
                    let markdown_element = preview_el.get().get().expect("markdown-body doesn't exist");
                    Msg::SubmitMarkdownHtml(markdown_element.inner_html())
                })
            ]
        ]
    ]
}

fn scroll_event_handler<Ms>(
    event: Ev,
    textarea_el: StateAccess<ElRef<HtmlTextAreaElement>>,
    preview_el: StateAccess<ElRef<HtmlElement>>,
) -> EventHandler<Ms>
where
    Ms: 'static + Default,
{
    textarea_el.input_ev(event, move |el, _| {
        if let (Some(textarea), Some(preview)) = (el.get(), preview_el.get().get()) {
            let textarea_scroll_percentage = {
                let textarea_max_scroll_top = textarea.scroll_height() - textarea.client_height();
                if textarea_max_scroll_top == 0 {
                    0.
                } else {
                    f64::from(textarea.scroll_top()) / f64::from(textarea_max_scroll_top)
                }
            };
            let new_preview_scroll_top = {
                let preview_max_scroll_top = preview.scroll_height() - preview.client_height();
                f64::from(preview_max_scroll_top) * textarea_scroll_percentage
            };
            preview.set_scroll_top(new_preview_scroll_top as i32);
        }
    })
}