fn main() {
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV");
    let delay_load = if target_env.as_deref() == Ok("gnu") {
        "-Wl,--delayload="
    } else {
        "/DELAYLOAD:"
    };
    println!("cargo:rustc-link-arg-cdylib={delay_load}victory-probe-delayed.dll");
    println!("cargo:rustc-link-lib=delayimp");
}
