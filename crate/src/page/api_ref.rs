use crate::{generated::css_classes::C, Msg};
use comp_state::{topo, use_state, CloneState};
use comp_state_seed_extras::after_render_once;
use comrak::{markdown_to_html, ComrakOptions};
use wasm_bindgen::JsCast;

use seed::{prelude::*, *};
pub fn view() -> Node<Msg> {
    div![
        class![C.flex C.flex_row],
        div![
            class![
                C.w_1of4,
                C.h_screen,
                C.bg_gray_700,
                C.text_gray_400,
                C.overflow_y_auto
            ],
            left_bar_content(),
        ],
        div![
            class![C.w_3of4, C.h_screen, C.overflow_y_auto],
            main_screen_content()
        ],
    ]
}

fn left_bar_content() -> Node<Msg> {
    div![
        h1![a![
            class![C.hover__text_gray_100],
            attrs![At::Href=>"api_ref#start_here"],
            "Start Here"
        ]],
        ul![
            a![
                class![C.hover__text_gray_100],
                attrs![At::Href=>"api_ref#state_functions"],
                "State Functions"
            ],
            li![a![
                class![C.hover__text_gray_100],
                attrs![At::Href=>"api_ref#use_state"],
                "use_state"
            ]],
            li![a![
                class![C.hover__text_gray_100],
                attrs![At::Href=>"api_ref#use_state_unique"],
                "use_state_unique"
            ]],
            li![a![
                class![C.hover__text_gray_100],
                attrs![At::Href=>"api_ref#use_state_current"],
                "use_state_current"
            ]],
        ],
        ul![
            a![
                class![C.hover__text_gray_100],
                attrs![At::Href=>"api_ref#condittional"],
                "Conditional Execution Functions"
            ],
            li![a![
                class![C.hover__text_gray_100],
                attrs![At::Href=>"api_ref#do_once"],
                "do_once"
            ]],
            li![a![
                class![C.hover__text_gray_100],
                attrs![At::Href=>"api_ref#after_render"],
                "after_render"
            ]],
        ],
        ul![
            a![
                class![C.hover__text_gray_100],
                attrs![At::Href=>"api_ref#state_access"],
                "StateAccess<T> Struct"
            ],
            li![a![
                class![C.hover__text_gray_100],
                attrs![At::Href=>"api_ref#use_state"],
                "get"
            ]],
            li![a![
                class![C.hover__text_gray_100],
                attrs![At::Href=>"api_ref#get_with"],
                "get_with"
            ]],
            li![a![
                class![C.hover__text_gray_100],
                attrs![At::Href=>"api_ref#set"],
                "set"
            ]],
            li![a![
                class![C.hover__text_gray_100],
                attrs![At::Href=>"api_ref#update"],
                "update"
            ]],
            li![a![
                class![C.hover__text_gray_100],
                attrs![At::Href=>"api_ref#use_state"],
                "changed"
            ]],
            li![a![
                class![C.hover__text_gray_100],
                attrs![At::Href=>"api_ref#events"],
                "..events"
            ]]
        ],
        ul![
            a![attrs![At::Href=>"api_ref#dx"], "DX Functions"],
            li![a![
                class![C.hover__text_gray_100],
                attrs![At::Href=>"api_ref#bind"],
                "bind"
            ]],
        ],
        ul![
            "Utility Functions",
            li![a![
                class![C.hover__text_gray_100],
                attrs![At::Href=>"api_ref#use_list"],
                "use_list"
            ]],
        ]
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

    let mut opts = ComrakOptions::default();
    opts.github_pre_lang = true;

    let title = markdown_to_html(&title.into(), &opts);
    let description =
        markdown_to_html(&description.into(), &opts);

        let desc_el = use_state(ElRef::<web_sys::HtmlElement>::default);

        after_render_once(move |_| {
            if let Some(desc_el) = desc_el.get().get() {
                let code_children = desc_el.get_elements_by_tag_name("h3");

                for idx in 0..code_children.length(){
                    let code_el = code_children.item(idx).unwrap();
                    code_el.set_class_name("text-xl py-3");
                }

                let code_children = desc_el.get_elements_by_tag_name("code");

                for idx in 0..code_children.length(){
                    let code_el = code_children.item(idx).unwrap();
                    code_el.set_class_name("language-rust");
                    highlightElement(code_el.dyn_into::<web_sys::HtmlElement>().unwrap());
                }
            }        
    });

    nodes![
        h2![
            class![C.m_3, C.text_2xl],
            a![attrs![At::Name=>href_name.into()], raw!(&title)]
        ],
        hr![class![C.my_8 C.border_b_2 C.border_gray_200]],
        div![el_ref(&desc_el.get()), class![C.m_3 C.leading_relaxed], raw!(&description)],
    ]
}

#[topo::nested]
fn function_desc<T: Into<String>>(
    href_name: T,
    title: T,
    signature: Option<&str>,
    description: T,
    code: T,
) -> Node<Msg> {
    let href_name = href_name.into();
    let title = markdown_to_html(&title.into(), &ComrakOptions::default());
    let description =
        markdown_to_html(&description.into(), &ComrakOptions::default());
    let code = code.into();
    let code_el = use_state(ElRef::<web_sys::HtmlElement>::default);

    let desc_el = use_state(ElRef::<web_sys::HtmlElement>::default);

    after_render_once(move |_| {
        if let Some(code_el) = code_el.get().get() {

            if let Some(desc_el) = desc_el.get().get() {
                let code_children = desc_el.get_elements_by_tag_name("code");

                for idx in 0..code_children.length(){
                    let code_el = code_children.item(idx).unwrap();
                    code_el.set_class_name("language-rust");
                    highlightElement(code_el.dyn_into::<web_sys::HtmlElement>().unwrap());
                }
            }        

            highlightElement(code_el);
        }
    });

    div![
        h3![
            class![C.m_3, C.text_xl],
            a![attrs![At::Name=> href_name], raw!(&title)]
        ],
        if let Some(sig) = signature {
            pre![class![C.p_4],code![sig]] }
             else {empty![]},
        div![
            class![C.flex C.flex_row],
            div![el_ref(&desc_el.get()),class![C.p_3, C.w_1of2 C.flex_none], raw!(&description)],
            div![
                class![C.p_3, C.w_1of2 C.flex_none],
                pre![
                    class!(C.rounded, C.shadow),
                    code![
                        class!("language-rust"),
                        el_ref(&code_el.get()),
                        code
                    ]
                ]
            ]
        ]
    ]
}

fn main_screen_content() -> Node<Msg> {
    div![
        section![section_desc(
            "#start_here",
            "Start Here",
            r#"**Seed Hooks** are an implementation of local component state in Seed:

```rust
#[topo::nested]
fn name_input() -> Node<Msg> {
    let name = use_state(|| "".to_string());

    div![
        input![bind(At::Value, name)],
        format!("Hello {}", name.get())
    ]
}
```
In the above code `name` is an accessor for a local **state variable** which is then bound to the `input!` field's value.

### Why are **Seed hooks** needed?

Seed hooks allow 'components' to have their own state and those components can then be freely composed and re-used at will. Due to this they are ideal
for functionality that does not need to touch the main Seed View->Message->Update->View loop. For instance a dropdown menu toggle, individual input element
state, or modal dialog visibiility. 

Due to component behaviour being freely composable complex components can be created and re-used such as date pickers which do not need to be wired into the main app.

### Setup

`use_state` is the principal function to access local state, individual comoponents are identified by annotation with `#[topo::nested]`.

`#[topo::nested]` functions have a unique id which is based on the function's parent call hierarchy, callsite, and an indexed slot. 
This enables topologically aware functions to be considered as unique components with local state.

The only setup required is to ensure the root seed view function directly calls a `#[topo::nested]` function that acts as the root for the call heirachy. 
The following typically suffices:

```rust
use comp_state::{topo, use_state};

pub fn view(model: &Model) -> impl View<Msg> {
    root_view(model)
}

#[topo::nested]
pub fn root_view(model: &Model) -> Node<Msg> {
    ...
}
```

At present if event handlers helpers are to be used then the `Message` type should also implement a `default()` no-op. This restriction will be lifted eventually:

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
This api guide summarises the hooks and functions currently available in two crates:

a. [comp_state](https://github.com/rebo/comp_state)  
b. [comp_state_seed_extras](https://github.com/rebo/comp_state_seed_extras)

Only the main functions are described here there are many more for use in specific circumstances, 
please refer to the `doc.rs` documentation for a full list.
"#,
        )],
        section![
            section_desc(
                "#state_functions",
                "State Functions",
                "Seed hooks' **state functions** are functions that relate to the storing of local state for a component.
The primary function used is `use_state` which stores an arbitary value and returns an accessor struct. The other functions are used
in specific situations, of which `use_state_unique` is covered here."
            ),
            function_desc(
                "#use_state",
                "`use_state`",
                Some("fn use_state<T: 'static, F: FnOnce() -> T>(data_fn: F) -> StateAccess<T>"),
                "`use_state` is the standard state function for storing of local state for a component. It returns a `StateAccess` accessor which
is responsible for all getting, setting and updating of the underlying value.

The function takes a lazily evaluated closure which returns a value that gets stored on first execution. 

The code snippet on the right demonstrates the use of `use_state` to store a count which gets updated on a button click.
",
r#"#[topo::nested]
fn my_button() -> Node<Msg> {
    let count = use_state(|| 0);

    div![
        count.get().to_string(),
        button!["+", count.mouse_ev(Ev::Click, |count, _| *count += 1)],
    ]
}"#
            ),
            function_desc(
                "#use_state_unique",
                "`use_state_unique`",
                Some("fn use_state_unique<T: 'static, F: FnOnce() -> T>(data_fn: F) -> StateAccess<T>"),
                r#"This function is identical to `use_state` with the exception that every time the function is executed it creates a new 
topological context. The closure runs on every execution.

The use-case for this is to allow creation of state variables and associated accessors in an event callback.

Consider the following code, it will create a state variable when the button is clicked. But because the callsite, parent call tree, 
and slot are all identical it will refer to the same `topoId`.

```
button![
    todos.mouse_ev(Ev::Click, move |t| t.push(use_state(String::new))),
    "Add" 
]
```
The problem with this is that every state accessor stored within the todo list will refer to the same component. 
Simply using `use_state_unique` will ensure that every accessor stored will refer to a new topological context:

```
button![
    todos.mouse_ev(Ev::Click, move |t| t.push(use_state_unique(String::new))),
    "Add" 
]
```
The code example on the right is a fully interactive todo list in 15 line of code. `todos` is a state accessor that stores 
a `Vec` of `String` state accessors, these are then used to store the state of each todo. 

`use_state_unique` is used in the on click event do add a new unique todo.
"#,
r#"#[topo::nested]
fn todos() -> Node<Msg> {
    let todos = use_state(|| vec![use_state(String::new)]);
    div![
        todos.get().iter().enumerate().map(|(idx, todo)| {
            vec![
                input![bind(At::Value, *todo)],
                button![
                    todos.mouse_ev(Ev::Click, move |t| t.remove(idx)),
                    "X" 
                ]
                br![],
            ]
        }),
        button![
            todos.mouse_ev(Ev::Click, move |t| t.push(use_state_unique(String::new))),
            "Add" 
        ]
    ]
}"#
            ),
        ],
        /* section![
         *     h2![a![attrs![At::Name=>"#conditional"], "Conditional Functions"]],
         *     p!["Introduction here"],
         *     function_desc(
         *         "#do_once",
         *         "do_once",
         *         "this is the do_once",
         *         "fn main()->{}",
         *     )
         * ],
         * section![
         *     h2![a![attrs![At::Name=>"#state_access"], "StateAccess Struct"]],
         *     p!["Introduction here"],
         *     function_desc(
         *         "#state_access_get",
         *         "get",
         *         "this is the get",
         *         "this is the for the get"
         *     ),
         *     function_desc(
         *         "#state_access_set",
         *         "set",
         *         "this is the StateAccess set",
         *         "this is the code for the StateAcess set"
         *     ),
         *     function_desc(
         *         "#state_access_update",
         *         "use_state_current",
         *         "this is the use state access update function",
         *         "this is the code for the state access update"
         *     )
         * ],
         * section![
         *     h2![a![attrs![At::Name=>"#dx"], "Developer Experience"]],
         *     p!["Introduction here"]
         * ], */
    ]
}
