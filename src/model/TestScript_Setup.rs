#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::TestScript_Action::TestScript_Action;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.

#[derive(Debug)]
pub struct TestScript_Setup<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl TestScript_Setup<'_> {
    pub fn new(value: &Value) -> TestScript_Setup {
        TestScript_Setup {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Action would contain either an operation or an assertion.
    pub fn action(&self) -> Vec<TestScript_Action> {
        self.value
            .get("action")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| TestScript_Action {
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
            .action()
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
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct TestScript_SetupBuilder {
    pub(crate) value: Value,
}

impl TestScript_SetupBuilder {
    pub fn build(&self) -> TestScript_Setup {
        TestScript_Setup {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: TestScript_Setup) -> TestScript_SetupBuilder {
        TestScript_SetupBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(action: Vec<TestScript_Action>) -> TestScript_SetupBuilder {
        let mut __value: Value = json!({});
        __value["action"] = json!(action.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return TestScript_SetupBuilder { value: __value };
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut TestScript_SetupBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut TestScript_SetupBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut TestScript_SetupBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
