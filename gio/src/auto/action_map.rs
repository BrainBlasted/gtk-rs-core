// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Action;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GActionMap")]
    pub struct ActionMap(Interface<ffi::GActionMap, ffi::GActionMapInterface>);

    match fn {
        type_ => || ffi::g_action_map_get_type(),
    }
}

pub const NONE_ACTION_MAP: Option<&ActionMap> = None;

pub trait ActionMapExt: 'static {
    #[doc(alias = "g_action_map_add_action")]
    fn add_action(&self, action: &impl IsA<Action>);

    //#[doc(alias = "g_action_map_add_action_entries")]
    //fn add_action_entries(&self, entries: /*Ignored*/&[&ActionEntry], user_data: /*Unimplemented*/Option<Fundamental: Pointer>);

    #[doc(alias = "g_action_map_lookup_action")]
    fn lookup_action(&self, action_name: &str) -> Option<Action>;

    #[doc(alias = "g_action_map_remove_action")]
    fn remove_action(&self, action_name: &str);
}

impl<O: IsA<ActionMap>> ActionMapExt for O {
    fn add_action(&self, action: &impl IsA<Action>) {
        unsafe {
            ffi::g_action_map_add_action(
                self.as_ref().to_glib_none().0,
                action.as_ref().to_glib_none().0,
            );
        }
    }

    //fn add_action_entries(&self, entries: /*Ignored*/&[&ActionEntry], user_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi:g_action_map_add_action_entries() }
    //}

    fn lookup_action(&self, action_name: &str) -> Option<Action> {
        unsafe {
            from_glib_none(ffi::g_action_map_lookup_action(
                self.as_ref().to_glib_none().0,
                action_name.to_glib_none().0,
            ))
        }
    }

    fn remove_action(&self, action_name: &str) {
        unsafe {
            ffi::g_action_map_remove_action(
                self.as_ref().to_glib_none().0,
                action_name.to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for ActionMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ActionMap")
    }
}
