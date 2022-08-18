mod fmt;

pub use fmt::{ErrChainFmt, ErrChainFmtExt};

#[cfg(test)]
mod tests {
    use anyhow::anyhow;

    use crate::ErrChainFmtExt;

    #[test]
    fn compact_format() {
        let err = anyhow!("error 0").context("error 1").context("error 2");
        println!("{}", err.err_chain_fmt());
        assert_eq!(err.err_chain_fmt().to_string(), "error 2: error 1: error 0");
    }
}
