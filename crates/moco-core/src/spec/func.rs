use facet::Facet;
use facet_styx::Schema;

#[derive(Facet)]
pub struct FuncSpec {
    pub name: String,
    pub title: String,
    pub description: String,
    pub input_schema: Schema,
    pub output_schema: Schema,
}
