use derbe_shared::dto::response::location::LocationResponse;

pub struct Location {
    pub id: i32,

    pub name: String,
}

impl From<LocationResponse> for Location {
    fn from(value: LocationResponse) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

impl From<Location> for LocationResponse {
    fn from(value: Location) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}
