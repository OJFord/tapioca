#![feature(plugin_registrar)]
#![feature(rustc_private)]

extern crate rustc_plugin;
extern crate syntax;

pub mod traits;

use rustc_plugin::Registry;
use syntax::feature_gate::AttributeType;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_attribute("SchemaURL".to_owned(), AttributeType::Whitelisted);
}
