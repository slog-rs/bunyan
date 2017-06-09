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

#[macro_use]
extern crate slog;
extern crate nix;
extern crate chrono;
extern crate slog_json;

use slog::{Record, Level, FnValue};

use std::io;

fn get_hostname() -> String {

    let mut buf = vec!(0u8; 256);
    match nix::unistd::gethostname(&mut buf) {
        Ok(hostname_c) => {
            // TODO: BUG: use locale to known encoding?
            hostname_c.to_string_lossy().into()
        }
        Err(_) => "n/a".to_string(),
    }
}

fn level_to_string(level: Level) -> i8 {
    match level {
        Level::Critical => 60,
        Level::Error => 50,
        Level::Warning => 40,
        Level::Info => 30,
        Level::Debug => 20,
        Level::Trace => 10,
    }
}

fn new_with_ts_fn<F, W>(io : W, ts_f: F) -> slog_json::JsonBuilder<W>
    where F: Fn(&Record) -> String + Send + Sync + std::panic::RefUnwindSafe + 'static,
          W : io::Write
{
    slog_json::Json::new(io)
        .add_key_value(o!(
            "pid" => nix::unistd::getpid() as usize,
            "hostname" => get_hostname(),
            "time" => FnValue(ts_f),
            "level" => FnValue(|rinfo : &Record| {
                level_to_string(rinfo.level())
            }),
            // TODO: slog loggers don't have names...
            "name" => "slog-rs",
            "v" => 0usize,
            "msg" => FnValue(|rinfo : &Record| {
                rinfo.msg().to_string()
            })
        ))
}

/// Create `slog_json::FormatBuilder` with bunyan key-values
pub fn new<W>(io : W) -> slog_json::JsonBuilder<W>
where
          W : io::Write
{
    new_with_ts_fn(io, |_: &Record| chrono::Local::now().to_rfc3339())
}

/// Create `slog_json::Format` with bunyan key-values
pub fn default<W>(io : W) -> slog_json::Json<W>
where
          W : io::Write {
    new_with_ts_fn(io, |_: &Record| chrono::Local::now().to_rfc3339()).build()
}

#[cfg(test)]
mod test {
    use super::new_with_ts_fn;
    use super::get_hostname;
    use chrono::{TimeZone, UTC};
    use nix;
    use slog::{Record, RecordStatic, RecordLocation};
    use slog::{Level, Drain, Logger};
    use std::sync::{Mutex, Arc};
    use std::io;

    struct V(Arc<Mutex<Vec<u8>>>);

    impl io::Write for V {
        fn write(&mut self, buf : &[u8]) -> io::Result<usize> {
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
            let drain =
                new_with_ts_fn(v, |_: &Record| UTC.ymd(2014, 7, 8).and_hms(9, 10, 11).to_rfc3339()).build();


            let rs = RecordStatic {
                level: Level::Info,
                location : &RecordLocation {
                    file: "filepath",
                    line: 11192,
                    column: 0,
                    function: "",
                    module: "modulepath",
                },
                tag : "target"
            };

            let log = Logger::root(Mutex::new(drain).fuse(), o!());
            log.log(&Record::new(&rs, &format_args!("message"), b!()));
        }

        assert_eq!(String::from_utf8_lossy(&(*(*v).lock().unwrap())),
                   "{".to_string() +
                   "\"msg\":\"message\"," +
                   "\"v\":0," +
                   "\"name\":\"slog-rs\"," +
                   "\"level\":30," +
                   "\"time\":\"2014-07-08T09:10:11+00:00\"," +
                   "\"host\":\"" + &get_hostname() + "\"," +
                   "\"pid\":" + &nix::unistd::getpid().to_string() +
                   "}\n");
    }
}
