//! TODO: documentation

#![allow(single_use_lifetimes)]

/// TODO: documentation
#[derive(Debug)]
pub struct MetadataMapMut<'a>(pub &'a mut tonic::metadata::MetadataMap);

impl<'a> opentelemetry::propagation::Injector for MetadataMapMut<'a> {
    /// Set a key and value in the `MetadataMap`.  Does nothing if the key or value are not valid inputs
    fn set(&mut self, key: &str, value: String) {
        if let Ok(key) = tonic::metadata::MetadataKey::from_bytes(key.as_bytes()) {
            if let Ok(value) = tonic::metadata::MetadataValue::from_str(&value) {
                let _value = self.0.insert(key, value);
            }
        }
    }
}

/// TODO: documentation
#[derive(Debug)]
pub struct MetadataMap<'a>(pub &'a tonic::metadata::MetadataMap);

impl<'a> opentelemetry::propagation::Extractor for MetadataMap<'a> {
    /// Get a value for a key from the `MetadataMap`.  If the value can't be converted to &str, returns None
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|metadata| metadata.to_str().ok())
    }

    /// Collect all the keys from the `MetadataMap`.
    fn keys(&self) -> Vec<&str> {
        self.0
            .keys()
            .map(|key| match key {
                tonic::metadata::KeyRef::Ascii(v) => v.as_str(),
                tonic::metadata::KeyRef::Binary(v) => v.as_str(),
            })
            .collect()
    }
}
