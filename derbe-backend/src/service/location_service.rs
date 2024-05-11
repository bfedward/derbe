use crate::{model::location::Location, repository::location_repository::LocationRepository};

use super::DerbeService;

pub struct LocationService {}

impl DerbeService<Location, LocationRepository> for LocationService {}
