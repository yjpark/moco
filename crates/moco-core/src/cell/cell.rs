use super::spec::Spec;

pub trait Cell {
    const SPEC: &'static Spec;
}
