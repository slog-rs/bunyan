# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

* Update to Rust 2021
* Update MSRV to Rust 1.59

## 2.4.0 - 2022-02-19
### Changed

* Switch from `chrono` to `time` (PR #11 by @ShellWowza)
    * Based on disucssion in the PR, this actually corrects a bug in the time formatting output
* Upgrade to using Rust 2018
* Bump Minimum Supported Rust Version (MSRV) to 1.53
    * This is required because of the `time` crate
* Switch from Travis to Github Actions
* Require `slog-json` version >= `2.6`

## 2.3.0 - 2021-01-10
### Changed

* Update dependencies

## 2.2.0 - 2020-01-19
## Changed

* Add with_name API for setting bunyan name field
* Replace level_to_string function with BunyanLevel enum
* Make slog-bunyan compatible with windows and x86_64-sun-solaris builds

## 2.1.0 - 2017-04-29
### Fixed

* Use `hostname` instead of `host` key

## 2.0.0 - 2017-04-29
### Changed

* Update dependencies


## 2.0.0-4.0 - 2017-04-11
### Changed

* Update dependencies

## 2.0.0-3.0 - 2017-04-07

### Changed

* Update dependencies

## 2.0.0-2.0 - 2017-03-20
### Changed

* Update slog v2 dependency

## 2.0.0-1.0 - 2017-03-05

* Move to slog v2

## 1.1.1 - 2016-11-30
### Changed

* Moved to own repository
