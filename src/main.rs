use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "basis", rename_all = "kebab-case")]
struct AppOptions {
    #[structopt(subcommand)]
    mode: Mode,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, StructOpt)]
pub enum Mode {
    Hash {
        #[structopt(short, long)]
        cost: Option<u32>,
        plain_texts: Vec<String>,
    },
    Match {
        plain_text: String,
        hash: String,
    },
}

fn main() -> Result<(), anyhow::Error>{
    let options = AppOptions::from_args();

    match options.mode {
        Mode::Hash { cost, plain_texts } => do_hash(cost, &plain_texts)?,
        Mode::Match { plain_text, hash} => do_match(&plain_text, &hash)?,
    }

    Ok(())
}

fn do_hash<S: AsRef<str>>(cost: Option<u32>, plain_texts: &[S]) -> Result<(), anyhow::Error> {
    for s in plain_texts {
        let hash = bcrypt::hash(s.as_ref(), cost.unwrap_or(bcrypt::DEFAULT_COST))?;
        println!("{} => {}", s.as_ref(), hash);
    }

    Ok(())
}

fn do_match(_plain_text: &str, _hash: &str) -> Result<(), anyhow::Error> {
    unimplemented!()
}
