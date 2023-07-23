pub mod app;
pub use rialight_filesystem as filesystem;
pub use rialight_intl as intl;
pub use rialight_temporal as temporal;
pub use rialight_util as util;

#[doc(hidden)]
pub use tokio as __tokio;

#[macro_export]
macro_rules! initialize {
    ($lambda_exp:expr) => {
        use rialight::__tokio as __rialight_tokio;

        #[__rialight_tokio::main(crate = "__rialight_tokio")]
        async fn main() {
            use std::sync::{Arc};
            use rialight::app::Application;
            include!(concat!(env!("OUT_DIR"), "/rialight_entry.rs"));
            let app = Arc::new(Application {});
            let user_ie: fn(Arc<Application>) -> _ = $lambda_exp;
            user_ie(app.clone()).await;
        }
    };
}