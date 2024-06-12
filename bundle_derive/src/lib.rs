extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Bundle)]
pub fn derive_bundle(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let register_impl = match input.data {
        Data::Struct(ref data) => {
            let component_registers = data.fields.iter().map(|f| {
                let ty = &f.ty;
                quote! {
                    store.try_register::<#ty>();
                }
            });
            quote! {
                fn register(store: &mut ComponentStorage) {
                    #(#component_registers)*
                }
            }
        }
        _ => unimplemented!(),
    };

    let add_data_impl = match input.data {
        Data::Struct(ref data) => {
            let inserts = data.fields.iter().map(|f| {
                let name = &f.ident;
                let ty = &f.ty;
                quote! {
                    store.get_mut::<#ty>().unwrap().insert(entity, self.#name);
                }
            });
            quote! {
                fn add_data(self, store: &mut ComponentStorage, bundles: &Bundles) -> Option<Entity> {
                    if bundles.get(&TypeId::of::<Self>()).is_none() {
                        return None;
                    }

                    let entity = store.create();
                    #(#inserts)*
                    Some(entity)
                }
            }
        }
        _ => unimplemented!(),
    };

    let expanded = quote! {
        impl Bundle for #name {
            #register_impl
            #add_data_impl
        }
    };

    TokenStream::from(expanded)
}
