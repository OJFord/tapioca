use std::error::Error;
use ::syn::Ident;
use ::quote::Tokens;
use ::yaml_rust::Yaml;

pub(super) fn infer_schema(name: &Ident, schema: &Yaml) -> Result<Tokens, Box<Error>> {
    Ok(quote! {
        extern crate tapioca;
        use tapioca::Schema;

        struct Resource;

        impl #name {
            fn resource() -> Resource {
                Resource
            }
        }

        impl Schema for #name {
            fn get(&self) {
                //!TODO: Replace for compile_error! macro
                //! when rust-lang/rust#40872 implemented
                panic!("GET not allowed for /")
            }
        }

        impl Schema for Resource {
            fn get(&self) {
                panic!("Not implemented!")
            }
        }
    })
}
