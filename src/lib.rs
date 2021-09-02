pub mod counter;

use pulldown_cmark::{Event, CowStr};

pub struct IgnoredPlaceholder;

impl IgnoredPlaceholder {
    pub fn to_event(&self) -> Event<'static> {
        Event::Html(ignored_placeholder().into())
    }

    pub fn to_cow_str(&self) -> CowStr<'static> {
        ignored_placeholder().into()
    }
}

fn ignored_placeholder() -> &'static str {
    r#"<div class="mdplayscript-count ignored-holder">Ignored</div>"#
}
