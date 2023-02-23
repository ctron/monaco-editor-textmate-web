use crate::{sys::create_tokens_provider, GrammarDefinition};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;

/// Set a textmate grammar on a `CodeEditor` component.
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use monaco::{api::*, sys::editor::BuiltinTheme, yew::{CodeEditor, CodeEditorLink}};
/// use monaco_editor_textmate_web::prelude::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///
///   /// The ID of the language in monaco
///   const LANGUAGE_ID: &str = "my-language-id";
///   /// The (root) scope ID of the textmate definition
///   const SCOPE_ID: &str = "source.my-language";
///   /// The actual textmate grammar
///   const TEXTMATE_GRAMMAR: &str = r#""#;
///   // Better import from a file, like this:
///   // const TEXTMATE_GRAMMAR: &str = include_str!("languages/my.tmLanguage.json");
///
///   // Register language ID
///   register_language(LANGUAGE_ID);
///
///   let model = use_memo(|()|{
///     TextModel::create(
///       "initial content",
///       Some(LANGUAGE_ID),
///       None,
///     )
///     .unwrap()
///   }, ());
///
///   let options = use_memo(|()|{
///     CodeEditorOptions::default()
///       .with_scroll_beyond_last_line(false)
///       .with_language(LANGUAGE_ID.to_string())
///       .with_builtin_theme(BuiltinTheme::Vs)
///       .with_automatic_layout(true)
///       .to_sys_options()
///   }, ());
///
///   let editor = use_memo(|(model, options)|{
///     let on_editor_created = Callback::from(|editor: CodeEditorLink| {
///       // on initialization, set the token provider
///       set_textmate_provider(
///         &editor,
///         GrammarDefinition::new("json", TEXTMATE_GRAMMAR),
///         LANGUAGE_ID,
///         SCOPE_ID,
///       );
///     });
///
///     html!(<CodeEditor
///       model={(**model).clone()}
///       options={(**options).clone()}
///       {on_editor_created}
///     />)
///   }, (model, options));
///
///   html!(
///     <div> { (*editor).clone() } </div>
///   )
/// }
/// ```
pub fn set_textmate_provider(
    editor: &monaco::yew::CodeEditorLink,
    grammar: GrammarDefinition,
    language_id: &'static str,
    initial_scope: &'static str,
) {
    if let Some(f) = editor.with_editor(|editor| {
        let editor: JsValue = editor.as_ref().into();
        async move { create_tokens_provider(&editor, grammar, initial_scope).await }
    }) {
        spawn_local(async move {
            if let Ok(provider) = f.await {
                monaco::sys::languages::set_tokens_provider(language_id, &provider);
            }
        })
    }
}
