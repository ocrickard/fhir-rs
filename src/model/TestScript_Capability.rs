#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.

#[derive(Debug)]
pub struct TestScript_Capability<'a> {
    pub value: &'a Value,
}

impl TestScript_Capability<'_> {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for validated
    pub fn _validated(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_validated") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for required
    pub fn _required(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_required") {
            return Some(Element { value: val });
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

    /// Minimum capabilities required of server for test script to execute successfully.
    /// If server does not meet at a minimum the referenced capability statement, then
    /// all tests in this script are skipped.
    pub fn capabilities(&self) -> &str {
        self.value.get("capabilities").unwrap().as_str().unwrap()
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
                    .map(|e| Extension { value: e })
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
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Whether or not the test execution will validate the given capabilities of the
    /// server in order for this test script to execute.
    pub fn validated(&self) -> Option<bool> {
        if let Some(val) = self.value.get("validated") {
            return Some(val.as_bool().unwrap());
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

    /// Extensions for destination
    pub fn _destination(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_destination") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Description of the capabilities that this test script is requiring the server to
    /// support.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for link
    pub fn _link(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_link") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._validated() {
            _val.validate();
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self._required() {
            _val.validate();
        }
        if let Some(_val) = self.required() {}
        let _ = self.capabilities();
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._origin() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.link() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.validated() {}
        if let Some(_val) = self.origin() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self._destination() {
            _val.validate();
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self._link() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.destination() {}
        return true;
    }
}
