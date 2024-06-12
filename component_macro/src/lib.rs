extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(ComponentBundle)]
pub fn component_bundle_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => fields_named.named,
            _ => panic!("ComponentBundle can only be derived for structs with named fields"),
        },
        _ => panic!("ComponentBundle can only be derived for structs"),
    };

    let field_names = fields.iter().map(|f| &f.ident);

    let gen = quote! {
        impl Bundle for #name {
            type Components = std::vec::IntoIter<Box<dyn Component>>;

            fn components(&self) -> Self::Components {
                let mut components: Vec<Box<dyn Component>> = Vec::new();
                #(
                    components.push(Box::new(self.#field_names.clone()) as Box<dyn Component>);
                )*
                components.into_iter()
            }
        }
    };
    gen.into()
}
