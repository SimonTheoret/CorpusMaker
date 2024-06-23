use serde_json::Value;
use std::fs::File;
use std::ops::Deref;

pub struct Example {
    file: File,
    content: String,
    length: Option<u32>,
}

pub trait ToExample: Deref<Target = Example> {}

pub struct JsonExample {
    example: Example,
    json_content: Value,
}

impl Deref for JsonExample {
    type Target = Example;
    fn deref(&self) -> &Self::Target {
        &self.example
    }
}

impl ToExample for JsonExample {}

