extern crate vcpkg;

fn main() {
    if cfg!(target_env = "msvc") {
        if try_vcpkg() {
            return;
        }
    }
}

fn try_vcpkg() -> bool {
    match vcpkg::find_package("libsndfile") {
        Ok(_) => true,
        _ => false
    }
}
