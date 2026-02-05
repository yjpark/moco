use facet::Facet;

#[derive(Facet)]
pub struct CellSpec {
    pub name: String,
    pub version: String,
    pub title: String,
    pub description: String,
}
