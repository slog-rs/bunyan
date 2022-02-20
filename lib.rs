//! [Bunyan](https://www.npmjs.com/package/bunyan) formatting for `slog-rs`
//!
//! ```
//! #[macro_use]
//! extern crate slog;
//! extern crate slog_bunyan;
//!
//! use slog::Drain;
//! use std::sync::Mutex;
//!
//! fn main() {
//!     let root = slog::Logger::root(
//!                 Mutex::new(
//!                     slog_bunyan::default(
//!                         std::io::stderr()
//!                     )
//!                 ).fuse(),
//!                 o!("build-id" => "8dfljdf")
//!     );
//! }
//! ```
#![warn(missing_docs)]

use slog::{o, FnValue, Level, Record};

use std::io;

const DEFAULT_NAME: &str = "slog-rs";

fn get_hostname() -> String {
    match hostname::get() {
        Ok(h) => h.to_string_lossy().into_owned(),
        Err(_) => "n/a".to_string(),
    }
}

enum BunyanLevel {
    Fatal = 60,
    Error = 50,
    Warn = 40,
    Info = 30,
    Debug = 20,
    Trace = 10,
}

impl From<Level> for BunyanLevel {
    fn from(l: Level) -> Self {
        match l {
            Level::Critical => BunyanLevel::Fatal,
            Level::Error => BunyanLevel::Error,
            Level::Warning => BunyanLevel::Warn,
            Level::Info => BunyanLevel::Info,
            Level::Debug => BunyanLevel::Debug,
            Level::Trace => BunyanLevel::Trace,
        }
    }
}

fn new_with_ts_fn<F, W>(name: &'static str, io: W, ts_f: F) -> slog_json::JsonBuilder<W>
where
    F: Fn(&Record) -> Option<String> + Send + Sync + std::panic::RefUnwindSafe + 'static,
    W: io::Write,
{
    slog_json::Json::new(io).add_key_value(o!(
        "pid" => ::std::process::id(),
        "hostname" => get_hostname(),
        "time" => FnValue(ts_f),
        "level" => FnValue(|rinfo : &Record| {
            BunyanLevel::from(rinfo.level()) as i8
        }),
        "name" => name,
        "v" => 0usize,
        "msg" => FnValue(|rinfo : &Record| {
            rinfo.msg().to_string()
        })
    ))
}

/// Create `slog_json::FormatBuilder` with bunyan key-values
pub fn new<W>(io: W) -> slog_json::JsonBuilder<W>
where
    W: io::Write,
{
    with_name(DEFAULT_NAME, io)
}

/// Create `slog_json::Format` with bunyan key-values
pub fn default<W>(io: W) -> slog_json::Json<W>
where
    W: io::Write,
{
    with_name(DEFAULT_NAME, io).build()
}

/// Create `slog_json::FormatBuilder` with keys for the bunyan [core
/// fields](https://www.npmjs.com/package/bunyan#core-fields). The
/// value of the `name` parameter is used to populate the bunyan `name` field.
pub fn with_name<W>(name: &'static str, io: W) -> slog_json::JsonBuilder<W>
where
    W: io::Write,
{
    new_with_ts_fn(name, io, |_: &Record| {
        time::OffsetDateTime::now_local()
            .unwrap_or_else(|_| time::OffsetDateTime::now_utc())
            .format(&time::format_description::well_known::Rfc3339)
            .ok()
    })
}

#[cfg(test)]
mod test {
    use super::get_hostname;
    use super::new_with_ts_fn;
    use super::DEFAULT_NAME;
    use slog::{b, o};
    use slog::{Drain, Level, Logger};
    use slog::{Record, RecordLocation, RecordStatic};
    use std::io;
    use std::sync::{Arc, Mutex};
    use time::macros::datetime;

    struct V(Arc<Mutex<Vec<u8>>>);

    impl io::Write for V {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.lock().unwrap().write(buf)
        }
        fn flush(&mut self) -> io::Result<()> {
            self.0.lock().unwrap().flush()
        }
    }

    #[test]
    fn trivial() {
        let v = Arc::new(Mutex::new(vec![]));
        {
            let v = V(v.clone());
            let drain = new_with_ts_fn(DEFAULT_NAME, v, |_: &Record| {
                datetime!(2014-07-08 09:10:11 UTC)
                    .format(&time::format_description::well_known::Rfc3339)
                    .ok()
            })
            .build();

            let rs = RecordStatic {
                level: Level::Info,
                location: &RecordLocation {
                    file: "filepath",
                    line: 11192,
                    column: 0,
                    function: "",
                    module: "modulepath",
                },
                tag: "target",
            };

            let log = Logger::root(Mutex::new(drain).fuse(), o!());
            log.log(&Record::new(&rs, &format_args!("message"), b!()));
        }

        assert_eq!(
            String::from_utf8_lossy(&(*(*v).lock().unwrap())),
            "{".to_string()
                + "\"msg\":\"message\","
                + "\"v\":0,"
                + "\"name\":\"slog-rs\","
                + "\"level\":30,"
                + "\"time\":\"2014-07-08T09:10:11Z\","
                + "\"hostname\":\""
                + &get_hostname()
                + "\","
                + "\"pid\":"
                + &::std::process::id().to_string()
                + "}\n"
        );
    }

    #[test]
    fn custom_name_field() {
        let v = Arc::new(Mutex::new(vec![]));
        {
            let v = V(v.clone());
            let name = "test-name-123";
            let drain = new_with_ts_fn(name, v, |_: &Record| {
                datetime!(2014-07-08 09:10:11 UTC)
                    .format(&time::format_description::well_known::Rfc3339)
                    .ok()
            })
            .build();

            let rs = RecordStatic {
                level: Level::Info,
                location: &RecordLocation {
                    file: "filepath",
                    line: 11192,
                    column: 0,
                    function: "",
                    module: "modulepath",
                },
                tag: "target",
            };

            let log = Logger::root(Mutex::new(drain).fuse(), o!());
            log.log(&Record::new(&rs, &format_args!("message"), b!()));
        }

        assert_eq!(
            String::from_utf8_lossy(&(*(*v).lock().unwrap())),
            "{".to_string()
                + "\"msg\":\"message\","
                + "\"v\":0,"
                + "\"name\":\"test-name-123\","
                + "\"level\":30,"
                + "\"time\":\"2014-07-08T09:10:11Z\","
                + "\"hostname\":\""
                + &get_hostname()
                + "\","
                + "\"pid\":"
                + &::std::process::id().to_string()
                + "}\n"
        );
    }
}
