use std::error::Error;
use std::fmt;

pub struct ErrChainFmt<'a, E: ?Sized> {
    err: &'a E,
    opts: FmtOpts,
}

struct FmtOpts {
    compact: bool,
}

impl<'a, E: ?Sized> ErrChainFmt<'a, E> {}

impl<'a, E> fmt::Display for ErrChainFmt<'a, E>
where
    E: Error + ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.err)?;

        let mut source = self.err.source();
        while let Some(cause) = source {
            let delimiter = match self.opts {
                FmtOpts { compact: true, .. } => ": ",
                FmtOpts { compact: false, .. } => "\n  Caused by: ",
            };
            write!(f, "{}{}", delimiter, cause)?;
            source = cause.source();
        }

        Ok(())
    }
}

/// An extension trait for [`Error`] types to display their sources in a chain.
pub trait ErrChainFmtExt {
    /// Provides an [fmt::Display] implementation for an error as a chain.
    fn err_chain_fmt(&self) -> ErrChainFmt<'_, Self>;
}

impl<E> ErrChainFmtExt for E
where
    E: Error + ?Sized,
{
    fn err_chain_fmt(&self) -> ErrChainFmt<'_, Self> {
        ErrChainFmt {
            err: self,
            opts: FmtOpts { compact: true },
        }
    }
}
