use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::ops::Deref;
use std::panic::Location;

/// Result type for the advent of code problems, defaulting the error type
pub type Result<Ok, Err = Report> = std::result::Result<Ok, Err>;

/// The main error type for AoC
pub struct Report(Box<ReportInner>); // Boxed to reduce the result size

/// A trait that allows wrapping the error with some context
pub trait WrapErr<T>: private::Sealed {
    /// Add some context to the error of this result if present (context is computed eagerly)
    #[track_caller]
    fn wrap_err(self, msg: &'static str) -> Result<T>;

    /// Add some context to the error of this result if present (context is computed on demand)
    #[track_caller]
    fn wrap_err_with(self, f: impl FnOnce() -> String) -> Result<T>;
}

impl<Ok, Err: ErrorExt> WrapErr<Ok> for Result<Ok, Err> {
    #[inline]
    #[track_caller]
    fn wrap_err(self, msg: &'static str) -> Result<Ok, Report> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(err.labelled(Cow::Borrowed(msg))),
        }
    }

    #[inline]
    #[track_caller]
    fn wrap_err_with(self, f: impl FnOnce() -> String) -> Result<Ok, Report> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => Err(err.labelled(Cow::Owned(f()))),
        }
    }
}

impl<Ok> WrapErr<Ok> for Option<Ok> {
    #[inline]
    #[track_caller]
    fn wrap_err(self, msg: &'static str) -> Result<Ok, Report> {
        match self {
            Some(ok) => Ok(ok),
            None => Err(Report::label(Cow::Borrowed(msg))),
        }
    }

    #[inline]
    #[track_caller]
    fn wrap_err_with(self, f: impl FnOnce() -> String) -> Result<Ok, Report> {
        match self {
            Some(ok) => Ok(ok),
            None => Err(Report::label(Cow::Owned(f()))),
        }
    }
}

/// Format an error message into an advent of code error
#[macro_export]
macro_rules! err {
    ($msg:literal $(,)?) => ({
        $crate::error::Report::label(::std::borrow::Cow::Owned(format!($msg)))
    });
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::Report::label(::std::borrow::Cow::Owned(format!($fmt, $($arg)*)))
    };
}

impl Report {
    #[track_caller]
    pub fn label(label: Cow<'static, str>) -> Self {
        let trace = vec![TraceElement::Context {
            label,
            location: Location::caller(),
        }];
        Self(Box::new(ReportInner { trace }))
    }

    #[track_caller]
    pub fn error(error: Box<dyn Error + Send + Sync + 'static>) -> Self {
        let trace = vec![TraceElement::Error {
            error,
            location: Location::caller(),
        }];
        Self(Box::new(ReportInner { trace }))
    }

    #[track_caller]
    pub fn labelled_error(
        label: Cow<'static, str>,
        error: Box<dyn Error + Send + Sync + 'static>,
    ) -> Self {
        let location = Location::caller();
        let trace = vec![
            TraceElement::Error { error, location },
            TraceElement::Context { label, location },
        ];
        Self(Box::new(ReportInner { trace }))
    }

    #[track_caller]
    pub fn wrap_err(mut self, label: Cow<'static, str>) -> Self {
        let location = Location::caller();
        self.0.trace.push(TraceElement::Context { label, location });
        self
    }
}

impl<Err: Error + Send + Sync + 'static> From<Err> for Report {
    #[track_caller]
    fn from(value: Err) -> Self {
        Self::error(Box::new(value))
    }
}

impl Deref for Report {
    type Target = dyn Error + Send + Sync + 'static;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for Report {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&self.0, f)
    }
}

impl Display for Report {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

/// Add a label to an error, turning it into a Report if it is not one already
trait ErrorExt {
    #[track_caller]
    fn labelled(self, label: Cow<'static, str>) -> Report;
}

/// The payload of an error report
struct ReportInner {
    trace: Vec<TraceElement>,
}

/// An element of the error report stack trace, either an erased error or a label
enum TraceElement {
    Context {
        label: Cow<'static, str>,
        location: &'static Location<'static>,
    },
    Error {
        error: Box<dyn Error + Send + Sync + 'static>,
        location: &'static Location<'static>,
    },
}

impl Debug for ReportInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.trace.last() {
            Some(e) => {
                write!(f, "{e:?}")?;
                for e in self.trace.iter().rev().skip(1) {
                    write!(f, "\n\t{e:?}")?;
                }
                Ok(())
            }
            None => write!(f, "empty error"), // Should not happen (always at least one element)
        }
    }
}

impl Display for ReportInner {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.trace.last() {
            Some(e) => write!(f, "{e}"),
            None => write!(f, "empty error"), // Should not happen (always at least one element)
        }
    }
}

impl Error for ReportInner {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // The source is the first element pushed on the stack trace if it is an error
        match self.trace.first() {
            Some(TraceElement::Error { error, .. }) => Some(&**error),
            _ => None,
        }
    }
}

impl Debug for TraceElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            TraceElement::Context { label, location } => {
                write!(f, "{label} (at {location})")
            }
            TraceElement::Error { error, location } => {
                write!(f, "{error:?} (at {location})")
            }
        }
    }
}

impl Display for TraceElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            TraceElement::Context { label, .. } => write!(f, "{label}"),
            TraceElement::Error { error, .. } => write!(f, "{error}"),
        }
    }
}

impl<E: Error + Send + Sync + 'static> ErrorExt for E {
    #[track_caller]
    fn labelled(self, label: Cow<'static, str>) -> Report {
        Report::labelled_error(label, Box::new(self))
    }
}

impl ErrorExt for Report {
    #[inline]
    #[track_caller]
    fn labelled(self, label: Cow<'static, str>) -> Report {
        self.wrap_err(label)
    }
}

/// Prevent extension of WrapErr from outside this module
mod private {
    pub trait Sealed {}
    impl<T, E: super::ErrorExt> Sealed for Result<T, E> {}
    impl<T> Sealed for Option<T> {}
}

#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    use super::*;

    #[test]
    fn literal_string_error() {
        let error = err!("hello error");
        assert_eq!(format!("{error}").as_str(), "hello error");
        assert!(format!("{error:?}").starts_with("hello error"));
    }

    #[test]
    fn templated_string_error_1() {
        let x = 3;
        let error = err!("hello template {x}, {a:?}", a = "errors");
        assert_eq!(format!("{error}").as_str(), "hello template 3, \"errors\"");
        assert!(format!("{error:?}").starts_with("hello template 3, \"errors\""));
    }

    #[test]
    fn templated_string_error_2() {
        let error = err!("positional {1}, {0:?}", 68, 78);
        assert_eq!(format!("{error}").as_str(), "positional 78, 68");
        assert!(format!("{error:?}").starts_with("positional 78, 68"));
    }

    #[test]
    fn erased_error() {
        let failed = "not an int".parse::<i32>();
        let convert = |e: Result<i32, ParseIntError>| -> Result<i32> { Ok(e?) };
        let error = convert(failed.clone()).unwrap_err();
        let wrapped = failed.unwrap_err();
        assert_eq!(format!("{error}").as_str(), format!("{wrapped}"));
        assert!(format!("{error:?}").starts_with(format!("{wrapped:?}").as_str()));
    }

    #[test]
    fn wrap_err() {
        let failed = "not an int".parse::<i32>();
        let convert = |e: Result<i32, ParseIntError>| e.wrap_err("this error is tragic");
        let error = convert(failed.clone()).unwrap_err();
        let wrapped = failed.unwrap_err();
        assert_eq!(format!("{error}").as_str(), "this error is tragic");
        let full_trace: Vec<String> = format!("{error:?}").lines().map(str::to_owned).collect();
        assert!(format!("{error:?}").starts_with("this error is tragic"));
        assert!(full_trace[0].starts_with("this error is tragic"));
        assert!(full_trace[1].starts_with(format!("\t{wrapped:?}").as_str()));
    }

    #[test]
    fn wrap_err_with() {
        let failed = "not an int".parse::<i32>();
        let convert = |e: Result<i32, ParseIntError>| {
            let x = 4;
            e.wrap_err_with(|| format!("formatted context {}, {x}", 3))
        };
        let error = convert(failed.clone()).unwrap_err();
        let wrapped = failed.unwrap_err();
        assert_eq!(format!("{error}").as_str(), "formatted context 3, 4");
        let full_trace: Vec<String> = format!("{error:?}").lines().map(str::to_owned).collect();
        assert!(full_trace[0].starts_with("formatted context 3, 4"));
        assert!(full_trace[1].starts_with(format!("\t{wrapped:?}").as_str()));
        assert_eq!(full_trace.len(), 2);
    }

    #[test]
    fn wrap_err_with_2() {
        let failed = "not an int".parse::<i32>();
        let convert = |e: Result<i32, ParseIntError>| {
            let x = 4;
            e.wrap_err_with(|| format!("first layer {}, {x}", 3))
                .wrap_err("second_layer")
                .wrap_err_with(|| format!("third layer {}", 42))
        };
        let error = convert(failed.clone()).unwrap_err();
        let wrapped = failed.unwrap_err();
        assert_eq!(format!("{error}").as_str(), "third layer 42");
        let full_trace: Vec<String> = format!("{error:?}").lines().map(str::to_owned).collect();
        assert!(full_trace[0].starts_with("third layer 42"));
        assert!(full_trace[1].starts_with("\tsecond_layer"));
        assert!(full_trace[2].starts_with("\tfirst layer 3, 4"));
        assert!(full_trace[3].starts_with(format!("\t{wrapped:?}").as_str()));
        assert_eq!(full_trace.len(), 4);
    }
}
