use std::collections::HashSet;
use std::path::{Path, PathBuf};
use mdplayscript::renderer::HtmlRenderer;
use pulldown_cmark::Event;
use rand::Rng;

pub struct IgnorePatterns(Vec<glob::Pattern>);

impl IgnorePatterns {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn from_toml_values(v: &[toml::Value]) -> IgnorePatterns {
        let mut pat = Vec::new();

        for s in v.iter() {
            let s = match s.as_str() {
                Some(s) => s,
                None => {
                    log::warn!("given pattern is not a string: {}", s);
                    continue;
                },
            };
            match glob::Pattern::new(s) {
                Ok(p) => {
                    pat.push(p);
                },
                Err(e) => {
                    log::warn!("ignore pattern error: {}", e);
                },
            }
        }

        IgnorePatterns(pat)
    }

    pub fn matches_path(&self, path: &Path) -> bool {
        self.0
            .iter()
            .any(|pat| pat.matches_path(path))
    }
}

pub struct CounterFactory<'a> {
    prefix: &'a str,
    issued_class: HashSet<String>,
}

impl<'a> CounterFactory<'a> {
    pub fn new(prefix: &'a str) -> Self {
        Self {
            prefix: prefix,
            issued_class: HashSet::new(),
        }
    }

    pub fn issue(&mut self) -> Counter {
        loop {
            let class = generate_random_class_name(self.prefix);
            if self.issued_class.insert(class.clone()) {
                return Counter { class: class };
            }
        }
    }
}

fn generate_random_class_name(prefix: &str) -> String {
    let mut rng = rand::thread_rng();

    let tag: String = std::iter::repeat(())
        .map(|_| rng.sample(rand::distributions::Alphanumeric))
        .map(char::from)
        .take(10)
        .collect();

    format!("{}{}", prefix, tag)
}

pub struct Counter {
    class: String,
}

impl Counter {
    pub fn set_class_to_renderer(&self, renderer: &mut HtmlRenderer) {
        renderer.speech_classes.add(&self.class);
    }

    pub fn generate_placeholder(&self, ignored: &IgnorePatterns, src: Option<&PathBuf>) -> Vec<Event<'static>> {
        let div = match src {
            Some(src) if ignored.matches_path(src) => {
                log::info!("Ignore {}",src.display());
                crate::IgnoredPlaceholder.to_cow_str()
            },
            _ => {
                log::info!("Process {:?}", src);
                format!(r#"<div class="mdplayscript-count {}"></div>"#, self.class)
                    .into()
            },
        };

        vec![Event::Html(div.into())]
    }
}
