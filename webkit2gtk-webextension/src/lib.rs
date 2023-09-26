// Take a look at the license at the top of the repository in the LICENSE file.
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use ffi;
pub use gio;
pub use glib;
pub use gtk;
pub use javascriptcore;

macro_rules! assert_initialized_main_thread {
    () => {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[macro_export]
macro_rules! web_extension_init {
    () => {
        #[no_mangle]
        #[doc(hidden)]
        pub unsafe fn webkit_web_extension_initialize(
            extension: *mut $crate::ffi::WebKitWebExtension,
        ) {
            let extension: $crate::WebExtension =
                ::glib::translate::FromGlibPtrNone::from_glib_none(extension);
            web_extension_initialize(&extension);
        }
    };
}

#[macro_export]
macro_rules! web_extension_init_with_data {
    () => {
        #[no_mangle]
        #[doc(hidden)]
        pub unsafe fn webkit_web_extension_initialize_with_user_data(
            extension: *mut $crate::ffi::WebKitWebExtension,
            user_data: *mut $crate::glib_sys::GVariant,
        ) {
            let extension: $crate::WebExtension =
                ::glib::translate::FromGlibPtrNone::from_glib_none(extension);
            let user_data: Option<::glib::variant::Variant> =
                ::glib::translate::FromGlibPtrNone::from_glib_none(user_data);
            web_extension_initialize(&extension, user_data.as_ref());
        }
    };
}
#[allow(unused_imports)]
#[allow(non_snake_case)]
#[allow(clippy::let_unit_value)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::wrong_self_convention)]
mod auto;
mod dom_html_field_set_element;

pub use auto::*;

pub mod prelude {
    pub use super::auto::traits::*;
    pub use super::dom_html_field_set_element::DOMHTMLFieldSetElementExtManual;
    pub use javascriptcore::prelude::*;
}

unsafe impl Send for WebExtension {}
