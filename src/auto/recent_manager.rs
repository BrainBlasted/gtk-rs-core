// This file was generated by gir (a4dcfea) from gir-files (0bcaef9)
// DO NOT EDIT

use Error;
use RecentData;
use RecentInfo;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct RecentManager(Object<ffi::GtkRecentManager>);

    match fn {
        get_type => || ffi::gtk_recent_manager_get_type(),
    }
}

impl RecentManager {
    pub fn new() -> RecentManager {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_recent_manager_new())
        }
    }

    pub fn get_default() -> Option<RecentManager> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_recent_manager_get_default())
        }
    }
}

pub trait RecentManagerExt {
    fn add_full(&self, uri: &str, recent_data: &RecentData) -> bool;

    fn add_item(&self, uri: &str) -> bool;

    fn get_items(&self) -> Vec<RecentInfo>;

    fn has_item(&self, uri: &str) -> bool;

    fn lookup_item(&self, uri: &str) -> Result<Option<RecentInfo>, Error>;

    fn move_item<'a, P: Into<Option<&'a str>>>(&self, uri: &str, new_uri: P) -> Result<(), Error>;

    fn purge_items(&self) -> Result<i32, Error>;

    fn remove_item(&self, uri: &str) -> Result<(), Error>;

    fn get_property_filename(&self) -> Option<String>;

    fn get_property_size(&self) -> i32;

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<RecentManager> + IsA<glib::object::Object>> RecentManagerExt for O {
    fn add_full(&self, uri: &str, recent_data: &RecentData) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_manager_add_full(self.to_glib_none().0, uri.to_glib_none().0, recent_data.to_glib_none().0))
        }
    }

    fn add_item(&self, uri: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_manager_add_item(self.to_glib_none().0, uri.to_glib_none().0))
        }
    }

    fn get_items(&self) -> Vec<RecentInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_recent_manager_get_items(self.to_glib_none().0))
        }
    }

    fn has_item(&self, uri: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_manager_has_item(self.to_glib_none().0, uri.to_glib_none().0))
        }
    }

    fn lookup_item(&self, uri: &str) -> Result<Option<RecentInfo>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_recent_manager_lookup_item(self.to_glib_none().0, uri.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn move_item<'a, P: Into<Option<&'a str>>>(&self, uri: &str, new_uri: P) -> Result<(), Error> {
        let new_uri = new_uri.into();
        let new_uri = new_uri.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_recent_manager_move_item(self.to_glib_none().0, uri.to_glib_none().0, new_uri.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn purge_items(&self) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_recent_manager_purge_items(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn remove_item(&self, uri: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gtk_recent_manager_remove_item(self.to_glib_none().0, uri.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_property_filename(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "filename".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn get_property_size(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "size".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn changed_trampoline<P>(this: *mut ffi::GtkRecentManager, f: glib_ffi::gpointer)
where P: IsA<RecentManager> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&RecentManager::from_glib_none(this).downcast_unchecked())
}
