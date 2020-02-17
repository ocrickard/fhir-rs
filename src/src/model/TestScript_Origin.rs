#![allow(unused_imports, non_camel_case_types)]

use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.

#[derive(Debug)]
pub struct TestScript_Origin<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl TestScript_Origin<'_> {
    pub fn new(value: &Value) -> TestScript_Origin {
        TestScript_Origin {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for index
    pub fn _index(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_index") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
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

    /// Abstract name given to an origin server in this test script.  The name is
    /// provided as a number starting at 1.
    pub fn index(&self) -> Option<i64> {
        if let Some(val) = self.value.get("index") {
            return Some(val.as_i64().unwrap());
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

    /// The type of origin profile the test system supports.
    pub fn profile(&self) -> Coding {
        Coding {
            value: Cow::Borrowed(&self.value["profile"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._index() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.index() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.profile().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct TestScript_OriginBuilder {
    pub(crate) value: Value,
}

impl TestScript_OriginBuilder {
    pub fn build(&self) -> TestScript_Origin {
        TestScript_Origin {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: TestScript_Origin) -> TestScript_OriginBuilder {
        TestScript_OriginBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(profile: Coding) -> TestScript_OriginBuilder {
        let mut __value: Value = json!({});
        __value["profile"] = json!(profile.value);
        return TestScript_OriginBuilder { value: __value };
    }

    pub fn _index<'a>(&'a mut self, val: Element) -> &'a mut TestScript_OriginBuilder {
        self.value["_index"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut TestScript_OriginBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut TestScript_OriginBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn index<'a>(&'a mut self, val: i64) -> &'a mut TestScript_OriginBuilder {
        self.value["index"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut TestScript_OriginBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
