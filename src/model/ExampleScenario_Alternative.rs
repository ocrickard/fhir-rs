#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::ExampleScenario_Step::ExampleScenario_Step;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Example of workflow instance.

#[derive(Debug)]
pub struct ExampleScenario_Alternative<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ExampleScenario_Alternative<'_> {
    pub fn new(value: &Value) -> ExampleScenario_Alternative {
        ExampleScenario_Alternative {
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

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A human-readable description of the alternative explaining when the alternative
    /// should occur rather than the base step.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
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

    /// What happens in each alternative option.
    pub fn step(&self) -> Option<Vec<ExampleScenario_Step>> {
        if let Some(Value::Array(val)) = self.value.get("step") {
            return Some(
                val.into_iter()
                    .map(|e| ExampleScenario_Step {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The label to display for the alternative that gives a sense of the circumstance
    /// in which the alternative should be invoked.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._title() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
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
        if let Some(_val) = self.step() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.title() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ExampleScenario_AlternativeBuilder {
    pub(crate) value: Value,
}

impl ExampleScenario_AlternativeBuilder {
    pub fn build(&self) -> ExampleScenario_Alternative {
        ExampleScenario_Alternative {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ExampleScenario_Alternative) -> ExampleScenario_AlternativeBuilder {
        ExampleScenario_AlternativeBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ExampleScenario_AlternativeBuilder {
        let mut __value: Value = json!({});
        return ExampleScenario_AlternativeBuilder { value: __value };
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ExampleScenario_AlternativeBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _title<'a>(&'a mut self, val: Element) -> &'a mut ExampleScenario_AlternativeBuilder {
        self.value["_title"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut ExampleScenario_AlternativeBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ExampleScenario_AlternativeBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ExampleScenario_AlternativeBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ExampleScenario_AlternativeBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn step<'a>(
        &'a mut self,
        val: Vec<ExampleScenario_Step>,
    ) -> &'a mut ExampleScenario_AlternativeBuilder {
        self.value["step"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn title<'a>(&'a mut self, val: &str) -> &'a mut ExampleScenario_AlternativeBuilder {
        self.value["title"] = json!(val);
        return self;
    }
}
