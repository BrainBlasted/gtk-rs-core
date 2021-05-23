// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AsyncResult;
use crate::Cancellable;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GInputStream")]
    pub struct InputStream(Object<ffi::GInputStream, ffi::GInputStreamClass>);

    match fn {
        type_ => || ffi::g_input_stream_get_type(),
    }
}

pub const NONE_INPUT_STREAM: Option<&InputStream> = None;

pub trait InputStreamExt: 'static {
    #[doc(alias = "g_input_stream_clear_pending")]
    fn clear_pending(&self);

    #[doc(alias = "g_input_stream_close")]
    fn close(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<(), glib::Error>;

    #[doc(alias = "g_input_stream_close_async")]
    fn close_async<P: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    fn close_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "g_input_stream_has_pending")]
    fn has_pending(&self) -> bool;

    #[doc(alias = "g_input_stream_is_closed")]
    fn is_closed(&self) -> bool;

    #[doc(alias = "g_input_stream_read_bytes")]
    fn read_bytes(
        &self,
        count: usize,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<glib::Bytes, glib::Error>;

    #[doc(alias = "g_input_stream_read_bytes_async")]
    fn read_bytes_async<P: FnOnce(Result<glib::Bytes, glib::Error>) + Send + 'static>(
        &self,
        count: usize,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    fn read_bytes_async_future(
        &self,
        count: usize,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<glib::Bytes, glib::Error>> + 'static>>;

    #[doc(alias = "g_input_stream_set_pending")]
    fn set_pending(&self) -> Result<(), glib::Error>;

    #[doc(alias = "g_input_stream_skip")]
    fn skip(
        &self,
        count: usize,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<isize, glib::Error>;

    #[doc(alias = "g_input_stream_skip_async")]
    fn skip_async<P: FnOnce(Result<isize, glib::Error>) + Send + 'static>(
        &self,
        count: usize,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    );

    fn skip_async_future(
        &self,
        count: usize,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<isize, glib::Error>> + 'static>>;
}

impl<O: IsA<InputStream>> InputStreamExt for O {
    fn clear_pending(&self) {
        unsafe {
            ffi::g_input_stream_clear_pending(self.as_ref().to_glib_none().0);
        }
    }

    fn close(&self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_input_stream_close(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn close_async<P: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let user_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn close_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_input_stream_close_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<P> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = close_async_trampoline::<P>;
        unsafe {
            ffi::g_input_stream_close_async(
                self.as_ref().to_glib_none().0,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn close_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.close_async(io_priority, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    fn has_pending(&self) -> bool {
        unsafe {
            from_glib(ffi::g_input_stream_has_pending(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_closed(&self) -> bool {
        unsafe {
            from_glib(ffi::g_input_stream_is_closed(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn read_bytes(
        &self,
        count: usize,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<glib::Bytes, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_input_stream_read_bytes(
                self.as_ref().to_glib_none().0,
                count,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_bytes_async<P: FnOnce(Result<glib::Bytes, glib::Error>) + Send + 'static>(
        &self,
        count: usize,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let user_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn read_bytes_async_trampoline<
            P: FnOnce(Result<glib::Bytes, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_input_stream_read_bytes_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<P> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = read_bytes_async_trampoline::<P>;
        unsafe {
            ffi::g_input_stream_read_bytes_async(
                self.as_ref().to_glib_none().0,
                count,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn read_bytes_async_future(
        &self,
        count: usize,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<glib::Bytes, glib::Error>> + 'static>>
    {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.read_bytes_async(count, io_priority, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    fn set_pending(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_input_stream_set_pending(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn skip(
        &self,
        count: usize,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<isize, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_input_stream_skip(
                self.as_ref().to_glib_none().0,
                count,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn skip_async<P: FnOnce(Result<isize, glib::Error>) + Send + 'static>(
        &self,
        count: usize,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let user_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn skip_async_trampoline<
            P: FnOnce(Result<isize, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_input_stream_skip_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<P> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = skip_async_trampoline::<P>;
        unsafe {
            ffi::g_input_stream_skip_async(
                self.as_ref().to_glib_none().0,
                count,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn skip_async_future(
        &self,
        count: usize,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<isize, glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.skip_async(count, io_priority, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }
}

impl fmt::Display for InputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("InputStream")
    }
}
