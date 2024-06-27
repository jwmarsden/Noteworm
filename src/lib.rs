use errors::BookwormError;
use log::trace;
use opts::Opts;

pub mod errors;
pub mod opts;

pub fn bookworm(opts: &Opts) -> Result<(), BookwormError> {
    trace!("Enter Bookworm. Dun dun dun. {:?}", opts);
    Ok(())
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
