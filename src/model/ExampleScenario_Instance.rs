#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::ExampleScenario_ContainedInstance::ExampleScenario_ContainedInstance;
use crate::model::ExampleScenario_Version::ExampleScenario_Version;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// Example of workflow instance.

#[derive(Debug)]
pub struct ExampleScenario_Instance<'a> {
    pub value: &'a Value,
}

impl ExampleScenario_Instance<'_> {
    /// The type of the resource.
    pub fn resource_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("resourceType") {
            return Some(string);
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

    /// Human-friendly description of the resource instance.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Resources contained in the instance (e.g. the observations contained in a
    /// bundle).
    pub fn contained_instance(&self) -> Option<Vec<ExampleScenario_ContainedInstance>> {
        if let Some(Value::Array(val)) = self.value.get("containedInstance") {
            return Some(
                val.into_iter()
                    .map(|e| ExampleScenario_ContainedInstance { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The id of the resource for referencing.
    pub fn resource_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("resourceId") {
            return Some(string);
        }
        return None;
    }

    /// A specific version of the resource.
    pub fn version(&self) -> Option<Vec<ExampleScenario_Version>> {
        if let Some(Value::Array(val)) = self.value.get("version") {
            return Some(
                val.into_iter()
                    .map(|e| ExampleScenario_Version { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for resourceType
    pub fn _resource_type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_resourceType") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A short name for the resource instance.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for resourceId
    pub fn _resource_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_resourceId") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.resource_type() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.description() {}
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.contained_instance() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.resource_id() {}
        if let Some(_val) = self.version() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._resource_type() {
            _val.validate();
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._name() {
            _val.validate();
        }
        if let Some(_val) = self._resource_id() {
            _val.validate();
        }
        return true;
    }
}
