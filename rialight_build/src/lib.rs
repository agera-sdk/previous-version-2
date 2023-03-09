pub use rialight_app_config::ApplicationConfig;
use std::fs;

#[macro_export]
macro_rules! initialize {
    ($lambda_exp:expr) => {
        fn main() {
            ::rialight_build::_build_preprocess(std::env::var("OUT_DIR").unwrap().as_ref());
            let user_ie: fn() -> () = $lambda_exp;
            user_ie();
        }
    };
}

#[doc(hidden)]
pub fn _build_preprocess(out_dir: &str) {
    // let _app_config = ApplicationConfig::read("/").unwrap();

    // rialight_entry.rs
    let rialight_entry_contents = "\
{
    #[cfg(debug_assertions)]
    unsafe {
        rialight::filesystem::__APP_DIRECTORY = Some(String::from(std::env::current_dir().unwrap().to_str().unwrap()));
        rialight::filesystem::__APP_STORAGE_DIRECTORY = Some(String::from(concat!(env!(\"OUT_DIR\"), \"/rialight.debug.app_storage_dir\")));
    }
    #[cfg(not(debug_assertions))]
    unsafe {
    }
}
";

    fs::write(out_dir.to_owned() + "/rialight_entry.rs", rialight_entry_contents).unwrap();

    // rialight_debug_app_storage_dir
    drop(fs::remove_dir_all(out_dir.to_owned() + "/rialight.debug.app_storage_dir"));
    fs::create_dir_all(out_dir.to_owned() + "/rialight.debug.app_storage_dir").unwrap();
}

// pub use initialize_build;