use crate::{generated::css_classes::C, Msg, Page};
use comp_state::{
    do_once, topo, use_state, use_state_unique, ChangedState, CloneState, StateAccess,
};
use comp_state_seed_extras::{
    after_render, after_render_once, bind, get_html_element_by_id, StateAccessEventHandlers,
    UpdateElLocal,
};
use comrak::{markdown_to_html, ComrakOptions};
use wasm_bindgen::JsCast;

use seed::{prelude::*, *};
pub fn view() -> Node<Msg> {
    div![
        id!("page"),
        header![
            style!{St::GridArea => "header"},
            class![
                C.shadow_xl,
                C.bg_gray_600,
                C.text_gray_200,
                C.flex,
                C.justify_end,
                C.content_center,
                C.items_center
            ],
            a![
                class![
                    C.h_full,
                    C.py_2,
                    C.px_2,
                    C.mx_4,
                    C.hover__text_white,
                    C.text_gray_500,
                    C.pointer_events_none
                ],
                attrs!(At::Href => ""),
                "SEED HOOKS"
            ],
            div![class![C.h_6, C.border_r_2]],
            a![
                class![
                    C.h_full,
                    C.py_2,
                    C.px_2,
                    C.mx_4,
                    C.hover__text_white,
                ],
                attrs!(At::Href => Page::Tutorial.to_href()),
                "TUTORIAL"
            ],
            div![class![C.h_6, C.border_r_2]],
            a![
                class![
                    C.h_full,
                    C.py_2,
                    C.px_2,
                    C.mx_4,
                    C.hover__text_white,
                ],
                attrs!(At::Href => Page::ApiRef.to_href()),
                "API REFERENCE"
            ],
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
        h1!["TUTORIAL"],
        hr![class![C.my_8 C.border_b_2 C.border_gray_200]],
        ul![
            li![a![
                class![
                    C.ml_2,
                    C.hover__text_gray_100,
                    C.border_b_2,
                    C.border_transparent,
                    C.hover__border_gray_300
                ],
                attrs![At::Href=>"tutorial#start_here"],
                "Start Here"
            ]],
            li![a![
                class![
                    C.ml_2,
                    C.hover__text_gray_100,
                    C.border_b_2,
                    C.border_transparent,
                    C.hover__border_gray_300
                ],
                attrs![At::Href=>"tutorial#step1"],
                "Step 1 - Design & component data flow",
            ]],
            li![a![
                class![
                    C.ml_2,
                    C.hover__text_gray_100,
                    C.border_b_2,
                    C.border_transparent,
                    C.hover__border_gray_300
                ],
                attrs![At::Href=>"tutorial#step2"],
                "Step 2 - Markdown processing",
            ]],
            li![a![
                class![
                    C.ml_2,
                    C.hover__text_gray_100,
                    C.border_b_2,
                    C.border_transparent,
                    C.hover__border_gray_300
                ],
                attrs![At::Href=>"tutorial#step3"],
                "Step 3 - Prettifying the output",
            ]],
            li![a![
                class![
                    C.ml_2,
                    C.hover__text_gray_100,
                    C.border_b_2,
                    C.border_transparent,
                    C.hover__border_gray_300
                ],
                attrs![At::Href=>"tutorial#step4"],
                "Step 4 - Adding auto scrolling",
            ]],
            li![a![
                class![
                    C.ml_2,
                    C.hover__text_gray_100,
                    C.border_b_2,
                    C.border_transparent,
                    C.hover__border_gray_300
                ],
                attrs![At::Href=>"tutorial#step5"],
                "Step 5 - Adding message submission",
            ]],
            li![a![
                class![
                    C.ml_2,
                    C.hover__text_gray_100,
                    C.border_b_2,
                    C.border_transparent,
                    C.hover__border_gray_300
                ],
                attrs![At::Href=>"tutorial#step6"],
                "Step 6 - Reusing the markdown editor",
            ]],
        ],
    ]
}

// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Prism)]
    pub fn highlightAll();
    #[wasm_bindgen(js_namespace = Prism)]
    pub fn highlightElement(el: web_sys::HtmlElement);
    #[wasm_bindgen(js_namespace = Prism)]
    fn highlightAllUnder(el: web_sys::HtmlElement);
}

// Prism.highlightElement(element, async, callback)

#[topo::nested]
fn section_desc<T: Into<String>>(href_name: T, title: T, description: T) -> Vec<Node<Msg>> {
    let drop_type = use_state(crate::DropType::default);

    let mut opts = ComrakOptions::default();
    opts.github_pre_lang = true;

    let title = markdown_to_html(&title.into(), &opts);
    let description = markdown_to_html(&description.into(), &opts);

    let desc_el = use_state(ElRef::<web_sys::HtmlElement>::default);
    do_once(|| drop_type.update(|dt| dt.dropped = true));

    if drop_type.get().dropped {
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
                    highlightElement(code_el.dyn_into::<web_sys::HtmlElement>().unwrap());
                }
            }
        });
    }

    nodes![
        h2![
            class![C.m_3, C.text_2xl],
            a![attrs![At::Name=>href_name.into()], raw!(&title)]
        ],
        hr![class![C.my_8 C.border_b_2 C.border_gray_200]],
        div![
            el_ref(&desc_el.get()),
            class!["api-description" C.m_3 C.leading_relaxed],
            raw!(&description)
        ],
    ]
}

fn main_screen_content() -> Node<Msg> {
    div![
        section_desc(
            "start_here",
            "Start Here",
            r#"
# Getting Started

In this tutorial we will use **Seed Hooks** to create a markdown editor component that can freely re-used in any Seed app.

You can see a demo of the finished component by clicking [here](/tutorial_example).

To assist checking progress in this tutorial please see here for the [final code](https://github.com/rebo/markdown-editor).

What we are going to do in this section is 

1. Clone and run the Seed quickstart
1. Install and setup nightly Rust for this project
1. Install TailwindCSS
1. Setup Seed Hooks

## Before we start 


## Clone and run the Seed quickstart

Clone the Seed basic quick start:

```
git clone https://github.com/seed-rs/seed-quickstart markdown-editor
cd markdown-editor
```

To run this quickstart project you need cargo-make installed:

```
cargo install cargo-make
```
This will install cargo-make in your `~/.cargo/bin`.
Make sure to add `~/.cargo/bin` directory to your PATH variable.

You will also need to ensure that Rust can target wasm by adding this component:

```
rustup target add wasm32-unknown-unknown
```

Check it compiles and serves correctly with:

```
cargo make build; cargo make serve
```

You can access the site from `http://localhost:8000`. This will display a simple button counter.

Currently **Seed Hooks** only work on nightly rust, this is due to requiring the feature `TrackCaller` therefore it is 
important to install a recently nightly. The below has been built with the nightly of 26th February 2020. 

To install nightly rust do this:

```
rustup install nightly
```

To ensure that nightly is used for only this project add a `rust-toolchain` file to the project root, the contents of this file should be the single line `nightly`. 

## CSS 

We will be using [TailwindCSS](https://tailwindcss.com) for our CSS. Therefore we need to set this up.  Create a `package.json` in the project root with
these contents:

```
// In package.json...
{
    "name": "markdown-editor",
    "version": "0.0.1"
}
```

Also add `/node_modules` to your `.gitignore` file. 

Next install TailwindCss with 

```
yarn add tailwindcss
```

Next create a `styles.css` in the project root with these contents: 

```
// In styles.css...

@tailwind base;

.markdown-body ol, .markdown-body ul {
    list-style-type: unset;
}

@tailwind components;

@tailwind utilities;
```

The ".markdown-body" classes are needed later because Tailwind removes bullet points 
which is needed for rendering `<li>` tags.

Now build the TailwindCSS with:

```
npx tailwindcss build styles.css -o output.css
```

and add this to the `index.html`:

```
// In index.html...

<head>
    ...
    <link rel="stylesheet" type="text/css" href="output.css">
    ...
</head>
```

### Seed Hooks Setup

In order to enable **Seed Hooks** add the following to `Cargo.toml` in the `[dependencies]` section. 

```
// In Cargo.toml...

comp_state = "0.2.1"
comp_state_seed_extras = "0.0.8"
```

Next, Seed hooks rely on the nightly `TrackCaller` feature you need to add the `#![feature(track_caller)]` feature flag to the top of `lib.rs`.

Remove all existing `Model` and `Msg` fields/variants. You will also want to remove the match processing of `Msg` in your update function.

You should also glob import both `comp_state` and `comp_state_seed_extras` with 

```
// In in lib.rs...

use comp_state::*;
use comp_state_seed_extras::*;
```

The final bit of setup required is to add a root component to the Seed view. This is achieved by annotating 
the main Seed view function with `#[topo::nested]`. For now replace the contents of the root view with a simple `div![]`.

```
#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
    div![]
}
```

This annotation means that the view function becomes part of the component hierarchy. Indeed this acts as the root compnent 
under which all other components are structured.

The final base `lib.rs` should be as per below :

```
// In in lib.rs...

#![feature(track_caller)]
use comp_state::*;
use comp_state_seed_extras::*;
use seed::{prelude::*, *};

#[derive(Default)]
struct Model {}

enum Msg {}

fn update(msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {}

#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
    div![]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
```
"#
        ),
        section_desc(
            "step1",
            "Step 1 - Design & component data flow",
            r#"
### Design

We want to create a reusable markdown renderer component. The only brief is that the user should be able to pass a message
to the component which sends the rendered html to Seed on submission.

What we are going to do in this section is 

1. Design the rough layout and styling for the component
1. Decide what state needs to be stored locally
1. Ensure state can be outputted into a preview div

Visually we want a split view, the left effectively being one big textbox, the right being a preview render of the markdown.
We also want a submit button below both these panes. Add the below code to `lib.rs` to setup the above layout.


```
// In in lib.rs...
#[topo::nested]
fn markdown_editor() -> Node<Msg> {
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
                class!["bg-red-300 h-full flex-none w-1/2"],
                attrs![At::Type => "textbox"],
            ],
            div![
                class!["markdown-body bg-yellow-300 h-full flex-none w-1/2"],
            ]
        ],
        div![
            class!["flex justify-end pt-2"],
            button![class!["bg-green-400 p-4 m-2"], "Submit (See console log)"]
        ]
    ]
} 
```
The reason we annotate with `#[topo::nested]` is so that `markdown_editor` can operate as its own component with local state. 

This component can be rendered by calling it in the root view:

```
#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
    markdown_editor()
}
```

Currently nothing in the page is functional, the above code only sets up the layout.

### Cargo Watch

At this stage it would be worth setting up a cargo watch loop to rebuild the wasm file and re-serve so that you can see your changes
more immediately in the browser.

Run the following in separate terminal windows

```
cargo make serve
```
and 
```
cargo make watch
```

`cargo make serve` will ensure that your server is always running and `cargo make watch` will automatically re-compile the `.wasm` file. 
Therefore the only thing that you will need to do is refresh the browser after updating any of your rust files.



### Data Flow

So what do we want our component to do? When someone types in the textarea we want the contents to be processed
by a markdown processor and the results viewable in the preview box on the right.

Ideally we would like the preview box to be in sync with the textarea cursor.

Datawise, this means we need a **state variable** to store the current textarea content, this then gets processed by a filter 
on `Input` event.  

In order to create this **state variable** we will use the first (and most used) hook function `use_state()`. Add the following
at the top of the function.

```
// In in lib.rs...
fn markdown_editor() -> Node<Msg> {
    let source = use_state(|| String::new());
    ...
```
This creates a **state variable** accessor `source`. This accessor is used to get and set some `String` data associated with this component. 

Next we need to bind this source to the `value` attribute of the textarea. Modify the `textarea!` to include this a bind directive.

```
// In in lib.rs...
fn markdown_editor() -> Node<Msg> {
    ...
    textarea![
        bind(At::Value, source),
        ...
]
```

You will get an error that `Msg` does not implement `Default`. The reason for this error is that currently all Seed `EventHandler`s need to
return a Msg to the Seed app. This requirement will be lifted in the future.  To fix this add a default implementation to the `Msg` enum.

```
// In in lib.rs...

enum Msg {
    NoOp,
}

impl Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
}
```
If you still have a `match msg` in your `update()` function you will need to add this variant to the match. i.e.

```

fn update(msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::NoOp => {}
    }
}
```

Lastly lets ensure that the bind is working correctly and we will output the raw `textarea![]` input into the preview div:

```
// In in lib.rs...
fn markdown_editor() -> Node<Msg> {
    ...

    div![
        class!["markdown-body bg-yellow-300 h-full flex-none w-1/2"],
        source.get()
    ]
    ...
```
Refreshing your browser now (`https://localhost:8000`) and typing in the textarea should output the text directly within the 
markdown preview div.

"#
        ),
        section_desc(
            "step2",
            "Step 2 - Markdown processing",
            r#"

What we are going to do in this section is 

1. Process the source state variable as markdown prior to rendering in the view.

### How to process

We now have a basic bind set up, updating the textarea will update the `source` state variable.
This is then directly output to the preview div.

Instead of outputting directly to the preview div, we want it to be processed as markdown. 
Fortunately Seed has an in-built macro that renders markdown from a `&str`.

Simply wrap `source.get()` in `md!(&source.get())` in the markdown preview div:

```
// In in lib.rs...

div![
    class!["markdown-body bg-yellow-300 h-full flex-none w-1/2"],
    md!(&source.get())
]
```

Here is the final `markdown_editor` function at this stage. 

```
#[topo::nested]
fn markdown_editor() -> Node<Msg> {
    let source = use_state(|| String::new());

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
                bind(At::Value, source),
                class!["bg-red-300 h-full flex-none w-1/2"],
                attrs![At::Type => "textbox"],
            ],
            div![
                class!["markdown-body bg-yellow-300 h-full flex-none w-1/2"],
                md!(&source.get())
            ]
        ],
        div![
            class!["flex justify-end pt-2"],
            button![class!["bg-green-400 p-4 m-2"], "Submit (See console log)"]
        ]
    ]
}
```
"#
        ),
        section_desc(
            "step3",
            "Step 3 - Prettifying the output",
            r#"

What we are going to do in this section is 

1. Use a Github styled markdown CSS
1. Fix content overflows in the preview div
1. Improve the visual look of the textarea

### Github styled markdown CSS

The UI currently is functional but it can be improved, specifically regarding the preview render.

We therefore need to style both to better improve the UI.  TailwindCSS by default does a normalise
pass on all styles. 

We will use `github-markdown-css` for this, we can simply use the CDN version of this file:

```
// in index.html
... 
<head>
    ...
    <link rel="stylesheet" type="text/css" href="output.css">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/4.0.0/github-markdown.min.css">
    .... 
```

Because we have already used the class `markdown-body` for our markdown preview div then the CSS should just work.

1. Fix content overflows in the preview div

Currently the styled processed markdown will overflow the preview div. Therefore we need to ensure that `overflow` is set to auto
for this div. Furthermore we can adjust the styling on the div for an improved look. Modify the existing `class!`s  as follows: 

```
/// in lib.rs
div![
    class!["markdown-body"],
    class!["overflow-auto p-2 pl-4 h-full flex-none w-1/2 border-gray-200 bg-indigo-100 border shadow-lg"],
```

This will ensure the preview pane's markdown is rendered correctly.

### Improve the look of the textarea

Furthermore we woud like the textarea input to be mono-space. Therefore adjust its class:

```
// In in lib.rs...

textarea![
    ...
    class!["font-mono p-2 h-full flex-none w-1/2 border-gray-200 border shadow-lg"],
    ...
],
```            

Lets try how it all works now, save the file refresh the browser. Try typing the following into the textarea: 

```
# Seed Rocks

**Yes** indeed it does *rock*.

```"#
        ),
        section_desc(
            "step4",
            "Step 4 - Adding auto scrolling",
            r#"
### Auto scrolling the preview

When we edit the textarea we ideally would like the preview to scroll to a similar position. 
This would enable our edits to be easier to see. Therefore we want to programatically scroll the `markdown-body` div 
on `KeyUp` and also on `Scroll` events.

In order to do this we need to identify the preview div and also the textarea with `ElRef`s. 
These are Seed's way of identifying individual elements. 

Due to the fact that we are going to refer to specific html elements via `web_sys` we need to add that as a dependency.

In `Cargo.toml` add the following to the dependencies section: 

```
[dependencies]
...
web-sys = "0.3.35"
...
```

and enable access to the following types at the top of `lib.rs`:

```
use web_sys::{HtmlElement, HtmlTextAreaElement};
```

after the `let source = use_state..` line add two more `use_state` hooks. 

```
// In in lib.rs...

let preview_el = use_state(ElRef::<HtmlElement>::default);
let textarea_el = use_state(ElRef::<HtmlTextAreaElement>::default);
```

This provides access to two el_refs which we can later associate with specific elements. 

In order to do this we use the `el_ref()` function within the respective elements...

```
// In in lib.rs...

textarea![
    el_ref(&textarea_el.get()),
    ...
```

and 

```
// In in lib.rs...

div![
    el_ref(&preview_el.get()),
    class!["markdown-body"],
    ...
```

In order to set the respective scroll on the preview we use a simple percentage of textarea
scroll as a guide.

This is achieved via the following event handler, add this to the bottom of the `textarea![]`
node:

```
// In in lib.rs...
textarea![
    ...

    ...
    textarea_el.input_ev(Ev::KeyUp, move |el, _| {
        if let (Some(textarea), Some(preview)) = (el.get(), preview_el.get().get()) {
            let scroll_percentage = (textarea.scroll_top() as f64) / (textarea.scroll_height() as f64);
            let new_scroll_top = (preview.scroll_height() as f64) * scroll_percentage;
            preview.set_scroll_top(new_scroll_top as i32);
        }
    }),
]
```

There are some issues with this simple percentage based scroll above. For markup that 
results in larger rendered markdown, such as headers, the scrolling will not perfectly match up.

Furthermore we also need an identical `EventHandler` callback for an `Ev::Scroll` event.  You could cut and paste code above
however we can prevent needless repetition by using a function.  

We fix both these issues by using the code below.

First, remove the `EventHandler` above and add the following function to `lib.rs`.

```
//in lib.rs 

fn scroll_event_handler(
    event: Ev,
    textarea_el: StateAccess<ElRef<HtmlTextAreaElement>>,
    preview_el: StateAccess<ElRef<HtmlElement>>
) -> EventHandler<Msg> {
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
```

Finally, the following two lines at the bottom of the `textarea![]` will generate the correct event handlers:

```
//in lib.rs  

textarea![
    ...

    ... 
    scroll_event_handler(Ev::KeyUp ,textarea_el, preview_el),
    scroll_event_handler(Ev::Scroll, textarea_el, preview_el),

]
```

Once all the above is completed, scrolling and cursor navigating through the textarea will 
result in a corresponding scroll of the preview div.


Try pasting the following into the textarea and try scrolling or moving the cursor around:

```
* this
* is
* a 
* very
* long
* list
* that
* goes
* on
* and
  1. on
  1. and
  1. on
  1. it
* Should
* demonstrate
* Auto scrolling the preview 
* in response to 
* scrolling of the 
* text area
* and cursor movement

```

"#
        ),
        section_desc(
            "step5",
            "Step 5 - Adding Message submission",
            r#"
We now modify the function signature to allow an arbitrary message to be passed to Seed's update function.
The message will be sent to Seed when the submit button is pressed.

The message should permit a String argument which should be the content of the rendered markdown. 
Hence we will use the following `markdown_editor()` function signature: 

```
// In in lib.rs...

fn markdown_editor(on_submit: impl FnOnce(String) -> Msg + 'static + Clone) -> Node<Msg>
```

We modify the `Msg` type to add a variant that accepts a `String` argument.

```
// In in lib.rs...

enum Msg {
    NoOp,
    SubmitMarkdownHtml(String),
}
```

We add an `Ev::Click` event handler to the submit button, this sends the `Msg` to Seed.

```
// In in lib.rs...

button![
    class!["bg-green-400 p-4 m-2"],
    "Submit (See console log)",
    mouse_ev(Ev::Click, move |_| {
        let markdown_element = preview_el.get().get().expect("markdown-body doesn't exist");
        on_submit(markdown_element.inner_html())
    })
]
```

We handle the `Msg` in the Seed app's update function by printing it to the console. The last tweak is adjusting the calling function in the view:

```
// In in lib.rs...

fn update(msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::SubmitMarkdownHtml(html) => log!(html),
        Msg::NoOp => {}
    }
} 

#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
    markdown_editor(Msg::SubmitMarkdownHtml)
}
```

Now when the form button is clicked, an output of the processed html will be
logged to the console from the Seed update function.

The final `lib.rs` file is below:

```
#![feature(track_caller)]
use comp_state::*;
use comp_state_seed_extras::*;
use seed::{prelude::*, *};
use web_sys::{HtmlElement, HtmlTextAreaElement};


#[derive(Default)]
struct Model {}

enum Msg {
    NoOp,
    SubmitMarkdownHtml(String),
}

impl Default for Msg {
    fn default() -> Self {
        Msg::NoOp
    }
}

fn update(msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::SubmitMarkdownHtml(html) => log!(html),
        Msg::NoOp => {}
    }
}

#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
    markdown_editor(Msg::SubmitMarkdownHtml)
}

#[topo::nested]
fn markdown_editor(on_submit: impl FnOnce(String) -> Msg + 'static + Clone) -> Node<Msg> {
    let source = use_state(|| String::new());
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
                    on_submit(markdown_element.inner_html())
                })
            ]
        ]
    ]
}

fn scroll_event_handler(
    event: Ev,
    textarea_el: StateAccess<ElRef<HtmlTextAreaElement>>,
    preview_el: StateAccess<ElRef<HtmlElement>>
) -> EventHandler<Msg> {
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

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}

```
    "#
        ),
        section_desc(
            "step6",
            "Step 6 - Reusing the markdown editor",
            r#"
So far we have created a functional live markdown editor with local state
which stores, processes, and renders the markdown source in a preview pane.

This editor can be re-used freely in the current Seed app by simply calling the `markdown_editor` function
freely multiple times.  For instance changing the view to the following creates four markdown editors
all of which function independently.

```
#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
    div![
        markdown_editor(Msg::SubmitMarkdownHtml),
        markdown_editor(Msg::SubmitMarkdownHtml),
        markdown_editor(Msg::SubmitMarkdownHtml),
        markdown_editor(Msg::SubmitMarkdownHtml),
    ]
}
```

However there is an issue if we want to re-use this component in a different app. This is because the component
currently relies on specifically that the `Msg` type to be used as the `on_submit` argument and in the return type as part of a `Node<Msg>`.

We therefore need to adjust the code to allow for this function to be freely re-used in any Seed application that may use a completely different
message type.  

In order to do this we will make the function generic over the message type.

Change the `markdown-editor` function signature as follows:

```
#[topo::nested]
fn markdown_editor<Ms, F>(on_submit: F) -> Node<Ms>
where
    F: FnOnce(String) -> Ms + 'static + Clone,
    Ms: Default + 'static,
{
        ...
```
What we have done is create a generic function rather than a function that is specific for our app's `Msg` type.
The `Ms` type is a type parameter that we supply to the function to tell it what message type it should use.

Because `Ms` will be used in `bind` it must implement Default, therefore we state that `Ms` must have this trait bound.

We also need to ensure that all parts of our component refer to this generic `Ms` type.

If you look in the function body of `markdown_editor` you will not see any `Msg` type referenced directly,
and therefore you might think that no further changes are needed. However have a look again at the `scroll_event_handler` function: 

```
fn scroll_event_handler<Msg>(
    event: Ev,
    textarea_el: StateAccess<ElRef<HtmlTextAreaElement>>,
    preview_el: StateAccess<ElRef<HtmlElement>>,
) -> EventHandler<Ms> {
```

This return an `EventHandler<Msg>` and the `Msg` is concrete here!

To fix we simply use a generic type parameter here as well. So replace the `scroll_event_handler` signature with the below:  

```
fn scroll_event_handler<Ms>(
    event: Ev,
    textarea_el: StateAccess<ElRef<HtmlTextAreaElement>>,
    preview_el: StateAccess<ElRef<HtmlElement>>,
) -> EventHandler<Ms>
where
    Ms: Default + 'static,
{
```

Now this function will use the a generic `Ms` type as well. Rust is smart enough to
realise that it has to use the same `Ms` type due to the return type of `markdown_editor`. 

Once the above changes are made then we can call our markdown editor as below and use it freely in any Seed app.

```
#[topo::nested]
fn view(_model: &Model) -> impl View<Msg> {
    markdown_editor(Msg::SubmitMarkdownHtml)
}
```

The great thing is because the `Ms` type is defined by the argument passed to `on_submit` (in this case `Msg::SubmitMarkdownHtml`) **we don't actually have to 
explicitly state the message type to be used**. Our api surface is clean and easy to use.

### Taking the widget further.  

As it stands the widget is composable, versatile and fulfills the brief.   It can be
freely used in any Seed project (as long as hooks have been enabled) and can be rendered to the 
view with a simple function call. 

That said, it could be extended in a number of ways.  Here are some ideas:

* Configurable styling / theme. 
* A "Cancel" button along with associated Seed messaging.  
* Buttons and shortcuts to quickly add things like bolding text. I.e. a short cut that bolds
the highlighted word on `CTRL/CMD-B`.
* A "Reset" button that clears the edit pane.
* A version with a 'preview' and 'edit' tab instead of side by side panes.  
* A Seed Hooks version - instead of taking a `Msg` argument it takes a Seed Hooks state accessor. 
The widget then updates the accessor's state variable on submit.

For the last suggestion, the only things you would need to adjust are the function signature for the `markdown_editor` 
and the submit button callback:

```
// in alt_md.rs

pub fn markdown_editor_state<Ms, F>(md_state: StateAccess<String>) -> Node<Ms>
where
    F: FnOnce(String) -> Ms + 'static + Clone,
    Ms: Default + 'static,
{
    ...
    button![
        class!["bg-green-400 p-4 m-2"],
        "Submit (See console log)",
        mouse_ev(Ev::Click, move |_| {
            let markdown_element = preview_el.get().get().expect("markdown-body doesn't exist");
            md_state.set(markdown_element.inner_html()); // changed line
            Ms::default() // return default noop message.
        })
    ]
    ...
}
```
an [example](https://github.com/rebo/markdown-editor/blob/master/src/alt_md.rs) of this is in the [demo repository](https://github.com/rebo/markdown-editor) here.

"#
        ),
    ]
}
