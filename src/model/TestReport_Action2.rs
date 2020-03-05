#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::TestReport_Operation::TestReport_Operation;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A summary of information based on the results of executing a TestScript.

#[derive(Debug)]
pub struct TestReport_Action2<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl TestReport_Action2<'_> {
    pub fn new(value: &Value) -> TestReport_Action2 {
        TestReport_Action2 {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// An operation would involve a REST request to a server.
    pub fn operation(&self) -> TestReport_Operation {
        TestReport_Operation {
            value: Cow::Borrowed(&self.value["operation"]),
        }
    }

    pub fn validate(&self) -> bool {
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
        if !self.operation().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct TestReport_Action2Builder {
    pub(crate) value: Value,
}

impl TestReport_Action2Builder {
    pub fn build(&self) -> TestReport_Action2 {
        TestReport_Action2 {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: TestReport_Action2) -> TestReport_Action2Builder {
        TestReport_Action2Builder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(operation: TestReport_Operation) -> TestReport_Action2Builder {
        let mut __value: Value = json!({});
        __value["operation"] = json!(operation.value);
        return TestReport_Action2Builder { value: __value };
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut TestReport_Action2Builder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut TestReport_Action2Builder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut TestReport_Action2Builder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
