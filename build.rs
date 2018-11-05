extern crate vcpkg;

fn main() {
    if cfg!(target_env = "msvc") {
        if try_vcpkg() {
            return;
        }
    }
}

fn try_vcpkg() -> bool {
    vcpkg::find_package("libsndfile").is_ok()
}
