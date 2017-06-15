use ::inflector::Inflector;
use ::quote::Tokens;
use ::syn::Ident;
use ::yaml_rust::Yaml;

use infer::datatype;
use infer::StructBoundArgImpl;

fn ident(param: &str) -> Ident {
    Ident::new(param.to_snake_case())
}

pub(super) fn infer_v3(method: &str, schema: &Yaml) -> StructBoundArgImpl {
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
        placeholders.push(format!("{{{}}}", name));

        if let Some(supporting_type) = maybe_at {
            supporting_types.push(supporting_type);
        }
    }

    let params = idents.clone();

    let endpoint_id_arg = match (method.as_ref(), idents.pop(), types.pop()) {
        ("delete", Some(endp_ident), Some(endp_type))
            // The resource ID value is moved here, to avoid its reuse
            // !FIXME: this assumes that the DELETE request succeeds
            => quote!(#endp_ident: #endp_type),

        (_, Some(endp_ident), Some(endp_type))
            // We take a reference to the ID, as for any others if nested
            => quote!(#endp_ident: &#endp_type),

        (_, None, _)
        | (_, _, None) => panic!("params::infer called without any params to infer"),
    };

    Ok((
        quote!{ #(#supporting_types)* },
        quote!(),
        quote!{ #(#idents: &#types,)* #endpoint_id_arg },
        quote! {
            .path_segments_mut().unwrap()
                .clear()
                .push(Url::parse(self::API_URL).unwrap().path())
                .extend(self::API_PATH.split('/').map(|p| match p {
                    #(#placeholders => #params.to_string(),)*
                    _ => p.to_string(),
                }))
        }
    ))
}
