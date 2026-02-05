use facet::Facet;

#[derive(Facet)]
pub struct FuncSpec {
    pub name: String,
    pub title: String,
    pub description: String,
}
