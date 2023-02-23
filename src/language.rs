use monaco::sys::languages::ILanguageExtensionPoint;
use wasm_bindgen::JsCast;

/// Registers a new language just using an id.
pub fn register_language(id: &str) {
    register_language_with(|language| {
        language.set_id(id);
    })
}

/// Register a new language using a customizer function.
///
/// **NOTE:** You need to call at least [`ILanguageExtensionPoint::set_id`].
pub fn register_language_with<F>(f: F)
where
    F: FnOnce(&mut ILanguageExtensionPoint),
{
    let mut language: ILanguageExtensionPoint = js_sys::Object::new().unchecked_into();
    f(&mut language);
    monaco::sys::languages::register(&language);
}
