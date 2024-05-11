use crate::{model::division::Division, repository::division_repository::DivisionRepository};

use super::DerbeService;

pub struct DivisionService {}

impl DerbeService<Division, DivisionRepository> for DivisionService {}
