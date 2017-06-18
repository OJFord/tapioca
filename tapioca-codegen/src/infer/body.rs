use ::syn::Ident;
use ::yaml_rust::Yaml;

use infer::datatype;
use infer::StructBoundArgImpl;

fn infer_v3_json(structs_mod: &Ident, schema: &Yaml) -> StructBoundArgImpl {
    let (inferred_type, aux_types) = datatype::infer_v3(&schema)?;
    Ok((
        Some(quote! {
            #(#aux_types)*
            pub type RequestBody = #inferred_type;
        }),
        None,
        Some(quote!{ body: &#structs_mod::RequestBody }),
        Some(quote!{
            .json(body)
            .header(header::ContentLength(
                ::tapioca::serde_json::to_vec(body).unwrap().len() as u64
            ))
        })
    ))
}

pub(super) fn infer_v3(structs_mod: &Ident, schema: &Yaml) -> StructBoundArgImpl {
    match schema["content"]["application/json"]["schema"] {
        Yaml::BadValue => Err(From::from("only JSON bodies supported at this time")),
        ref schema => infer_v3_json(structs_mod, &schema),
    }
}
