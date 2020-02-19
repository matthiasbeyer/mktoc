use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "mktoc")]
struct Cli {
    #[structopt()]
    file: String,

    #[structopt(long, short)]
    write: bool,

    #[structopt(long, short = "m", default_value = "1", env="MKTOC_MIN_DEPTH")]
    min_depth: i32,

    #[structopt(long, short = "M", default_value = "6", env="MKTOC_MAX_DEPTH")]
    max_depth: i32,
}

fn handle_write(new_toc: String) {
    let opts = Cli::from_args();
    if opts.write {
        let res_write = std::fs::write(opts.file, new_toc.as_bytes());
        match res_write {
            Ok(_r) => {
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Failed to write file. Error kind: {:?}", e.kind());
                std::process::exit(1);
            }
        }
    } else {
        println!("{}", new_toc);
    }
}

fn main() {
    let opts = Cli::from_args();
    let res = mktoc::make_toc(opts.file, opts.min_depth, opts.max_depth);

    match res {
        Ok(new_toc) => {
            handle_write(new_toc);
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
}
