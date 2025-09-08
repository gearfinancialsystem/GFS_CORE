use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Parse le code d'entrée en une structure manipulable
    let ast = parse_macro_input!(input as DeriveInput);

    // Récupère le nom du type (struct, enum, etc.)
    let name = &ast.ident;

    // Génère le code de l'implémentation
    let expanded = quote! {
        impl #name {
            pub fn hello_macro() {
                println!("Bonjour, je suis une macro procédurale pour {}!", stringify!(#name));
            }
        }
    };

    // Retourne le code généré
    TokenStream::from(expanded)
}
