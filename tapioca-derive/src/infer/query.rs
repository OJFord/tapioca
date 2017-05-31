use ::inflector::Inflector;
use ::quote::Tokens;
use ::syn::Ident;
use ::yaml_rust::Yaml;

use infer::datatype;
use infer::Error;

type StructBoundArgImpl = Result<
    (Tokens, Tokens, Tokens, Tokens),
    Box<Error + Send + Sync>
>;

fn ident(param: &str) -> Ident {
    Ident::new(param.to_snake_case())
}

pub(super) fn infer_v3(struct_ident: &Ident, schema: &Yaml) -> StructBoundArgImpl {
    let mut idents: Vec<Ident> = Vec::new();
    let mut types: Vec<Tokens> = Vec::new();
    let mut name_strs: Vec<Tokens> = Vec::new();
    let mut accessors: Vec<Tokens> = Vec::new();

    for param_schema in schema.as_vec().unwrap() {
        let name = param_schema["name"].as_str()
            .expect("Parameter name must be a string.");
        let field = ident(name);

        idents.push(ident(name));
        types.push(datatype::infer_v3(&param_schema["schema"])?.0);
        name_strs.push(quote!{ #name });
        accessors.push(quote!{ query_parameters.#field });
    }

    Ok((
        quote! {
            pub struct #struct_ident {
                #(pub #idents: #types),*
            }
        },
        quote! {},
        quote! { query_parameters: &#struct_ident },
        quote! {
            url.query_pairs_mut()
                .clear()
                #(.append_pair(
                    #name_strs,
                    #accessors.to_string().as_str()
                ))*
                ;
        }
    ))
}
