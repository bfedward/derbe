use serde::{de, Deserialize, Deserializer};

pub fn deserialize_non_negative_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = f64::deserialize(deserializer)?;
    if value < 0.0 {
        Err(de::Error::custom("value must be non-negative"))
    } else {
        Ok(value)
    }
}

pub fn deserialize_non_negative_option_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<f64>::deserialize(deserializer)?;

    match value {
        None => Ok(value),
        Some(v) => {
            if v < 0.0 {
                Err(de::Error::custom("value must be non-negative"))
            } else {
                Ok(Some(v))
            }
        }
    }
}

pub fn deserialize_non_empty_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    if value.is_empty() {
        Err(de::Error::custom("string cannot be empty"))
    } else {
        Ok(value)
    }
}
