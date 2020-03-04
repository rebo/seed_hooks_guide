use crate::{generated::css_classes::C, Msg};
use seed_hooks::*;
use wasm_bindgen::JsCast;
use ifmt::iformat as f;
use crate::Page;

use seed::{prelude::*, *};
pub fn view() -> Node<Msg> {
    div![
        id!("page"),
        class![C.flex C.flex_col],
        header![
            style!{St::GridArea => "header"},
            class![C.shadow_xl, C.bg_gray_600, C.text_gray_200, C.flex, C.justify_end, C.content_center, C.items_center],
            a![class![C.h_full, C.py_2, C.px_2, C.mx_4, C.hover__text_white, C.text_gray_500, C.pointer_events_none], attrs!(At::Href => "#"), "SEED HOOKS"],
            div![class![C.h_6, C.border_r_2]],
            a![class![C.h_full, C.py_2, C.px_2, C.mx_4, C.hover__text_white], attrs!(At::Href => Page::Tutorial.to_href()), "TUTORIAL"],
            div![class![C.h_6, C.border_r_2]],
            a![class![C.h_full, C.py_2, C.px_2, C.mx_4 ,C.hover__text_white], attrs!(At::Href => Page::ApiRef.to_href()), "API REFERENCE"],
        ], 
        nav![
            style!{St::GridArea => "nav"},
            class![
                C.bg_gray_700,
                C.text_gray_400,
                C.overflow_y_auto
            ],
            left_bar_content(),
        ],
        main![
            style!{St::GridArea => "main"},
            class![C.overflow_y_auto],
            main_screen_content()
        ],
    ]
}

fn left_bar_content() -> Node<Msg> {
    div![
        class![C.p_3],
        h1!["API"],
        hr![class![C.my_8 C.border_b_2 C.border_gray_200]],
        h2![a![
            class![C.hover__text_gray_100, C.border_b_2, C.border_transparent, C.hover__border_gray_300],
            attrs![At::Href=>"api_ref#intro"],
            "Introduction"
        ]],
        ul![
            a![
                class![C.hover__text_gray_100, C.border_b_2, C.border_transparent, C.hover__border_gray_300],
                attrs![At::Href=>"api_ref#state_functions"],
                "State Functions"
            ],
            li![a![
                class![C.ml_2,C.hover__text_gray_100, C.border_b_2, C.border_transparent, C.hover__border_gray_300],
                attrs![At::Href=>"api_ref#use_state"],
                "use_state"
            ]],
            li![a![
                class![C.ml_2,C.hover__text_gray_100, C.border_b_2, C.border_transparent, C.hover__border_gray_300],
                attrs![At::Href=>"api_ref#new_state"],
                "new_state"
            ]],
            li![a![
                class![C.ml_2,C.hover__text_gray_100, C.border_b_2, C.border_transparent, C.hover__border_gray_300],
                attrs![At::Href=>"api_ref#on_unmount"],
                "on_unmount"
            ]],
        ],
        ul![
            a![
                class![C.hover__text_gray_100, C.border_b_2, C.border_transparent, C.hover__border_gray_300],
                attrs![At::Href=>"api_ref#conditional"],
                "Conditional Execution"
            ],
            li![a![
                class![C.ml_2,C.hover__text_gray_100, C.border_b_2, C.border_transparent, C.hover__border_gray_300],
                attrs![At::Href=>"api_ref#do_once"],
                "do_once"
            ]],
            li![a![
                class![C.ml_2,C.hover__text_gray_100, C.border_b_2, C.border_transparent, C.hover__border_gray_300],
                attrs![At::Href=>"api_ref#after_render"],
                "after_render"
            ]],
        ],
        ul![
            a![
                class![C.hover__text_gray_100, C.border_b_2, C.border_transparent, C.hover__border_gray_300],
                attrs![At::Href=>"api_ref#state_access"],
                "StateAccess<T>"
            ],
            li![a![
                class![C.ml_2,C.hover__text_gray_100, C.border_b_2, C.border_transparent, C.hover__border_gray_300],
                attrs![At::Href=>"api_ref#get"],
                "get"
            ]],
            li![a![
                class![C.ml_2,C.hover__text_gray_100, C.border_b_2, C.border_transparent, C.hover__border_gray_300],
                attrs![At::Href=>"api_ref#get_with"],
                "get_with"
            ]],
            li![a![
                class![C.ml_2,C.hover__text_gray_100, C.border_b_2, C.border_transparent, C.hover__border_gray_300],
                attrs![At::Href=>"api_ref#set"],
                "set"
            ]],
            li![a![
                class![C.ml_2,C.hover__text_gray_100, C.border_b_2, C.border_transparent, C.hover__border_gray_300],
                attrs![At::Href=>"api_ref#update"],
                "update"
            ]],
            // li![a![
            //     class![C.hover__text_gray_100],
            //     attrs![At::Href=>"api_ref#changed"],
            //     "changed"
            // ]],
            // li![a![
            //     class![C.hover__text_gray_100],
            //     attrs![At::Href=>"api_ref#events"],
            //     "..events"
            // ]]
        ],
        ul![
            a![attrs![At::Href=>"api_ref#dx"], "DX"],
            li![a![
                class![C.ml_2,C.hover__text_gray_100, C.border_b_2, C.border_transparent, C.hover__border_gray_300],
                attrs![At::Href=>"api_ref#bind"],
                "bind"
            ]],
        ], 
        ul![
            a![attrs![At::Href=>"api_ref#glossary"], "Glossary"],
        ],

        // ul![
        //     "Utility Functions",
        //     li![a![
        //         class![C.hover__text_gray_100],
        //         attrs![At::Href=>"api_ref#use_list"],
        //         "use_list"
        //     ]],
        // ]
    ]
}

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Prism)]
    fn highlightElement(el: web_sys::HtmlElement);
    #[wasm_bindgen(js_namespace = Prism)]
    fn highlightAllUnder(el: web_sys::HtmlElement);
}

// Prism.highlightElement(element, async, callback)



#[topo::nested]
fn section_desc<T: Into<String>>(
    href_name: T,
    title: T,
    description: T,
) -> Vec<Node<Msg>> {
    
    

    let title = md!(&title.into());
    let description = md!(&description.into());

    let desc_el = use_state(ElRef::<web_sys::HtmlElement>::default);


    do_once(||
        after_render(move |_| {

        if let Some(desc_el) = desc_el.get().get() {
            let code_children = desc_el.get_elements_by_tag_name("h3");

            for idx in 0..code_children.length() {
                let code_el = code_children.item(idx).unwrap();
                code_el.set_class_name("text-xl py-3 pt-4");
            }

            let code_children = desc_el.get_elements_by_tag_name("code");

            for idx in 0..code_children.length() {
                let code_el = code_children.item(idx).unwrap();
                code_el.set_class_name("language-rust");
                highlightElement(
                    code_el.dyn_into::<web_sys::HtmlElement>().unwrap(),
                );
            }
        }
    })).reset_on_unmount();


    nodes![
        h2![
            class![C.m_3, C.text_2xl],
            a![attrs![At::Name=>href_name.into()],  title]
        ],
        hr![class![C.my_8 C.border_b_2 C.border_gray_200]],
        div![
            el_ref(&desc_el.get()),
            class!["api-description" C.m_3 C.leading_relaxed],
            description
        ],
    ]
}

#[topo::nested]
fn function_desc<T: Into<String>>(
    href_name: T,
    title: T,
    signature: Option<&str>,
    description: T,
    code: T,
    modal_content: StateAccess<(bool, fn() -> Node<Msg>)>,
    code_example: fn() -> Node<Msg>,
) -> Node<Msg> {
    let href_name = href_name.into();
    let title = md!(&title.into());
    let description =
        md!(&description.into());
    let code = code.into();
    let code_el = use_state(ElRef::<web_sys::HtmlElement>::default);
    let desc_el = use_state(ElRef::<web_sys::HtmlElement>::default);

    
    

    do_once(|| {
        after_render(move |_| {
    
        if let Some(code_el) = code_el.get().get() {
            if let Some(desc_el) = desc_el.get().get() {
                let code_children = desc_el.get_elements_by_tag_name("code");

                for idx in 0..code_children.length() {
                    let code_el = code_children.item(idx).unwrap();
                    code_el.set_class_name("language-rust");
                    highlightElement(
                        code_el.dyn_into::<web_sys::HtmlElement>().unwrap(),
                    );
                }
            }

            highlightElement(code_el);
        }
    });
}).reset_on_unmount();

    div![
        h3![
            class![C.m_3, C.text_xl],
            a![attrs![At::Name=> href_name], title]
        ],
        if let Some(sig) = signature {
            pre![class![C.p_4], code![sig]]
        } else {
            empty![]
        },
        div![
            class![C.flex C.flex_row],
            div![
                el_ref(&desc_el.get()),
                class![C.p_3, C.w_1of2 C.flex_none],
                description
            ],
            div![
                class![C.p_3, C.w_1of2 C.flex_none],
                pre![
                    class!(C.rounded, C.shadow),
                    code![
                        class!("language-rust"),
                        el_ref(&code_el.get()),
                        code
                    ]
                ],
                div![class![C.flex, C.justify_end, C.pt_2],
                button!["Show Example",
                    class![
                        C.mx_3 
                        C.bg_indigo_500,
                        C.hover__bg_indigo_400,
                        C.text_white,
                        C.font_bold,
                        C.py_2,
                        C.px_4,
                        C.border_b_4,
                        C.border_indigo_600,
                        C.hover__border_indigo_500,
                        C.rounded_sm
                ], 
                modal_content.mouse_ev(Ev::Click, move |mc,_| *mc = (true, code_example))]
                ]
            ]
        ]
    ]
}

#[topo::nested]
fn use_state_example() -> Node<Msg> {
    let count = use_state(|| 0);
    div!["Count:",
        count.get().to_string(),
        button!["Increase Count",  class![
            C.mx_2
            C.bg_gray_500,
            C.hover__bg_gray_400,
            C.text_white,
            C.font_bold,
            C.py_2,
            C.px_2,
            C.text_sm,
            C.border_b_4,
            C.border_gray_600,
            C.hover__border_gray_500,
            C.rounded_lg
    ] , count.mouse_ev(Ev::Click, |count, _| *count += 1)],
    ]
}

#[topo::nested]
fn numberbind() -> Node<Msg> {
    let a = use_state(|| 0);
    let b = use_state(|| 0);

    div![
        input![attrs![At::Type=>"number"], bind(At::Value, a)],
        input![attrs![At::Type=>"number"], bind(At::Value, b)],
        p![f!(a "+" b "=" a + b)]
    ]
} 

#[topo::nested]
fn unmount_example() -> Node<Msg> {
    let count = use_state(|| 0);

    let resettable_count = use_state(|| 0);
    on_unmount(move ||resettable_count.delete());

    let shortcut_count = use_state(|| 0).reset_on_unmount();
 
    div!["Count:",
        div!["normal count - ", f!(count)],
        div!["resettable count - ", f!(resettable_count)],
        div!["shortcut count - ", f!(shortcut_count)],
        button!["Increase Counts",  class![
            C.mx_2
            C.bg_gray_500,
            C.hover__bg_gray_400,
            C.text_white,
            C.font_bold,
            C.py_2,
            C.px_2, 
            C.text_sm,
            C.border_b_4,
            C.border_gray_600,
            C.hover__border_gray_500,
            C.rounded_lg
    ] , count.mouse_ev(Ev::Click, |count, _| *count += 1)
      , resettable_count.mouse_ev(Ev::Click, |count, _| *count += 1)
      , shortcut_count.mouse_ev(Ev::Click, |count, _| *count += 1)
        ],
    ]
}
    
    

#[topo::nested]
fn if_example() -> Node<Msg> {
    use std::cmp::Ordering;
    let input_a = use_state(String::new);
    let input_b = use_state(String::new);

    if input_a.changed() || input_b.changed() {
        after_render(move |_| {
            if let (Ok(a), Ok(b)) = (input_a.get().parse::<i32>(), input_b.get().parse::<i32>()) {
                let smallest = match a.cmp(&b) {
                    Ordering::Less => "<li>A is the smallest</li>",
                    Ordering::Greater => "<li>B is the smallest</li>",
                    Ordering::Equal => "<li>Neither is the smallest</li>",
                };

                if let Some(elem) = get_html_element_by_id("list") {
                    let _ = elem.insert_adjacent_html("beforeend", smallest);
                }
            }
        });
    }

    div![
        "A:",
        input![bind(At::Value, input_a), class![C.border_gray_600, C.rounded_sm,C.border_2, C.shadow, C.p_2, C.m_3],],
        "B:",
        input![bind(At::Value, input_b), class![C.border_gray_600, C.rounded_sm,C.border_2, C.shadow, C.p_2, C.m_3],],
        ul![id!("list"), "Smallest Log:"],
    ]
}

#[topo::nested]
fn do_once_example() -> Node<Msg> {
    let name = "bob";
    let message = use_state(||empty![]);
    
    message.set(
        span!["This will only been seen after the first re-render, 
        the welcome message will never be seen again."]
    );

    do_once(|| 
        message.set(
            div![span!["Welcome ", name],
                button!["Clear Message",class![
                    C.mx_2
                    C.bg_gray_500,
                    C.hover__bg_gray_400,
                    C.text_white,
                    C.font_bold,
                    C.py_2,
                    C.px_2,
                    C.text_sm,
                    C.border_b_4,
                    C.border_gray_600,
                    C.hover__border_gray_500,
                    C.rounded_lg
                ], 
                mouse_ev(Ev::Click, |_| Msg::default())]])
    );

    message.get()
}

#[topo::nested]
fn new_state_example() -> Node<Msg> {
    let todos = use_state(|| vec![use_state(String::new)]);
    div![
        todos.get().iter().enumerate().map(|(idx, todo)| {
            vec![
                input![
                    class![C.border_gray_600, C.rounded_sm,C.border_2, C.shadow, C.p_2, C.m_3],
                    bind(At::Value, *todo)],
                button![class![
                    C.mx_2
                    C.bg_gray_500,
                    C.hover__bg_gray_400,
                    C.text_white,
                    C.font_bold,
                    C.py_2,
                    C.px_2,
                    C.text_sm,
                    C.border_gray_600,
                    C.rounded_lg
            ] ,
                    todos.mouse_ev(Ev::Click, move |t,_| {t.remove(idx);}),
                    "X" 
                ],
                br![],
            ]
        }),
        button![class![
            C.mx_2
            C.bg_green_500,
            C.hover__bg_green_400,
            C.text_white,
            C.font_bold,
            C.py_2,
            C.px_2,
            C.text_sm,
            C.border_b_4,
            C.border_green_600,
            C.hover__border_green_500,
            C.rounded_lg
    ] ,
            todos.mouse_ev(Ev::Click, move |t,_| t.push(new_state(String::new))),
            "Add Todo" 
        ]
    ]
}



fn modal(modal_content: StateAccess<(bool, fn() -> Node<Msg> )>) -> Node<Msg> {
    let (show, content) = modal_content.get();
    if show {
    div![
        div![
            div![class!["absolute w-full h-full bg-gray-900 opacity-50"]]
        ],
        class![
            C.fixed, C.inset_0, C.z_50, C.overflow_auto, C.flex,
        ],
        div![
            class![
                C.relative, C.p_8, C.bg_white, C.w_full, C.max_w_5xl, C.m_auto, C.flex_col, C.flex, C.rounded_sm, C.shadow_2xl
            ],
            div![
                class![C.flex_col],
                div![h2![class![C.font_bold, C.text_center], "Code Example"]],
                hr![class![C.my_8 C.border_b_2 C.border_gray_200]],
                div![class![C.p_4], content()],
                div![class![C.flex, C.justify_end, C.pt_2],
                    button![
                        attrs! {At::Type => "button"},
                        class![
                            C.mx_3 
                            C.bg_indigo_500,
                            C.hover__bg_indigo_400,
                            C.text_white,
                            C.font_bold,
                            C.py_2,
                            C.px_4,
                            C.border_b_4,
                            C.border_indigo_600,
                            C.hover__border_indigo_500,
                            C.rounded_sm  
                        ],
                        modal_content.mouse_ev(Ev::Click, |mc,_| *mc = (false, || empty![])),
                        "Close"
                    ],
                ]
            ]]
        ]
    } else {
        empty![]
    }
}
struct NonCloneString(String);

#[topo::nested]
fn my_non_clone_input() -> Node<Msg> {
    let input_access = use_state(|| NonCloneString("".to_string()));
    let val = input_access.get_with(|v| format!("{}", v.0));

    div![
        input![attrs![At::Value => val], 
        class![C.border_gray_600, C.rounded_sm,C.border_2, C.shadow, C.p_2, C.m_3],
            input_access.input_ev(Ev::Input, |i,text| *i=NonCloneString(text))
        ],
        format!("Text inputted: {}", val)
    ]
}


#[topo::nested]
fn update_example() -> Node<Msg> {
    let count = use_state(|| 3);

    div![
        button![
            "-",
            class![
                    C.mx_2
                    C.bg_gray_500,
                    C.hover__bg_gray_400,
                    C.text_white,
                    C.font_bold,
                    C.py_2,
                    C.px_2,
                    C.text_sm,
                    C.border_b_4,
                    C.border_gray_600,
                    C.hover__border_gray_500,
                    C.rounded_lg
                ], 
            mouse_ev(Ev::Click, move |_| {
                count.update(|v| *v -= 1);
                Msg::NoOp
            }),
        ],
        count.get().to_string(),
        button![
            "+",
            class![
                    C.mx_2
                    C.bg_gray_500,
                    C.hover__bg_gray_400,
                    C.text_white,
                    C.font_bold,
                    C.py_2,
                    C.px_2,
                    C.text_sm,
                    C.border_b_4,
                    C.border_gray_600,
                    C.hover__border_gray_500,
                    C.rounded_lg
                ], 
            mouse_ev(Ev::Click, move |_| {
                count.update(|v| *v += 1);
                Msg::NoOp
            }),
        ],
    ]
}

#[topo::nested]
fn my_input() -> Node<Msg> {
    let input_access = use_state(|| "".to_string());

    div![
        input![attrs![At::Value => input_access.get()], 
        class![C.border_gray_600, C.rounded_sm,C.border_2, C.shadow, C.p_2, C.m_3],
            input_access.input_ev(Ev::Input, |i,text| *i=text)
        ],
        format!("Text inputted: {}", input_access)
    ]
}

#[topo::nested]
fn set_list() -> Node<Msg> {
    let selected = use_state(||"");

    ul![ "Selected Item:", selected.get(),
        li!["1st Item", mouse_ev(Ev::Click, move |_| { selected.set("1"); Msg::default() }), class![C.cursor_pointer]],
        li!["2nd Item", mouse_ev(Ev::Click, move |_| { selected.set("2"); Msg::default() }), class![C.cursor_pointer]],
        li!["3rd Item", mouse_ev(Ev::Click, move |_| { selected.set("3"); Msg::default() }), class![C.cursor_pointer]],
        li!["4th Item", mouse_ev(Ev::Click, move |_| { selected.set("4"); Msg::default() }), class![C.cursor_pointer]],
        li!["5th Item", mouse_ev(Ev::Click, move |_| { selected.set("5"); Msg::default() }), class![C.cursor_pointer]],
    ]
}

fn empty_fn() -> Node<Msg> {
    empty![]
}

fn main_screen_content() -> Node<Msg> {
    let modal_content = use_state(||(false, empty_fn as fn()-> Node<Msg>));
    div![
        modal(modal_content),
        section![section_desc(
            "start_here",
            "Introduction",
            r#"[**Seed hooks**](https://crates.io/crates/seed_hooks) enable local component state in Seed. A hook function, often
`use_state()` is used to store state associated with a component. This is an example of the code that uses Seed hooks.

```rust
#[topo::nested]
fn name_input() -> Node<Msg> {
    let name = use_state(|| "".to_string());

    div![
        input![bind(At::Value, name)],
        format!("Hello {}", name)
    ]
}
```
In the above code `name` is a **state accessor** for a local **state variable** which is then bound to the `input!` field's value attribute.

On this page all words in **bold** have a glossary definition at the bottom of this page.

### Why are **Seed hooks** needed?

Seed hooks allow for the creation of re-usable components that can be freely integrated into any Seed app at any location in the view hierarchy. 
Traditionally in Seed, and other Elm architecture style frameworks, this has required a fair amount of boilerplate and glue code. 

Seed hooks allow 'components' to have their own state and those components can then be freely composed and re-used at will. Due to this they are ideal
for functionality that does not need to touch the main Seed `View->Message->Update->View` loop. For instance, a drop-down menu toggle, an input element
state, or modal dialog visibility.

Complex components can be created and re-used, such as date pickers, which do not need to be hard wired into the main app.

### Setting up a Seed app to use Seed hooks

Currently **Seed hooks** only work on nightly rust, this is due to requiring the feature `TrackCaller` therefore it is 
important to install a recently nightly. This site has been built with the nightly of 26th February 2020. You need to ensure this feature
is enabled at the top of `lib.rs`

```
// in lib.rs

#![feature(track_caller)]

use seed_hooks::*;
```




When using Seed hooks, components are structured as a tree. This means that components have a parent and potentially a number of children. Therefore 
you need to ensure you have setup a root component. You can do this by annotating the root Seed view with `#[topo::nested]`

```
// in lib.rs

#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
    div![]
}
```

`#[topo::nested]` functions have a unique id which is based on the function's **execution context**, source file call-site and an indexed **slot**.
This enables functions to be **topologically aware** and therefore considered as unique components with local state.

At present if event handlers helpers are to be used then the Seed `Msg` type should also implement a `default()` no-op. This restriction will be lifted eventually:

```rust
enum Msg {
    NoOp
}

impl Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
}
```
This api guide summarises the hooks and functions currently available in the crate[seed_hooks](https://github.com/rebo/seed_hooks).

Only the main functions are described here there are many more available for use in specific circumstances, please refer 
to the `doc.rs` documentation for a full list.

Eventually these crates will be incorporated into Seed directly

Here is a complete `lib.rs` file demonstrating the basic usage.

```
#![feature(track_caller)]
use seed::{prelude::*, *};
use seed_hooks::*;

#[derive(Default)]
struct Model {}

enum Msg {
    NoOp,
}

impl Default for Msg {
    fn default() -> Msg {
        Msg::NoOp
    }
}

fn update(msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::NoOp => (),
    }
}

#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
    div![
        my_button(),
        my_button(),
        my_button(),
    ]
}

#[topo::nested]
fn my_button() -> Node<Msg> {
    let count = use_state(|| 3);

    div![
        count.get().to_string(),
        button!["+", count.mouse_ev(Ev::Click, |count, _| *count += 1)],
    ]
}
```
"#,
        )],
        section![
            section_desc(
                "state_functions",
                "State Functions",
                "Seed hooks' **state functions** are functions that relate to the storing of local state for a component.
The primary function used is `use_state` which stores an arbitrary value and returns an accessor. The other functions are used
in specific situations, of which `new_state` is also covered here.

Users of React will notice some similarity between Seed Hooks and React Hooks, please note that Seed Hooks do not have the same
restrictions as React Hooks as regards calling of hook functions. For instance Seed Hook's can be freely called within conditionals 
and loops.
"
            ),
            function_desc(
                "use_state",
                "`use_state`",
                Some("fn use_state<T: 'static, F: FnOnce() -> T>(data_fn: F) -> StateAccess<T>"),
                "`use_state` is the standard state function for storing of a local **state variable** for a component. 
                It returns a `StateAccess` **state accessor** which is responsible for all getting, setting and updating of the underlying value.

The function takes a lazily evaluated closure which returns a value that gets stored on first execution. The stored value is
linked to the component by a `topo::Id` which is also stored in the `StateAccess` accessor.

The only limit on the type of value stored is a `'static` lifetime, however if use of the `get()` method, whilst clones the stored state, is required then
the type should be `Clone`.

The code snippet demonstrates the use of `use_state` to store a count which gets updated on a button click.
",
r#"#[topo::nested]
fn my_button() -> Node<Msg> {
    let count = use_state(|| 0);

    div![
        count.get().to_string(),
        button!["+", count.mouse_ev(Ev::Click, |count, _| *count += 1)],
    ]
}"#,modal_content, use_state_example
            ),
            function_desc(
                "new_state",
                "`new_state`",
                Some("fn new_state<T: 'static, F: FnOnce() -> T>(data_fn: F) -> StateAccess<T>"),
                r#"This function is identical to `use_state` with the exception that every time the function is executed it creates a new 
topological context. The closure runs on every execution.

The use-case for this is to allow creation of state variables and associated accessors in an event callback.

Consider the following code, it will store a **state variable** when the button is clicked. But because the call-site, parent call tree, 
and slot are all identical it will undesirably refer to the same `topo::Id` therefore when multiple buttons are rendered in a view they
will all update the same state.

```
button![
    todos.mouse_ev(Ev::Click, move |t,_| t.push(use_state(String::new))),
    "Add" 
]
```
The problem with this is that every state accessor stored within the todo list will effectively refer to the same component. 
Simply using `new_state` instead of `use_state` will ensure that every accessor stored will refer to a new topological context:

```
button![
    todos.mouse_ev(Ev::Click, move |t,_| t.push(new_state(String::new))),
    "Add" 
]
```
The code example is a fully interactive todo list in 15 line of code. `todos` is a state accessor that stores 
a `Vec` of `String` state accessors, these are then used to store the state of each todo. 

`new_state` is used in the on click event do add a new unique todo.
"#,
r#"#[topo::nested]
fn todos() -> Node<Msg> {
    let todos = use_state(|| vec![use_state(String::new)]);
    div![
        todos.get().iter().enumerate().map(|(idx, todo)| {
            vec![
                input![
                    bind(At::Value, *todo)],
                button![
                    todos.mouse_ev(Ev::Click, move |t,_| {t.remove(idx);}),
                    "X" 
                ],
                br![],
            ]
        }),
        button![
            todos.mouse_ev(Ev::Click, move |t,_| t.push(new_state(String::new))),
            "Add Todo" 
        ]
    ]
}"#, modal_content ,new_state_example
            ),
            function_desc(
                "on_unmount",
                "`on_unmount() and handle_unmount()`",
                Some("fn on_unmount<F: Fn() -> () + 'static>(unmount_fn: F) -> StateAccess<Unmount>"),
r#"
These functions are used to execute a closure when a component has stopped being rendered. Typically this is used to
allow `do_once` blocks to be re-run, or to reset components when they are no longer being rendered.

`on_unmount()` accepts a closure as the only argument, this is then stored as a *state variable* in a `Unmount`. 

Seed Hooks know when this *state variable* is no longer accessed and can therefore then execute the closure.

`StateAccessors` have a helper method `reset_on_unmount()` defined on them which will delete the state when the state is no longer rendered.

Creating the `Unmount` is insufficient for the closure to be fired.  This is where `handle_unmounts()` comes in.
The function is added as the last item in the main view macro. It will then cause the closure to be activated just prior to the end of 
the view function. This function returns an `empty![]` therefore will not affect the view.

Is is essential that if a state variable is deleted by a `Unmount` callback or otherwise that it is not later accessed 
because this will result in a panic.

The example is 3 simple counters, one counter retains state when the modal dialog is closed the other two do not.
    
"#,
r#"
#[topo::nested]
fn unmount_example() -> Node<Msg> {
    let count = use_state(|| 0);

    let resettable_count = use_state(|| 0);
    let unmount = on_unmount(move ||resettable_count.delete());

    let shortcut_count = use_state(|| 0).reset_on_unmount();
    
    div!["Counts (Try closing modal, then re-opening after increasing counters):",
        div!["normal count", f!(count)],
        div!["resettable count", f!(resettable_count)],
        div!["resettable count 2", f!(shortcut_count)],
        button!["Increase Count", 
            count.mouse_ev(Ev::Click, |count, _| *count += 1), 
            resettable_count.mouse_ev(Ev::Click, |count, _| *count += 1), 
            shortcut_count.mouse_ev(Ev::Click, |count, _| *count += 1)
        ],
    ]
}

// in the root view:
#[topo::nested]
pub fn view(model: &Model) -> impl View<Msg> {
    div![
        ...,
        handle_unmounts()
    ]
}
"#
        ,
        modal_content, unmount_example
            ),
        
        ],
        section![
            section_desc(
                "conditional",
                "Conditional Execution Functions",
                r#"**Seed Hooks** provide some functions to assist with conditionally executing code. 
This is required when taking a hooks approach to component design because some logic may need to be executed in the view. 
The primary hooks in this regard are `do_once` and `after_render`.
    "#),
    function_desc(
        "do_once",
        "`do_once`",
        Some("fn do_once<F: Fn() -> ()>(func: F) -> StateAccess<bool>"),
        "`do_once()` executes the closure supplied once and only once. The execution runs synchronously that is immediately prior to any further statement. 
Often this is combined with `after_render()` which schedules an closure to be executed after the next page render.  You typically use `do_once()` 
when triggering an external javascript library that needs to complete an action a single time prior to a component being mounted.

The `do_once` function returns a *state accessor* to the *state variable* that controls whether `do_once` can run. This means we can allow the 
`do_once` to run again by calling `reset_on_unmount()` on the return value. See `on_unmount` for more information on this.

The example on the right outputs a welcome message once and once only.
",
r#"#[topo::nested]
fn welcome_user_once(name: String) -> Node<Msg> {

    let message = use_state(||empty![]);
       
    message.set(
        span!["This will only been seen after the first re-render, 
        the welcome message will never be seen again."]
    );

    do_once(|| 
        message.set(
            div![
                span!["Welcome ", name], 
            ]
        )
    );

    message.get()
}
"#,modal_content, do_once_example
    ),
    function_desc(
        "after_render",
        "`after_render`",
        Some("fn after_render<F: Fn(f64) -> () + 'static>(func: F)"),
        "`after_render()` executes the closure supplied after the next render. The execution runs asynchronously 
that is after the DOM tree has been created, diffed, and after the view has been painted to the window.
Often this is combined with `do_once()` which schedules an closure to be executed only once after the next page render.  

You typically use `after_render()` when triggering a dom interaction, for instance an animation or popup that is not part of the virtual dom tree.

The example on the right renders two input boxes and after an input event schedules a calculation to update the dom manually",
r#"
#[topo::nested]
fn if_example() -> Node<Msg> {
    use std::cmp::Ordering;
    let input_a = use_state(String::new);
    let input_b = use_state(String::new);

    if input_a.changed() || input_b.changed() {
        after_render(move |_| {
            if let (Ok(a), Ok(b)) = (input_a.get().parse::<i32>(), input_b.get().parse::<i32>()) {
                let smallest = match a.cmp(&b) {
                    Ordering::Less => "<li>A is the smallest</li>",
                    Ordering::Greater => "<li>B is the smallest</li>",
                    Ordering::Equal => "<li>Neither is the smallest</li>",
                };

                if let Some(elem) = get_html_element_by_id("list") {
                    let _ = elem.insert_adjacent_html("beforeend", smallest);
                }
            }
        });
    }

    div![
        "A:",
        input![bind(At::Value, input_a)],
        "B:",
        input![bind(At::Value, input_b)],
        ul![id!("list"), "Smallest Log:"],
    ]
}
"#,modal_content, if_example
    ),
        ],
        section![
        //     h2![a![attrs![At::Name=>"state_access"], "StateAccess Struct"]],
        section_desc(
            "state_access",
            "StateAccess<T>",
            r#"Seed Hook's **State Functions** return a `StateAccess<T>` value. This is an accessor which
provides amongst other features getter and setter access to the stored value of type T.

The `StateAccess<T>` accessor knows what component's state to update and therefore this accessor can be used 
in `EventHandler` callbacks to update state.

Please note that unlike React Hooks StateAccess getter & setters do not reschedule a re-render of the 
virtual DOM.

The struct implements `Copy` and therefore can be freely shared, this is independent as to whether `T` implements `Copy`.

The primary method used to retrieve stored data is `get()`, this only works with `Clone` types. For non-`Clone` types
the `get_with()` method is available.

Advanced patterns include using `bind()` to link an accessor to a DOM element's attribute or storing a 
collection of state accessors to manage complex tree structures.
"#),
        
        function_desc(
            "get",
            "`get`",
            Some("fn get(&self) -> T // T must be Clone + 'static"),
            "This method returns a clone of the stored data, therefore in order for it to be used `T` must of course implement `Clone`.
Although all accesses will therefore cause an allocation due to the clone, this is the most direct way in which to access the stored data.
Care should be taken in understanding that the clone may be stale if this value is used in a callback.

For this reason using `update()` in a callback is usually preferable to using `set()`.

The example demonstrates displaying a value stored by an accessor from an `Input` event.",
    r#"
    #[topo::nested]
    fn my_input() -> Node<Msg> {
        let input_access = use_state(|| "".to_string());
    
        div![
            input![attrs![At::Value => input_access.get()], 
                input_ev(Ev::Input , |text| {
                    input_access.set(text);
                    Msg::default()
                })
            ],
            format!("Text inputted: {}", input_access.get())
        ]
    }
    "#,modal_content, my_input
        ) ,
        function_desc(
            "get_with",
            "`get_with`",
            Some("fn get_with<F: FnOnce(&T) -> R, R>(self, func: F) -> R"),
            "This method provides read access to a stored store variable via a closure.
This method is primarily used to read non-`Clone` values or where cloning is seen as expensive.

The typical pattern is to return a representation of the data stored from the `get_with()` closure. 
For instance, if a non-`Clone` struct that contains date information is stored then the closure might return
a `String` representation of this date information.

It is essential to understand that in order to provide unfettered read access to the stored value 
`get_with()` temporarily removes the value from the backing-store and re-inserts it at the end of the block. 
The effect of this is that any use of the `StateAccess`or within the `get_with()` closure is almost always an error.

The example demonstrates displaying a non Clone value stored by an accessor from an `Input` event.",
r#"
struct NonCloneString(String);

#[topo::nested]
fn my_non_clone_input() -> Node<Msg> {
    let input_access = use_state(|| NonCloneString("".to_string()));
    let val = input_access.get_with(|v| format!("{}", v.0));

    div![
        input![attrs![At::Value => val], 
            input_access.input_ev(Ev::Input, |i,text| *i=NonCloneString(text))
        ],
        format!("Text inputted: {}", val)
    ]
}
    "#,modal_content, my_non_clone_input
        ),
        function_desc(
            "set",
            "`set`",
            Some("fn set(self, value: T)"),
            "This method simply updates the stored value. `set()` is generally called in an `EventHandler` callback.
If the updated value depends on the current value it is generally better to use `update()` rather than `set()`

The example on the right set the value based on a clicked item in a list.",
r#"
#[topo::nested]
fn set_list() -> Node<Msg> {
    let selected = use_state(||"");

    ul![ "Selected Item:", selected.get(),
        li!["1st Item", mouse_ev(Ev::Click, move |_| { selected.set("1"); Msg::default() }), class![C.cursor_pointer]],
        li!["2nd Item", mouse_ev(Ev::Click, move |_| { selected.set("2"); Msg::default() }), class![C.cursor_pointer]],
        li!["3rd Item", mouse_ev(Ev::Click, move |_| { selected.set("3"); Msg::default() }), class![C.cursor_pointer]],
        li!["4th Item", mouse_ev(Ev::Click, move |_| { selected.set("4"); Msg::default() }), class![C.cursor_pointer]],
        li!["5th Item", mouse_ev(Ev::Click, move |_| { selected.set("5"); Msg::default() }), class![C.cursor_pointer]],
    ]
}
"#,modal_content, set_list
        ),
        function_desc(
            "update",
            "`update`",
            Some("fn update<F: FnOnce(&mut T) -> ()>(self, func: F)"),
            "This method simply updates the stored value by providing mutable access within a closure.
This is the prefered method if updating a value in place, particularly if the change depends on the existing value.

It is essential to understand that in order to provide unfettered read access to the stored value 
`update()` temporarily removes the value from the backing-store and re-inserts it at the end of the block. 
The effect of this is that any use of the `StateAccess`or within the `update()` closure is almost always an error.

The example on the right demonstrates a increasing / decreasing counter.",
r#"#[topo::nested]
fn update_example() -> Node<Msg> {
    let count = use_state(|| 3);

    div![
        button![
            "-",
            mouse_ev(Ev::Click, move |_| {
                count.update(|v| *v -= 1);
                Msg::NoOp
            }),
        ],
        count.get().to_string(),
        button![
            "+",
            mouse_ev(Ev::Click, move |_| {
                count.update(|v| *v += 1);
                Msg::NoOp
            }),
        ],
    ]
}
"#,modal_content, update_example
        ),
        ],
        section![
            section_desc(
                "dx",
                "Developer Experience",
                "Seed hooks provide a number of functions to simplify working with hooks."
            ),
            function_desc(
                "bind",
                "`bind`",
                Some("fn bind<Ms: Default, T: 'static + std::str::FromStr + std::fmt::Display>( attr: At,
val: StateAccess<T>,) -> (seed::virtual_dom::attrs::Attrs, seed::EventHandler<Ms>)"),
                "it is a common requirement that the value of element attributes such as an input's 
value attribute is linked to some value. `bind()` provides a shortcut to link an attribute to a value.
You simplfy specify the attribute and state accessor to bind.  Currently limited to updating on `Input` events, 
therefore currently only usable with `input![]` elements.

The example on the right binds integers to an input and then calculates a value with them. Under this example is 
the code for similar functionality but without using `bind()`.
",
r#"#[topo::nested]
fn numberbind() -> Node<Msg> {
    let a = use_state(|| 0);
    let b = use_state(|| 0);

    div![
        input![attrs![At::Type=>"number"], bind(At::Value, a)],
        input![attrs![At::Type=>"number"], bind(At::Value, b)],
        p![format!("{} + {} = {}", a, b, a + b)]
    ]
}


// Without bind there is a lot more boilerplate: 

#[topo::nested]
fn number_without_bind() -> Node<Msg> {
    let a = use_state(|| 0);
    let b = use_state(|| 0);

    div![
        input![attrs![At::Type=>"number", At::Value => a.get()], 
            input_ev(Ev::Input, |text| 
                {
                    if let Ok(a_i32) = text.parse::<i32>() {
                        a.set(a_i32)
                    }
                }
            )
        ],
        input![attrs![At::Type=>"number", At::Value => b.get()], 
        input_ev(Ev::Input, |text| 
            {
                if let Ok(b_i32) = text.parse::<i32>() {
                    b.set(a_i32)
                }
            }
        )
    ],
        p![format!("{} + {} = {}", a.get(), b.get(), a.get() + b.get())]
    ]
}

"#,modal_content, numberbind)
    ]  
    ,
    section![
        section_desc(
            "glossary",
            "Glossary",
            r#"
**Seed hook** - any of the Seed functions that facilitate storing and updating of *component state*.  For example, functions such as `use_state`, `new_state` and `bind`. The term *hook* refers to React Hooks which have similar functionality.

**Component state** - the collection of *state variables* that are used and stored by a component.

**State variable** - a variable that is stored locally in a component by `use_state` or `new_state`.  i.e. the integer value in:

```
let counter = use_state(||0);
```

**State accessor**  - responsible for getting, setting, and updating a *state variable*
within its linked *topological context*. i.e. `name` in:

```
let name = use_state(||"Bob");
```

**Topologically aware function** - a function that has been annotated with `[topo::nested]`.
This function will have its own `topo::Id`. I.e. the below function is topologically aware: 

```
#[topo::nested]
pub fn view(model: &Model) -> impl View<Msg> {
    div![]
}
```        

**Topological context** -  the execution context of a *topologically aware function*. Based on
where in the source the function was called, any parent topologically aware functions, and 
a `slot` which counts sibling functions. Represented by a `topo::Id` value. i.e. the two `child` function calls
have a different topological context.

```
#[topo::nested]
pub fn parent(model: &Model) -> impl View<Msg> {
    div![
        child(),
        child()
    ]
}

#[topo::nested]
pub fn child(model: &Model) -> impl View<Msg> {
    span!["hi]
}
```

**Topology** - The tree like structure that is created by nesting of *topologically aware functions*. If `a()`, `b()`, `c()`
and `d()` are such functions and they are called in the following way:

```

fn layout(){
    a();
}

#[topo::nested]
fn a(){
    b();
    b():
    c();
}

#[topo::nested]
fn b(){
    c();
    c();
}

#[topo::nested]
fn c(){
    d();
}

#[topo::nested]
fn d(){
}
```

then the *topology* of `layout()` is : 

```
                root(a)
     _______________________  
    |            |          |
    b()          b()        c()
 _____         _____        |
|     |       |     |       d()
c()   c()     c()   c()
|     |       |     |
d()   d()     d()   d()
```
The reason this is important is that each node above will have its own `topo::Id` which means it can be uniquely identified. Therefore state can be associated locally with each node even though it is the same function.

**Execution context** - Where in the above topology a function has been executed.


**Slot** - An index for sibling nodes in the above topology. Needed to differentiate between two sequential `c()` calls. Occasionally needed to be specified when iterating over components in a non-stable order.  









"#
        ),
    ]
    ]
}
