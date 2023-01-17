// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// from webkit2gtk-gir-files
// DO NOT EDIT

use crate::PermissionRequest;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "WebKitInstallMissingMediaPluginsPermissionRequest")]
    pub struct InstallMissingMediaPluginsPermissionRequest(Object<ffi::WebKitInstallMissingMediaPluginsPermissionRequest, ffi::WebKitInstallMissingMediaPluginsPermissionRequestClass>) @implements PermissionRequest;

    match fn {
        type_ => || ffi::webkit_install_missing_media_plugins_permission_request_get_type(),
    }
}

impl InstallMissingMediaPluginsPermissionRequest {
    pub const NONE: Option<&'static InstallMissingMediaPluginsPermissionRequest> = None;
}

pub trait InstallMissingMediaPluginsPermissionRequestExt: 'static {
    #[doc(alias = "webkit_install_missing_media_plugins_permission_request_get_description")]
    #[doc(alias = "get_description")]
    fn description(&self) -> Option<glib::GString>;
}

impl<O: IsA<InstallMissingMediaPluginsPermissionRequest>>
    InstallMissingMediaPluginsPermissionRequestExt for O
{
    fn description(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(
                ffi::webkit_install_missing_media_plugins_permission_request_get_description(
                    self.as_ref().to_glib_none().0,
                ),
            )
        }
    }
}

impl fmt::Display for InstallMissingMediaPluginsPermissionRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("InstallMissingMediaPluginsPermissionRequest")
    }
}
