#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::TestScript_Capability::TestScript_Capability;
use crate::model::TestScript_Link::TestScript_Link;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.

#[derive(Debug)]
pub struct TestScript_Metadata<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl TestScript_Metadata<'_> {
    pub fn new(value: &Value) -> TestScript_Metadata {
        TestScript_Metadata {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Capabilities that must exist and are assumed to function correctly on the FHIR
    /// server being tested.
    pub fn capability(&self) -> Vec<TestScript_Capability> {
        self.value
            .get("capability")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| TestScript_Capability {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element. To make the use of extensions safe and manageable,
    /// there is a strict set of governance  applied to the definition and use of
    /// extensions. Though any implementer can define an extension, there is a set of
    /// requirements that SHALL be met as part of the definition of the extension.
    pub fn extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("extension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// A link to the FHIR specification that this test is covering.
    pub fn link(&self) -> Option<Vec<TestScript_Link>> {
        if let Some(Value::Array(val)) = self.value.get("link") {
            return Some(
                val.into_iter()
                    .map(|e| TestScript_Link {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element and that modifies the understanding of the element in
    /// which it is contained and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer can define an extension, there is a set of requirements that SHALL
    /// be met as part of the definition of the extension. Applications processing a
    /// resource are required to check for modifier extensions.    Modifier extensions
    /// SHALL NOT change the meaning of any elements on Resource or DomainResource
    /// (including cannot change the meaning of modifierExtension itself).
    pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if !self
            .capability()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.link() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct TestScript_MetadataBuilder {
    pub(crate) value: Value,
}

impl TestScript_MetadataBuilder {
    pub fn build(&self) -> TestScript_Metadata {
        TestScript_Metadata {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: TestScript_Metadata) -> TestScript_MetadataBuilder {
        TestScript_MetadataBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(capability: Vec<TestScript_Capability>) -> TestScript_MetadataBuilder {
        let mut __value: Value = json!({});
        __value["capability"] = json!(capability.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return TestScript_MetadataBuilder { value: __value };
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut TestScript_MetadataBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut TestScript_MetadataBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn link<'a>(&'a mut self, val: Vec<TestScript_Link>) -> &'a mut TestScript_MetadataBuilder {
        self.value["link"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut TestScript_MetadataBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
