use ::inflector::Inflector;
use ::quote::Tokens;
use ::syn::Ident;
use ::yaml_rust::Yaml;

use infer::datatype;
use infer::StructBoundArgImpl;

fn ident(param: &str) -> Ident {
    Ident::new(param.to_snake_case())
}

pub(super) fn infer_v3(structs_mod: &Ident, schema: &Yaml) -> StructBoundArgImpl {
    let mut idents: Vec<Ident> = Vec::new();
    let mut types: Vec<Tokens> = Vec::new();
    let mut name_strs: Vec<Tokens> = Vec::new();

    for schema in schema.as_vec().unwrap() {
        let name = schema["name"].as_str()
            .expect("Parameter name must be a string.");
        let (param_type, _) = datatype::infer_v3(&schema["schema"])?;

        let mandate: Tokens;
        if let Some(true) = schema["required"].as_bool() {
            mandate = quote!(::tapioca::datatype::Required);
        } else {
            mandate = quote!(::tapioca::datatype::Optional);
        }

        idents.push(ident(name));
        types.push(quote!{ #mandate<#param_type> });
        name_strs.push(quote!{ #name });
    }

    let idents2 = idents.clone();
    Ok((
        Some(quote! {
            use ::tapioca::query::QueryPair;
            use ::tapioca::query::QueryParameter;
            use ::tapioca::query::QueryString;

            #[derive(Debug)]
            pub struct QueryParams {
                #(pub #idents: #types),*
            }

            impl QueryString for QueryParams {
                fn as_query_kv(&self) -> Vec<QueryPair> {
                    let mut params: Vec<QueryPair> = Vec::new();
                    #(params.append(&mut self.#idents2.as_query_kv(#name_strs));)*
                    params
                }
            }
        }),
        None,
        Some(quote! { query_parameters: #structs_mod::QueryParams }),
        Some(quote! {
            .query_pairs_mut()
                .extend_pairs(query_parameters.as_query_kv())
                .finish()
        })
    ))
}
