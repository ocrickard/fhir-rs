#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.

#[derive(Debug)]
pub struct TestScript_Capability<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl TestScript_Capability<'_> {
    pub fn new(value: &Value) -> TestScript_Capability {
        TestScript_Capability {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for destination
    pub fn _destination(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_destination") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for link
    pub fn _link(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_link") {
            return Some(
                val.into_iter()
                    .map(|e| Element {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for origin
    pub fn _origin(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_origin") {
            return Some(
                val.into_iter()
                    .map(|e| Element {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for required
    pub fn _required(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_required") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for validated
    pub fn _validated(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_validated") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Minimum capabilities required of server for test script to execute successfully.
    /// If server does not meet at a minimum the referenced capability statement, then
    /// all tests in this script are skipped.
    pub fn capabilities(&self) -> &str {
        self.value.get("capabilities").unwrap().as_str().unwrap()
    }

    /// Description of the capabilities that this test script is requiring the server to
    /// support.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// Which server these requirements apply to.
    pub fn destination(&self) -> Option<i64> {
        if let Some(val) = self.value.get("destination") {
            return Some(val.as_i64().unwrap());
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

    /// Links to the FHIR specification that describes this interaction and the
    /// resources involved in more detail.
    pub fn link(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("link") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
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

    /// Which origin server these requirements apply to.
    pub fn origin(&self) -> Option<Vec<i64>> {
        if let Some(Value::Array(val)) = self.value.get("origin") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_i64().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Whether or not the test execution will require the given capabilities of the
    /// server in order for this test script to execute.
    pub fn required(&self) -> Option<bool> {
        if let Some(val) = self.value.get("required") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Whether or not the test execution will validate the given capabilities of the
    /// server in order for this test script to execute.
    pub fn validated(&self) -> Option<bool> {
        if let Some(val) = self.value.get("validated") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._destination() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._link() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._origin() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._required() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._validated() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.destination() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.link() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.origin() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.required() {}
        if let Some(_val) = self.validated() {}
        return true;
    }
}

#[derive(Debug)]
pub struct TestScript_CapabilityBuilder {
    pub(crate) value: Value,
}

impl TestScript_CapabilityBuilder {
    pub fn build(&self) -> TestScript_Capability {
        TestScript_Capability {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: TestScript_Capability) -> TestScript_CapabilityBuilder {
        TestScript_CapabilityBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(capabilities: &str) -> TestScript_CapabilityBuilder {
        let mut __value: Value = json!({});
        __value["capabilities"] = json!(capabilities);
        return TestScript_CapabilityBuilder { value: __value };
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut TestScript_CapabilityBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _destination<'a>(&'a mut self, val: Element) -> &'a mut TestScript_CapabilityBuilder {
        self.value["_destination"] = json!(val.value);
        return self;
    }

    pub fn _link<'a>(&'a mut self, val: Vec<Element>) -> &'a mut TestScript_CapabilityBuilder {
        self.value["_link"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _origin<'a>(&'a mut self, val: Vec<Element>) -> &'a mut TestScript_CapabilityBuilder {
        self.value["_origin"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _required<'a>(&'a mut self, val: Element) -> &'a mut TestScript_CapabilityBuilder {
        self.value["_required"] = json!(val.value);
        return self;
    }

    pub fn _validated<'a>(&'a mut self, val: Element) -> &'a mut TestScript_CapabilityBuilder {
        self.value["_validated"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut TestScript_CapabilityBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn destination<'a>(&'a mut self, val: i64) -> &'a mut TestScript_CapabilityBuilder {
        self.value["destination"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut TestScript_CapabilityBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut TestScript_CapabilityBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn link<'a>(&'a mut self, val: Vec<&str>) -> &'a mut TestScript_CapabilityBuilder {
        self.value["link"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut TestScript_CapabilityBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn origin<'a>(&'a mut self, val: Vec<i64>) -> &'a mut TestScript_CapabilityBuilder {
        self.value["origin"] = json!(val);
        return self;
    }

    pub fn required<'a>(&'a mut self, val: bool) -> &'a mut TestScript_CapabilityBuilder {
        self.value["required"] = json!(val);
        return self;
    }

    pub fn validated<'a>(&'a mut self, val: bool) -> &'a mut TestScript_CapabilityBuilder {
        self.value["validated"] = json!(val);
        return self;
    }
}
