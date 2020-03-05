#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::ExampleScenario_Alternative::ExampleScenario_Alternative;
use crate::model::ExampleScenario_Operation::ExampleScenario_Operation;
use crate::model::ExampleScenario_Process::ExampleScenario_Process;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Example of workflow instance.

#[derive(Debug)]
pub struct ExampleScenario_Step<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ExampleScenario_Step<'_> {
    pub fn new(value: &Value) -> ExampleScenario_Step {
        ExampleScenario_Step {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for pause
    pub fn _pause(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_pause") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates an alternative step that can be taken instead of the operations on the
    /// base step in exceptional/atypical circumstances.
    pub fn alternative(&self) -> Option<Vec<ExampleScenario_Alternative>> {
        if let Some(Value::Array(val)) = self.value.get("alternative") {
            return Some(
                val.into_iter()
                    .map(|e| ExampleScenario_Alternative {
                        value: Cow::Borrowed(e),
                    })
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

    /// Each interaction or action.
    pub fn operation(&self) -> Option<ExampleScenario_Operation> {
        if let Some(val) = self.value.get("operation") {
            return Some(ExampleScenario_Operation {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// If there is a pause in the flow.
    pub fn pause(&self) -> Option<bool> {
        if let Some(val) = self.value.get("pause") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Nested process.
    pub fn process(&self) -> Option<Vec<ExampleScenario_Process>> {
        if let Some(Value::Array(val)) = self.value.get("process") {
            return Some(
                val.into_iter()
                    .map(|e| ExampleScenario_Process {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._pause() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.alternative() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
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
        if let Some(_val) = self.operation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.pause() {}
        if let Some(_val) = self.process() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ExampleScenario_StepBuilder {
    pub(crate) value: Value,
}

impl ExampleScenario_StepBuilder {
    pub fn build(&self) -> ExampleScenario_Step {
        ExampleScenario_Step {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ExampleScenario_Step) -> ExampleScenario_StepBuilder {
        ExampleScenario_StepBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ExampleScenario_StepBuilder {
        let mut __value: Value = json!({});
        return ExampleScenario_StepBuilder { value: __value };
    }

    pub fn _pause<'a>(&'a mut self, val: Element) -> &'a mut ExampleScenario_StepBuilder {
        self.value["_pause"] = json!(val.value);
        return self;
    }

    pub fn alternative<'a>(
        &'a mut self,
        val: Vec<ExampleScenario_Alternative>,
    ) -> &'a mut ExampleScenario_StepBuilder {
        self.value["alternative"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ExampleScenario_StepBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ExampleScenario_StepBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ExampleScenario_StepBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn operation<'a>(
        &'a mut self,
        val: ExampleScenario_Operation,
    ) -> &'a mut ExampleScenario_StepBuilder {
        self.value["operation"] = json!(val.value);
        return self;
    }

    pub fn pause<'a>(&'a mut self, val: bool) -> &'a mut ExampleScenario_StepBuilder {
        self.value["pause"] = json!(val);
        return self;
    }

    pub fn process<'a>(
        &'a mut self,
        val: Vec<ExampleScenario_Process>,
    ) -> &'a mut ExampleScenario_StepBuilder {
        self.value["process"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
