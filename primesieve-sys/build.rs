extern crate pkg_config;

fn main() {
    pkg_config::Config::new()
        .atleast_version("7.7")
        .probe("primesieve")
        .unwrap();
}
