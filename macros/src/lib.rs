use proc_macro::TokenStream;

/// Mark a function as a Schematic Supertest while preserving the item.
#[proc_macro_attribute]
pub fn supertest(attribute: TokenStream, item: TokenStream) -> TokenStream {
    if !attribute.is_empty() {
        return "compile_error!(\"#[supertest] does not accept arguments\");"
            .parse()
            .expect("static compile_error token stream must parse");
    }
    item
}
