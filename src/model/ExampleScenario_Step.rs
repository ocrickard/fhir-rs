#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::ExampleScenario_Alternative::ExampleScenario_Alternative;
use crate::model::ExampleScenario_Operation::ExampleScenario_Operation;
use crate::model::ExampleScenario_Process::ExampleScenario_Process;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// Example of workflow instance.

#[derive(Debug)]
pub struct ExampleScenario_Step<'a> {
    pub value: &'a Value,
}

impl ExampleScenario_Step<'_> {
    /// Extensions for pause
    pub fn _pause(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_pause") {
            return Some(Element { value: val });
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

    /// Each interaction or action.
    pub fn operation(&self) -> Option<ExampleScenario_Operation> {
        if let Some(val) = self.value.get("operation") {
            return Some(ExampleScenario_Operation { value: val });
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Nested process.
    pub fn process(&self) -> Option<Vec<ExampleScenario_Process>> {
        if let Some(Value::Array(val)) = self.value.get("process") {
            return Some(
                val.into_iter()
                    .map(|e| ExampleScenario_Process { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates an alternative step that can be taken instead of the operations on the
    /// base step in exceptional/atypical circumstances.
    pub fn alternative(&self) -> Option<Vec<ExampleScenario_Alternative>> {
        if let Some(Value::Array(val)) = self.value.get("alternative") {
            return Some(
                val.into_iter()
                    .map(|e| ExampleScenario_Alternative { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._pause() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.operation() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.pause() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.process() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.alternative() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
