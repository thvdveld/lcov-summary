use anyhow::Result;

use lcov_summary::Lcov;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    lcov_file: std::path::PathBuf,
    /// Only show the summary.
    #[arg(short, long)]
    summary: bool,

    diff_lcov_file: Option<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let lcov = Lcov::parse(args.lcov_file)?;

    let lcov2 = if let Some(file2) = args.diff_lcov_file {
        Some(Lcov::parse(file2)?)
    } else {
        None
    };

    if args.summary {
        if let Some(lcov2) = lcov2 {
            lcov.diffsummarystd(&lcov2);
        } else {
            lcov.summarystd();
        }
    } else {
        lcov.printstd();
    }

    Ok(())
}
