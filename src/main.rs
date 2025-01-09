use noteworm::{noteworm, errors::NotewormError, opts::Opts};

fn main() -> Result<(), NotewormError> {
    env_logger::init();
    let opts = Opts::parse_args();
    //println!("{opts:#?}");
    noteworm(&opts)?;
    Ok(())
}

