#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A summary of information based on the results of executing a TestScript.

#[derive(Debug)]
pub struct TestReport_Participant<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl TestReport_Participant<'_> {
    pub fn new(value: &Value) -> TestReport_Participant {
        TestReport_Participant {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for display
    pub fn _display(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_display") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for uri
    pub fn _uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_uri") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The display name of the participant.
    pub fn display(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("display") {
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

    /// The type of participant.
    pub fn fhir_type(&self) -> Option<TestReport_ParticipantType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(TestReport_ParticipantType::from_string(&val).unwrap());
        }
        return None;
    }

    /// The uri of the participant. An absolute URL is preferred.
    pub fn uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("uri") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._display() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._uri() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.display() {}
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
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.uri() {}
        return true;
    }
}

#[derive(Debug)]
pub struct TestReport_ParticipantBuilder {
    pub(crate) value: Value,
}

impl TestReport_ParticipantBuilder {
    pub fn build(&self) -> TestReport_Participant {
        TestReport_Participant {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: TestReport_Participant) -> TestReport_ParticipantBuilder {
        TestReport_ParticipantBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> TestReport_ParticipantBuilder {
        let mut __value: Value = json!({});
        return TestReport_ParticipantBuilder { value: __value };
    }

    pub fn _display<'a>(&'a mut self, val: Element) -> &'a mut TestReport_ParticipantBuilder {
        self.value["_display"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut TestReport_ParticipantBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn _uri<'a>(&'a mut self, val: Element) -> &'a mut TestReport_ParticipantBuilder {
        self.value["_uri"] = json!(val.value);
        return self;
    }

    pub fn display<'a>(&'a mut self, val: &str) -> &'a mut TestReport_ParticipantBuilder {
        self.value["display"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut TestReport_ParticipantBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut TestReport_ParticipantBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut TestReport_ParticipantBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: TestReport_ParticipantType,
    ) -> &'a mut TestReport_ParticipantBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
    }

    pub fn uri<'a>(&'a mut self, val: &str) -> &'a mut TestReport_ParticipantBuilder {
        self.value["uri"] = json!(val);
        return self;
    }
}

#[derive(Debug)]
pub enum TestReport_ParticipantType {
    TestEngine,
    Client,
    Server,
}

impl TestReport_ParticipantType {
    pub fn from_string(string: &str) -> Option<TestReport_ParticipantType> {
        match string {
            "test-engine" => Some(TestReport_ParticipantType::TestEngine),
            "client" => Some(TestReport_ParticipantType::Client),
            "server" => Some(TestReport_ParticipantType::Server),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TestReport_ParticipantType::TestEngine => "test-engine".to_string(),
            TestReport_ParticipantType::Client => "client".to_string(),
            TestReport_ParticipantType::Server => "server".to_string(),
        }
    }
}
