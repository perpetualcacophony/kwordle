impl serde::Serialize for super::Letter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        char::serialize(&self.as_char(), serializer)
    }
}

impl<'de> serde::Deserialize<'de> for super::Letter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let ch = char::deserialize(deserializer)?;
        Self::from_char(ch).ok_or(serde::de::Error::custom("unrecognized char"))
    }
}
