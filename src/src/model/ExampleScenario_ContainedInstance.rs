#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Example of workflow instance.

#[derive(Debug)]
pub struct ExampleScenario_ContainedInstance<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ExampleScenario_ContainedInstance<'_> {
    pub fn new(value: &Value) -> ExampleScenario_ContainedInstance {
        ExampleScenario_ContainedInstance {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for resourceId
    pub fn _resource_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_resourceId") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for versionId
    pub fn _version_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_versionId") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
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

    /// Each resource contained in the instance.
    pub fn resource_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("resourceId") {
            return Some(string);
        }
        return None;
    }

    /// A specific version of a resource contained in the instance.
    pub fn version_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("versionId") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._resource_id() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._version_id() {
            if !_val.validate() {
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
        if let Some(_val) = self.resource_id() {}
        if let Some(_val) = self.version_id() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ExampleScenario_ContainedInstanceBuilder {
    pub(crate) value: Value,
}

impl ExampleScenario_ContainedInstanceBuilder {
    pub fn build(&self) -> ExampleScenario_ContainedInstance {
        ExampleScenario_ContainedInstance {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: ExampleScenario_ContainedInstance,
    ) -> ExampleScenario_ContainedInstanceBuilder {
        ExampleScenario_ContainedInstanceBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ExampleScenario_ContainedInstanceBuilder {
        let mut __value: Value = json!({});
        return ExampleScenario_ContainedInstanceBuilder { value: __value };
    }

    pub fn _resource_id<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ExampleScenario_ContainedInstanceBuilder {
        self.value["_resourceId"] = json!(val.value);
        return self;
    }

    pub fn _version_id<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ExampleScenario_ContainedInstanceBuilder {
        self.value["_versionId"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ExampleScenario_ContainedInstanceBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ExampleScenario_ContainedInstanceBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ExampleScenario_ContainedInstanceBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn resource_id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ExampleScenario_ContainedInstanceBuilder {
        self.value["resourceId"] = json!(val);
        return self;
    }

    pub fn version_id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ExampleScenario_ContainedInstanceBuilder {
        self.value["versionId"] = json!(val);
        return self;
    }
}
