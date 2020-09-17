extern crate pkg_config;
extern crate vcpkg;

fn main() {
    if cfg!(target_env = "msvc") {
        if try_vcpkg() {
            return;
        }
    }
    if try_pkg_config() {
        return;
    }
}

fn try_vcpkg() -> bool {
    vcpkg::find_package("libsndfile").is_ok()
}

fn try_pkg_config() -> bool {
    pkg_config::probe_library("sndfile").is_ok()
}
