#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A summary of information based on the results of executing a TestScript.

#[derive(Debug)]
pub struct TestReport_Assert<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl TestReport_Assert<'_> {
    pub fn new(value: &Value) -> TestReport_Assert {
        TestReport_Assert {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for detail
    pub fn _detail(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_detail") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for message
    pub fn _message(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_message") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for result
    pub fn _result(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_result") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A link to further details on the result.
    pub fn detail(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("detail") {
            return Some(string);
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

    /// An explanatory message associated with the result.
    pub fn message(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("message") {
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

    /// The result of this assertion.
    pub fn result(&self) -> Option<TestReport_AssertResult> {
        if let Some(Value::String(val)) = self.value.get("result") {
            return Some(TestReport_AssertResult::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._detail() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._message() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._result() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.detail() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.message() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.result() {}
        return true;
    }
}

#[derive(Debug)]
pub struct TestReport_AssertBuilder {
    pub(crate) value: Value,
}

impl TestReport_AssertBuilder {
    pub fn build(&self) -> TestReport_Assert {
        TestReport_Assert {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: TestReport_Assert) -> TestReport_AssertBuilder {
        TestReport_AssertBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> TestReport_AssertBuilder {
        let mut __value: Value = json!({});
        return TestReport_AssertBuilder { value: __value };
    }

    pub fn _detail<'a>(&'a mut self, val: Element) -> &'a mut TestReport_AssertBuilder {
        self.value["_detail"] = json!(val.value);
        return self;
    }

    pub fn _message<'a>(&'a mut self, val: Element) -> &'a mut TestReport_AssertBuilder {
        self.value["_message"] = json!(val.value);
        return self;
    }

    pub fn _result<'a>(&'a mut self, val: Element) -> &'a mut TestReport_AssertBuilder {
        self.value["_result"] = json!(val.value);
        return self;
    }

    pub fn detail<'a>(&'a mut self, val: &str) -> &'a mut TestReport_AssertBuilder {
        self.value["detail"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut TestReport_AssertBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut TestReport_AssertBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn message<'a>(&'a mut self, val: &str) -> &'a mut TestReport_AssertBuilder {
        self.value["message"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut TestReport_AssertBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn result<'a>(
        &'a mut self,
        val: TestReport_AssertResult,
    ) -> &'a mut TestReport_AssertBuilder {
        self.value["result"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum TestReport_AssertResult {
    Pass,
    Skip,
    Fail,
    Warning,
    Error,
}

impl TestReport_AssertResult {
    pub fn from_string(string: &str) -> Option<TestReport_AssertResult> {
        match string {
            "pass" => Some(TestReport_AssertResult::Pass),
            "skip" => Some(TestReport_AssertResult::Skip),
            "fail" => Some(TestReport_AssertResult::Fail),
            "warning" => Some(TestReport_AssertResult::Warning),
            "error" => Some(TestReport_AssertResult::Error),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TestReport_AssertResult::Pass => "pass".to_string(),
            TestReport_AssertResult::Skip => "skip".to_string(),
            TestReport_AssertResult::Fail => "fail".to_string(),
            TestReport_AssertResult::Warning => "warning".to_string(),
            TestReport_AssertResult::Error => "error".to_string(),
        }
    }
}
