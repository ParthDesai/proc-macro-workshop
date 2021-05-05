use proc_macro::TokenStream;
use syn::{DeriveInput, Data};
use quote::{quote, format_ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let derived_input: DeriveInput = syn::parse(input).unwrap();
    let ident = derived_input.ident;
    let generics = derived_input.generics;
    let visibility = derived_input.vis;
    let builder_name =  format_ident!("{}Builder", ident);

    let mut builder_fields = vec![];
    let mut default_builder_init = vec![];
    let mut builder_methods = vec![];
    let mut build_method_checks = vec![];
    let mut ident_creation_fragments = vec![];

    match derived_input.data {
        Data::Struct(s) => {
            for field in s.fields {
                let ident = field.ident.unwrap();
                let vis = field.vis;
                let ty = field.ty;

                builder_fields.push(quote!{
                   #vis #ident: Option<#ty>
                });

                default_builder_init.push(quote!{
                   #ident: None
                });

                builder_methods.push(quote!{
                    pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                        self.#ident = Some(#ident);
                        self
                    }
                });

                build_method_checks.push(quote!{
                    let #ident: #ty;
                    if !self.#ident.is_none() {
                        #ident = self.#ident.clone().unwrap();
                    } else {
                        return Err(Box::new(DummyError::new(ErrorKind::Other, "oh no!")));
                    }
                });

                ident_creation_fragments.push(quote!{
                   #ident: #ident
                });
            }
        }
        Data::Enum(_) => {
            // Return an error
        }
        Data::Union(_) => {
            // Return an error
        }
    };

    let tokens = quote!{
        use std::error::Error;
        use std::io::{Error as DummyError, ErrorKind};

        #visibility struct #builder_name#generics {
            #(#builder_fields),*
        }

        impl#generics #builder_name#generics {
            #(#builder_methods)*

            pub fn build(&mut self) -> Result<#ident, Box<dyn Error>> {
                #(#build_method_checks)*
                Ok(#ident {
                    #(#ident_creation_fragments),*
                })
            }
        }

        impl#generics #ident#generics {
            pub fn builder() -> #builder_name#generics {
                #builder_name {
                    #(#default_builder_init),*
                }
            }
        }
    };

    eprintln!("{}", tokens.to_string());

    tokens.into()
}
