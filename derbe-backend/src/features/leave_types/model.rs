use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// How a leave type appears in the "others' days" view.
/// `ShownByDefault` implies shown, so the invalid "default but hidden" state cannot exist.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, TS)]
#[sqlx(type_name = "valid_others_days_visibilities")]
#[ts(export)]
pub enum OthersDaysVisibility {
    #[default]
    Hidden,
    Shown,
    ShownByDefault,
}
