use facet::Facet;

use crate::FuncSpec;

#[derive(Facet)]
pub struct CellSpec {
    pub name: String,
    pub version: String,
    pub title: String,
    pub description: String,
    pub functions: Vec<FuncSpec>,
}
