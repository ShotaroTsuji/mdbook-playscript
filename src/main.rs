use structopt::StructOpt;
use mdplayscript::MdPlayScript;
use mdbook::preprocess::{Preprocessor, PreprocessorContext, CmdPreprocessor};
use mdbook::book::Book;

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

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> mdbook::errors::Result<Book> {
        Ok(book)
    }
}
