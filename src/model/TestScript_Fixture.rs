#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.

#[derive(Debug)]
pub struct TestScript_Fixture<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl TestScript_Fixture<'_> {
    pub fn new(value: &Value) -> TestScript_Fixture {
        TestScript_Fixture {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for autocreate
    pub fn _autocreate(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_autocreate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for autodelete
    pub fn _autodelete(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_autodelete") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Whether or not to implicitly create the fixture during setup. If true, the
    /// fixture is automatically created on each server being tested during setup,
    /// therefore no create operation is required for this fixture in the
    /// TestScript.setup section.
    pub fn autocreate(&self) -> Option<bool> {
        if let Some(val) = self.value.get("autocreate") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Whether or not to implicitly delete the fixture during teardown. If true, the
    /// fixture is automatically deleted on each server being tested during teardown,
    /// therefore no delete operation is required for this fixture in the
    /// TestScript.teardown section.
    pub fn autodelete(&self) -> Option<bool> {
        if let Some(val) = self.value.get("autodelete") {
            return Some(val.as_bool().unwrap());
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

    /// Reference to the resource (containing the contents of the resource needed for
    /// operations).
    pub fn resource(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("resource") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._autocreate() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._autodelete() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.autocreate() {}
        if let Some(_val) = self.autodelete() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.resource() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct TestScript_FixtureBuilder {
    pub(crate) value: Value,
}

impl TestScript_FixtureBuilder {
    pub fn build(&self) -> TestScript_Fixture {
        TestScript_Fixture {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: TestScript_Fixture) -> TestScript_FixtureBuilder {
        TestScript_FixtureBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> TestScript_FixtureBuilder {
        let mut __value: Value = json!({});
        return TestScript_FixtureBuilder { value: __value };
    }

    pub fn _autocreate<'a>(&'a mut self, val: Element) -> &'a mut TestScript_FixtureBuilder {
        self.value["_autocreate"] = json!(val.value);
        return self;
    }

    pub fn _autodelete<'a>(&'a mut self, val: Element) -> &'a mut TestScript_FixtureBuilder {
        self.value["_autodelete"] = json!(val.value);
        return self;
    }

    pub fn autocreate<'a>(&'a mut self, val: bool) -> &'a mut TestScript_FixtureBuilder {
        self.value["autocreate"] = json!(val);
        return self;
    }

    pub fn autodelete<'a>(&'a mut self, val: bool) -> &'a mut TestScript_FixtureBuilder {
        self.value["autodelete"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut TestScript_FixtureBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut TestScript_FixtureBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut TestScript_FixtureBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn resource<'a>(&'a mut self, val: Reference) -> &'a mut TestScript_FixtureBuilder {
        self.value["resource"] = json!(val.value);
        return self;
    }
}
