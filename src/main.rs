use bookworm::{bookworm, errors::BookwormError, opts::Opts};

fn main() -> Result<(), BookwormError> {
    env_logger::init();
    let opts = Opts::parse_args();
    println!("{opts:#?}");
    bookworm(&opts)?;
    Ok(())
}

