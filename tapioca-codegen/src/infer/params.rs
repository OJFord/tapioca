use ::inflector::Inflector;
use ::quote::Tokens;
use ::syn::Ident;
use ::yaml_rust::Yaml;

use infer::datatype;
use infer::StructBoundArgImpl;

fn ident(param: &str) -> Ident {
    Ident::new(param.to_snake_case())
}

pub(super) fn infer_v3(_: &Ident, schema: &Yaml) -> StructBoundArgImpl {
    let mut idents: Vec<Ident> = Vec::new();
    let mut types: Vec<Tokens> = Vec::new();
    let mut supporting_types: Vec<Tokens> = Vec::new();
    let mut placeholders: Vec<String> = Vec::new();

    for schema in schema.as_vec().unwrap() {
        let name = schema["name"].as_str()
            .expect("Parameter name must be a string.");
        let (param_type, maybe_at) = datatype::infer_v3(&schema["schema"])?;

        idents.push(ident(name));
        types.push(quote!{ ::tapioca::datatype::Required<#param_type> });
        placeholders.push(name.into());

        if let Some(supporting_type) = maybe_at {
            supporting_types.push(supporting_type);
        }
    }

    let params = idents.clone();
    Ok((
        quote!{ #(#supporting_types)* },
        quote!(),
        quote!{ #(#idents: #types),* },
        quote! {
            let parts = self::API_PATH.split('/')
                .map(|p| match p { #(#placeholders => #params.to_string(),)* _ => p.into() });
            url.path_segments_mut().unwrap()
                .extend(parts);
        }
    ))
}
