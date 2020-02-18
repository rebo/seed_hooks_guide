use crate::{
    asset_path, generated::css_classes::C, image_src, Msg, MAIL_TO_REBO,
};
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
