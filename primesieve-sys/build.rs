extern crate pkg_config;

fn main() {
    pkg_config::Config::new()
        .atleast_version("8.0")
        .probe("primesieve")
        .unwrap();
}
