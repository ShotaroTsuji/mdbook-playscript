use structopt::StructOpt;
use rust_embed::RustEmbed;
use pulldown_cmark::Parser;
use pulldown_cmark_to_cmark::cmark;
use mdplayscript::interface::*;
use mdbook::preprocess::{PreprocessorContext, CmdPreprocessor};
use mdbook::book::{Book, BookItem};

#[derive(Debug,StructOpt)]
struct PlayScriptOpt {
    #[structopt(subcommand)]
    command: Option<Command>,
}

#[derive(Debug,StructOpt)]
enum Command {
    Supports {
        renderer: String,
    },
}

fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    let opt = PlayScriptOpt::from_args();

    //eprintln!("{:#?}", opt);

    let preprocessor = PlayScriptPreprocessor::new();

    let result = match opt.command {
        Some(Command::Supports { renderer }) => {
            handle_renderer(preprocessor, &renderer)
        },
        _ => {
            handle_preprocessing(preprocessor)
        },
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn handle_renderer(prep: PlayScriptPreprocessor, renderer: &str) -> ! {
    if prep.supports_renderer(renderer) {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}

fn handle_preprocessing(prep: PlayScriptPreprocessor) -> Result<(), mdbook::errors::Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(std::io::stdin())?;

    let book = prep.run(&ctx, book)?;
    serde_json::to_writer(std::io::stdout(), &book)?;

    Ok(())
}

struct PlayScriptPreprocessor {
}

impl PlayScriptPreprocessor {
    fn new() -> Self {
        Self {
        }
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        match renderer {
            "html" => true,
            _ => false,
        }
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> mdbook::errors::Result<Book> {
        let css = Stylesheet::from_context(ctx);
        css.copy(ctx);

        let options = match ctx.config.book.language.as_ref() {
            Some(lang) if lang == "ja" => Options::default_ja(),
            _ => Options::default(),
        };

        let params = Params {
            title: ctx.config.book.title.clone(),
            subtitle: ctx.config.get("preprocessor.playscript.subtitle")
                .and_then(|v| v.as_str())
                .map(|s| s.to_owned()),
            authors: ctx.config.book.authors.clone(),
        };

        let title_conj = ctx.config.get("preprocessor.playscript.title-conjunction")
            .and_then(|v| v.as_str())
            .map(|s| s.to_owned());
        log::info!("title-conjunction: {:?}", title_conj);

        book.for_each_mut(|book_item| {
            match book_item {
                BookItem::Chapter(chapter) => {
                    let title_conj = title_conj.clone();
                    let len = chapter.content.len();
                    let mut content = String::new();
                    std::mem::swap(&mut chapter.content, &mut content);

                    let parser = MdPlayScriptBuilder::new()
                        .params(params.clone())
                        .options(options.clone())
                        .make_title(Box::new(move |params| make_title_fn(params, title_conj.as_ref())))
                        .build(Parser::new(&content));
                    let mut processed = String::with_capacity(len + len/2);
                    cmark(parser, &mut processed, None).unwrap();
                    //eprintln!("{}", processed);
                    std::mem::swap(&mut chapter.content, &mut processed);
                },
                _ => {},
            }
        });

        Ok(book)
    }
}

fn make_title_fn(params: &Params, conj: Option<&String>) -> String {
    let mut cover = "<div class=\"cover\">".to_owned();
    if let Some(title) = params.title.as_ref() {
        cover += &format!("<h1 class=\"cover-title\">{}</h1>", title);
    }

    if let Some(subtitle) = params.subtitle.as_ref() {
        if let Some(conj) = conj.as_ref() {
            cover += &format!("<p class=\"cover-conjunction\">{}</p>", conj);
        }
        cover += &format!("<p class=\"cover-subtitle\">{}</p>", subtitle);
    }

    if !params.authors.is_empty() {
        cover += "<div class=\"cover-authors\">";

        for author in params.authors.iter() {
            cover += &format!("<p class=\"cover-author\">{}</p>", author);
        }

        cover += "</div>";
    }

    cover += "</div>";

    cover
}

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/public"]
struct Asset;

struct Stylesheet {
    filename: &'static str,
}

impl Stylesheet {
    fn from_context(ctx: &PreprocessorContext) -> Self {
        assert_eq!(ctx.renderer.as_str(), "html");

        let filename = match ctx.config.book.language.as_ref() {
            Some(lang) if lang == "ja" => "mdplayscript_ja.css",
            _ => "mdplayscript.css",
        };

        Self {
            filename: filename,
        }
    }

    fn copy(&self, ctx: &PreprocessorContext) {
        let mut path = ctx.root.clone();
        assert!(path.exists(), "root directory does not exist");

        path.push(self.filename);

        //eprintln!("copy to {:?}", path);
        let css = Asset::get(self.filename).unwrap();
        std::fs::write(&path, &css).unwrap();
    }
}
