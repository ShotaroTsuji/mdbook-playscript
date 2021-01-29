use std::path::Path;
use structopt::StructOpt;
use rust_embed::RustEmbed;
use pulldown_cmark::Parser;
use pulldown_cmark_to_cmark::cmark;
use mdplayscript::MdPlayScript;
use mdbook::preprocess::{Preprocessor, PreprocessorContext, CmdPreprocessor};
use mdbook::book::{Book, BookItem};

#[derive(Debug,StructOpt)]
enum Opt {
    #[structopt(name="mdbook-preprocessor")]
    MdBookPreprocessor(PlayScriptOpt),
}

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
    let opt = Opt::from_args();

    eprintln!("{:#?}", opt);

    let preprocessor = PlayScriptPreprocessor::new();

    match opt {
        Opt::MdBookPreprocessor(opt) => {
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
        },
    }
}

fn handle_renderer<P: Preprocessor>(prep: P, renderer: &str) -> ! {
    if prep.supports_renderer(renderer) {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}

fn handle_preprocessing<P: Preprocessor>(prep: P) -> Result<(), mdbook::errors::Error> {
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
}

impl Preprocessor for PlayScriptPreprocessor {
    fn name(&self) -> &str {
        "mdplayscript"
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        match renderer {
            "html" => true,
            _ => false,
        }
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> mdbook::errors::Result<Book> {
        copy_css(&ctx.root);

        book.for_each_mut(|book_item| {
            match book_item {
                BookItem::Chapter(chapter) => {
                    let len = chapter.content.len();
                    let mut content = String::new();
                    std::mem::swap(&mut chapter.content, &mut content);

                    let parser = MdPlayScript::new(Parser::new(&content));
                    let mut processed = String::with_capacity(len + len/2);
                    cmark(parser, &mut processed, None).unwrap();
                    eprintln!("{}", processed);
                    std::mem::swap(&mut chapter.content, &mut processed);
                },
                _ => {},
            }
        });

        Ok(book)
    }
}

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/public"]
struct Asset;

const CSS_FILE_NAME: &'static str = "mdplayscript.css";

fn copy_css<P: AsRef<Path>>(root: P) {
    let mut path = root.as_ref().to_path_buf();
    assert!(path.exists(), "root directory does not exist");

    path.push(CSS_FILE_NAME);

    eprintln!("copy to {:?}", path);
    let css = Asset::get(CSS_FILE_NAME).unwrap();
    std::fs::write(&path, &css).unwrap();
}
