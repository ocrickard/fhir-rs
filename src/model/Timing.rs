#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Timing_Repeat::Timing_Repeat;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Specifies an event that may occur multiple times. Timing schedules are used to
/// record when things are planned, expected or requested to occur. The most common
/// usage is in dosage instructions for medications. They are also used when
/// planning care of various kinds, and may be used for reporting the schedule to
/// which past regular activities were carried out.

#[derive(Debug)]
pub struct Timing<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Timing<'_> {
    pub fn new(value: &Value) -> Timing {
        Timing {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for event
    pub fn _event(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_event") {
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

    /// A code for the timing schedule (or just text in code.text). Some codes such as
    /// BID are ubiquitous, but many institutions define their own additional codes. If
    /// a code is provided, the code is understood to be a complete statement of
    /// whatever is specified in the structured timing data, and either the code or the
    /// data may be used to interpret the Timing, with the exception that .repeat.bounds
    /// still applies over the code (and is not contained in the code).
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifies specific times when the event occurs.
    pub fn event(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("event") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
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

    /// A set of rules that describe when the event is scheduled.
    pub fn repeat(&self) -> Option<Timing_Repeat> {
        if let Some(val) = self.value.get("repeat") {
            return Some(Timing_Repeat {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._event() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.event() {
            _val.into_iter().for_each(|_e| {});
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
        if let Some(_val) = self.repeat() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct TimingBuilder {
    pub(crate) value: Value,
}

impl TimingBuilder {
    pub fn build(&self) -> Timing {
        Timing {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Timing) -> TimingBuilder {
        TimingBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> TimingBuilder {
        let mut __value: Value = json!({});
        return TimingBuilder { value: __value };
    }

    pub fn _event<'a>(&'a mut self, val: Vec<Element>) -> &'a mut TimingBuilder {
        self.value["_event"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn code<'a>(&'a mut self, val: CodeableConcept) -> &'a mut TimingBuilder {
        self.value["code"] = json!(val.value);
        return self;
    }

    pub fn event<'a>(&'a mut self, val: Vec<&str>) -> &'a mut TimingBuilder {
        self.value["event"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut TimingBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut TimingBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut TimingBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn repeat<'a>(&'a mut self, val: Timing_Repeat) -> &'a mut TimingBuilder {
        self.value["repeat"] = json!(val.value);
        return self;
    }
}
