// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use SelectionData;
use TreePath;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct TreeDragDest(Object<ffi::GtkTreeDragDest, ffi::GtkTreeDragDestIface>);

    match fn {
        get_type => || ffi::gtk_tree_drag_dest_get_type(),
    }
}

pub trait TreeDragDestExt: 'static {
    fn drag_data_received(&self, dest: &mut TreePath, selection_data: &mut SelectionData) -> bool;

    fn row_drop_possible(&self, dest_path: &mut TreePath, selection_data: &mut SelectionData) -> bool;
}

impl<O: IsA<TreeDragDest>> TreeDragDestExt for O {
    fn drag_data_received(&self, dest: &mut TreePath, selection_data: &mut SelectionData) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_drag_dest_drag_data_received(self.to_glib_none().0, dest.to_glib_none_mut().0, selection_data.to_glib_none_mut().0))
        }
    }

    fn row_drop_possible(&self, dest_path: &mut TreePath, selection_data: &mut SelectionData) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_drag_dest_row_drop_possible(self.to_glib_none().0, dest_path.to_glib_none_mut().0, selection_data.to_glib_none_mut().0))
        }
    }
}

impl fmt::Display for TreeDragDest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TreeDragDest")
    }
}
