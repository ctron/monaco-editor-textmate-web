//! WASM bindings for monaco-editor-textmate

use wasm_bindgen::prelude::*;

#[cfg_attr(debug_assertions, wasm_bindgen(module = "/js/debug/sys.js"))]
#[cfg_attr(not(debug_assertions), wasm_bindgen(module = "/js/release/sys.js"))]
extern "C" {

    pub type GrammarDefinition;

    /// Create a new grammar definition
    ///
    /// * `format` of the content, is either `json` or `plist`.
    /// * `content`, is the actual content of the definition, the "tmLanguage" file.
    #[wasm_bindgen(constructor)]
    pub fn new(format: &str, content: &str) -> GrammarDefinition;

    #[wasm_bindgen(catch, js_name= createTokensProvider)]
    pub async fn create_tokens_provider(
        editor: &JsValue,
        grammar: GrammarDefinition,
        initial_scope: &str,
    ) -> Result<JsValue, JsValue>;

}
