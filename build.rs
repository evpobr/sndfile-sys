extern crate pkg_config;

fn main() {
    if pkg_config::Config::new().atleast_version("1.0").probe("sndfile").is_ok() {
        return;
    }
}
