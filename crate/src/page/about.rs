use crate::{generated::css_classes::C, Msg};
use seed::{prelude::*, *};

#[allow(clippy::too_many_lines)]
pub fn view() -> impl View<Msg> {
    div![
        class![C.flex_grow,],
        // Photo section
        section![
            // Small photo background container
            div![],
            // Large photo background
        ],
        // Developer section
        section![div![]],
    ]
}
