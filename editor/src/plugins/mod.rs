mod paste_converter;
pub use paste_converter::PasteConverter;

use web_sys::Element;

// use web_sys::{Element, MouseEvent, window, Document};
// use wasm_bindgen::UnwrapThrowExt;

pub trait EditorPlugin {
    // fn remove_menu<'a>(curr: &'a Element, doc: &'a Document);
    // fn make_menu<'a>( curr: &'a Element, doc: &'a Document);
}

pub trait PluginTraits {}

impl<T> EditorPlugin for T
where
    T: PluginTraits,
{
    // fn remove_menu<'a>(curr: &'a Element, doc: &'a Document) {}
    // fn make_menu<'a>( curr: &'a Element, doc: &'a Document) {
    // }
}
