use crate::features::divisions::dto::response::DivisionResponse;

pub struct Division {
    pub id: i32,
    pub name: String,
}

impl From<Division> for DivisionResponse {
    fn from(value: Division) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}
