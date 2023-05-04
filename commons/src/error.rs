use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Arguments, Debug, Display, Formatter, Result as FmtResult};
use std::ops::Deref;
use std::panic::Location;

/// Result type for the advent of code problems, defaulting the error type
pub type Result<Ok, Err = Report> = std::result::Result<Ok, Err>;

/// The main error type for AoC
pub struct Report(Box<ErrorTrace>); // Boxed to reduce the result size

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
/// An error that is reported when `wrap_err` or `wrap_err_with` is used on an empty `Option`
#[derive(Debug, Clone, Copy)]
pub struct EmptyOptionError;

impl Display for EmptyOptionError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Option was empty")
    }
}

impl Error for EmptyOptionError {}

impl<Ok> WrapErr<Ok> for Option<Ok> {
    #[inline]
    #[track_caller]
    fn wrap_err(self, msg: &'static str) -> Result<Ok, Report> {
        match self {
            Some(ok) => Ok(ok),
            None => Err(Report::labelled_error(
                Cow::Borrowed(msg),
                Box::new(EmptyOptionError),
            )),
        }
    }

    #[inline]
    #[track_caller]
    fn wrap_err_with(self, f: impl FnOnce() -> String) -> Result<Ok, Report> {
        match self {
            Some(ok) => Ok(ok),
            None => Err(Report::labelled_error(
                Cow::Owned(f()),
                Box::new(EmptyOptionError),
            )),
        }
    }
}

/// Format an error message into a report
#[macro_export]
macro_rules! err {
    ($msg:literal $(,)?) => ({
        $crate::error::Report::formatted(::std::format_args!($msg))
    });
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::Report::formatted(::std::format_args!($fmt, $($arg)*))
    };
}

/// Format an error message into a report and return it directly
#[macro_export]
macro_rules! bail {
    ($msg:literal $(,)?) => ({
        return Err($crate::error::Report::formatted(::std::format_args!($msg)))
    });
    ($fmt:expr, $($arg:tt)*) => {
         return Err($crate::error::Report::formatted(::std::format_args!($fmt, $($arg)*)))
    };
}

/// Check a condition, and return an error if it is not true
#[macro_export]
macro_rules! ensure {
    ($cond:expr) => ({
        if (!$cond) {
            let msg = ::std::format_args!(::std::concat!("Condition failed: ", ::std::stringify!($cond)));
            return Err($crate::error::Report::formatted(msg));
        }
    });
    ($cond:expr, $msg:literal $(,)?) => ({
        if (!$cond) {
            return Err($crate::error::Report::formatted(::std::format_args!($msg)));
        }
    });
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if (!$cond) {
            return Err($crate::error::Report::formatted(::std::format_args!($fmt, $($arg)*)));
        }
    };
}

impl Report {
    #[doc(hidden)]
    #[track_caller]
    pub fn formatted(args: Arguments<'_>) -> Self {
        let trace = vec![TraceElement::Context {
            label: match args.as_str() {
                Some(literal) => Cow::Borrowed(literal),
                None => Cow::Owned(args.to_string()),
            },
            location: Location::caller(),
        }];
        Self(Box::new(ErrorTrace { trace }))
    }

    #[track_caller]
    pub fn wrap_err(mut self, label: Cow<'static, str>) -> Self {
        let location = Location::caller();
        self.0.trace.push(TraceElement::Context { label, location });
        self
    }

    #[track_caller]
    fn error(error: Box<dyn Error + Send + Sync + 'static>) -> Self {
        let trace = vec![TraceElement::Error {
            error,
            location: Location::caller(),
        }];
        Self(Box::new(ErrorTrace { trace }))
    }

    #[track_caller]
    fn labelled_error(
        label: Cow<'static, str>,
        error: Box<dyn Error + Send + Sync + 'static>,
    ) -> Self {
        let location = Location::caller();
        let trace = vec![
            TraceElement::Error { error, location },
            TraceElement::Context { label, location },
        ];
        Self(Box::new(ErrorTrace { trace }))
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
struct ErrorTrace {
    /// A stack of trace elements (not empty by construction), 0th is the source error (or label)
    trace: Vec<TraceElement>,
}

/// An element of the error trace, either an erased error or a label
#[derive(Debug)]
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

impl Debug for ErrorTrace {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if f.alternate() {
            f.debug_struct("ErrorTrace")
                .field("trace", &self.trace)
                .finish()
        } else if let Some(e) = self.trace.last() {
            match e {
                TraceElement::Context { label, location } => {
                    write!(f, "{label} (at {location})")
                }
                TraceElement::Error { error, location } => {
                    write!(f, "{error} (at {location})")
                }
            }?;
            self.trace.iter().rev().skip(1).try_for_each(|e| match e {
                TraceElement::Context { label, location } => {
                    write!(f, "\n\t{label} (at {location})")
                }
                TraceElement::Error { error, location } => {
                    write!(f, "\n\t{error} (at {location})")
                }
            })
        } else {
            write!(f, "empty error") // Should not happen (always at least one element)
        }
    }
}

impl Display for ErrorTrace {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.trace.last() {
            Some(e) => match e {
                TraceElement::Context { label, .. } => write!(f, "{label}"),
                TraceElement::Error { error, .. } => write!(f, "{error}"),
            },
            None => write!(f, "empty error"), // Should not happen (always at least one element)
        }
    }
}

impl Error for ErrorTrace {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // The source is the first element pushed on the error trace if it is an error
        match self.trace.first() {
            Some(TraceElement::Error { error, .. }) => Some(&**error),
            _ => None,
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
        assert!(error.source().is_none());
    }

    #[test]
    fn templated_string_error_1() {
        let x = 3;
        let error = err!("hello template {x}, {a:?}", a = "errors");
        assert_eq!(format!("{error}").as_str(), "hello template 3, \"errors\"");
        assert!(format!("{error:?}").starts_with("hello template 3, \"errors\""));
        assert!(error.source().is_none());
    }

    #[test]
    fn templated_string_error_2() {
        let error = err!("positional {1}, {0:?}", 68, 78);
        assert_eq!(format!("{error}").as_str(), "positional 78, 68");
        assert!(format!("{error:?}").starts_with("positional 78, 68"));
        assert!(error.source().is_none());
    }

    #[test]
    fn erased_error() {
        let failed = "not an int".parse::<i32>();
        let convert = |e: Result<i32, ParseIntError>| -> Result<i32> { Ok(e?) };
        let error = convert(failed.clone()).unwrap_err();
        let wrapped = failed.unwrap_err();
        assert_eq!(format!("{error}").as_str(), format!("{wrapped}"));
        assert!(format!("{error:?}").starts_with(format!("{wrapped }").as_str()));
        assert!(error
            .source()
            .unwrap()
            .downcast_ref::<ParseIntError>()
            .is_some());
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
        assert!(full_trace[1].starts_with(format!("\t{wrapped}").as_str()));
        assert!(error
            .source()
            .unwrap()
            .downcast_ref::<ParseIntError>()
            .is_some());
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
        assert!(full_trace[1].starts_with(format!("\t{wrapped}").as_str()));
        assert_eq!(full_trace.len(), 2);
        assert!(error
            .source()
            .unwrap()
            .downcast_ref::<ParseIntError>()
            .is_some());
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
        assert!(full_trace[3].starts_with(format!("\t{wrapped}").as_str()));
        assert_eq!(full_trace.len(), 4);
        assert!(error
            .source()
            .unwrap()
            .downcast_ref::<ParseIntError>()
            .is_some());
    }

    #[test]
    fn wrap_err_option() {
        let empty = Option::<i32>::None;
        let error = empty.wrap_err("a").wrap_err_with(|| format!("b {}", 2));
        let error = error.unwrap_err();
        assert_eq!(format!("{error}").as_str(), "b 2");
        let full_trace: Vec<String> = format!("{error:?}").lines().map(str::to_owned).collect();
        assert!(full_trace[0].starts_with("b 2"));
        assert!(full_trace[1].starts_with("\ta"));
        assert!(full_trace[2].starts_with("\tOption was empty"));
        assert_eq!(full_trace.len(), 3);
        assert!(error
            .source()
            .unwrap()
            .downcast_ref::<EmptyOptionError>()
            .is_some());
    }

    #[test]
    fn bail_test_literal() {
        fn test_fn() -> Result<i32> {
            bail!("hello bail")
        }

        let error = test_fn().unwrap_err();
        assert_eq!(format!("{error}"), "hello bail");
    }

    #[test]
    fn bail_test_formatted_1() {
        fn test_fn() -> Result<i32> {
            let x = 3;
            bail!("test bail {x}")
        }

        let error = test_fn().unwrap_err();
        assert_eq!(format!("{error}"), "test bail 3");
    }

    #[test]
    fn bail_test_formatted_2() {
        fn test_fn() -> Result<i32> {
            bail!("template {x}, {1} {0}", 44, "second", x = "named")
        }

        let error = test_fn().unwrap_err();
        assert_eq!(format!("{error}"), "template named, second 44");
    }

    #[test]
    fn ensure_test_no_message() {
        fn test_fn(ok: &str) -> Result<i32> {
            ensure!(ok == "ok");
            Ok(3)
        }

        let ok = test_fn("ok").unwrap();
        assert_eq!(ok, 3);
        let not_ok = test_fn("not ok").unwrap_err();
        assert_eq!(format!("{not_ok}"), "Condition failed: ok == \"ok\"");
    }

    #[test]
    fn ensure_test_literal() {
        fn test_fn(ok: &str) -> Result<i32> {
            ensure!(ok == "ok", "woops");
            Ok(3)
        }

        let ok = test_fn("ok").unwrap();
        assert_eq!(ok, 3);
        let not_ok = test_fn("not ok").unwrap_err();
        assert_eq!(format!("{not_ok}"), "woops");
    }

    #[test]
    fn ensure_test_formatted_1() {
        fn test_fn(ok: &str) -> Result<i32> {
            ensure!(ok == "ok", "test {ok}");
            Ok(3)
        }

        let ok = test_fn("ok").unwrap();
        assert_eq!(ok, 3);
        let not_ok = test_fn("not ok").unwrap_err();
        assert_eq!(format!("{not_ok}"), "test not ok");
    }

    #[test]
    fn ensure_test_formatted_2() {
        fn test_fn(ok: &str) -> Result<i32> {
            ensure!(
                ok == "ok",
                "message {text:?} {1} {0}",
                true,
                3,
                text = "this is some text"
            );
            Ok(3)
        }

        let ok = test_fn("ok").unwrap();
        assert_eq!(ok, 3);
        let not_ok = test_fn("not ok").unwrap_err();
        assert_eq!(format!("{not_ok}"), "message \"this is some text\" 3 true");
    }
}
