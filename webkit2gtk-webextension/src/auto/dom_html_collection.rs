// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/vhdirk/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::DOMNode;
use crate::DOMObject;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "WebKitDOMHTMLCollection")]
    pub struct DOMHTMLCollection(Object<ffi::WebKitDOMHTMLCollection, ffi::WebKitDOMHTMLCollectionClass>) @extends DOMObject;

    match fn {
        type_ => || ffi::webkit_dom_html_collection_get_type(),
    }
}

pub const NONE_DOMHTML_COLLECTION: Option<&DOMHTMLCollection> = None;

pub trait DOMHTMLCollectionExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_collection_get_length")]
    #[doc(alias = "get_length")]
    fn length(&self) -> libc::c_ulong;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_collection_item")]
    fn item(&self, index: libc::c_ulong) -> Option<DOMNode>;

    #[cfg_attr(feature = "v2_22", deprecated = "Since 2.22")]
    #[doc(alias = "webkit_dom_html_collection_named_item")]
    fn named_item(&self, name: &str) -> Option<DOMNode>;

    #[doc(alias = "length")]
    fn connect_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLCollection>> DOMHTMLCollectionExt for O {
    fn length(&self) -> libc::c_ulong {
        unsafe { ffi::webkit_dom_html_collection_get_length(self.as_ref().to_glib_none().0) }
    }

    fn item(&self, index: libc::c_ulong) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_collection_item(
                self.as_ref().to_glib_none().0,
                index,
            ))
        }
    }

    fn named_item(&self, name: &str) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_collection_named_item(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn connect_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_length_trampoline<
            P: IsA<DOMHTMLCollection>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WebKitDOMHTMLCollection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DOMHTMLCollection::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::length\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_length_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DOMHTMLCollection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DOMHTMLCollection")
    }
}