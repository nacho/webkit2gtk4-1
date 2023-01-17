// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::WebView;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitURISchemeRequest")]
    pub struct URISchemeRequest(Object<ffi::WebKitURISchemeRequest, ffi::WebKitURISchemeRequestClass>);

    match fn {
        type_ => || ffi::webkit_uri_scheme_request_get_type(),
    }
}

impl URISchemeRequest {
    pub const NONE: Option<&'static URISchemeRequest> = None;
}

pub trait URISchemeRequestExt: 'static {
    #[doc(alias = "webkit_uri_scheme_request_finish")]
    fn finish(
        &self,
        stream: &impl IsA<gio::InputStream>,
        stream_length: i64,
        content_type: Option<&str>,
    );

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    #[doc(alias = "webkit_uri_scheme_request_finish_error")]
    fn finish_error(&self, error: &mut glib::Error);

    //#[cfg(any(feature = "v2_36", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_36")))]
    //#[doc(alias = "webkit_uri_scheme_request_finish_with_response")]
    //fn finish_with_response(&self, response: /*Ignored*/&URISchemeResponse);

    //#[cfg(any(feature = "v2_36", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_36")))]
    //#[doc(alias = "webkit_uri_scheme_request_get_http_headers")]
    //#[doc(alias = "get_http_headers")]
    //fn http_headers(&self) -> /*Ignored*/Option<soup::MessageHeaders>;

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_36")))]
    #[doc(alias = "webkit_uri_scheme_request_get_http_method")]
    #[doc(alias = "get_http_method")]
    fn http_method(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_uri_scheme_request_get_path")]
    #[doc(alias = "get_path")]
    fn path(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_uri_scheme_request_get_scheme")]
    #[doc(alias = "get_scheme")]
    fn scheme(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_uri_scheme_request_get_uri")]
    #[doc(alias = "get_uri")]
    fn uri(&self) -> Option<glib::GString>;

    #[doc(alias = "webkit_uri_scheme_request_get_web_view")]
    #[doc(alias = "get_web_view")]
    fn web_view(&self) -> Option<WebView>;
}

impl<O: IsA<URISchemeRequest>> URISchemeRequestExt for O {
    fn finish(
        &self,
        stream: &impl IsA<gio::InputStream>,
        stream_length: i64,
        content_type: Option<&str>,
    ) {
        unsafe {
            ffi::webkit_uri_scheme_request_finish(
                self.as_ref().to_glib_none().0,
                stream.as_ref().to_glib_none().0,
                stream_length,
                content_type.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_2")))]
    fn finish_error(&self, error: &mut glib::Error) {
        unsafe {
            ffi::webkit_uri_scheme_request_finish_error(
                self.as_ref().to_glib_none().0,
                error.to_glib_none_mut().0,
            );
        }
    }

    //#[cfg(any(feature = "v2_36", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_36")))]
    //fn finish_with_response(&self, response: /*Ignored*/&URISchemeResponse) {
    //    unsafe { TODO: call ffi:webkit_uri_scheme_request_finish_with_response() }
    //}

    //#[cfg(any(feature = "v2_36", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_36")))]
    //fn http_headers(&self) -> /*Ignored*/Option<soup::MessageHeaders> {
    //    unsafe { TODO: call ffi:webkit_uri_scheme_request_get_http_headers() }
    //}

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_36")))]
    fn http_method(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_uri_scheme_request_get_http_method(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn path(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_uri_scheme_request_get_path(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn scheme(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_uri_scheme_request_get_scheme(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn uri(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_uri_scheme_request_get_uri(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn web_view(&self) -> Option<WebView> {
        unsafe {
            from_glib_none(ffi::webkit_uri_scheme_request_get_web_view(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for URISchemeRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("URISchemeRequest")
    }
}
