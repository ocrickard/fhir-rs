#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::GraphDefinition_Compartment::GraphDefinition_Compartment;
use crate::model::GraphDefinition_Link::GraphDefinition_Link;
use serde_json::value::Value;

/// A formal computable definition of a graph of resources - that is, a coherent set
/// of resources that form a graph by following references. The Graph Definition
/// resource defines a set and makes rules about the set.

#[derive(Debug)]
pub struct GraphDefinition_Target<'a> {
    pub value: &'a Value,
}

impl GraphDefinition_Target<'_> {
    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Additional links from target resource.
    pub fn link(&self) -> Option<Vec<GraphDefinition_Link>> {
        if let Some(Value::Array(val)) = self.value.get("link") {
            return Some(
                val.into_iter()
                    .map(|e| GraphDefinition_Link { value: e })
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

    /// A set of parameters to look up.
    pub fn params(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("params") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for params
    pub fn _params(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_params") {
            return Some(Element { value: val });
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

    /// Profile for the target resource.
    pub fn profile(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("profile") {
            return Some(string);
        }
        return None;
    }

    /// Type of resource this link refers to.
    pub fn fhir_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("type") {
            return Some(string);
        }
        return None;
    }

    /// Compartment Consistency Rules.
    pub fn compartment(&self) -> Option<Vec<GraphDefinition_Compartment>> {
        if let Some(Value::Array(val)) = self.value.get("compartment") {
            return Some(
                val.into_iter()
                    .map(|e| GraphDefinition_Compartment { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._type() {
            _val.validate();
        }
        if let Some(_val) = self.link() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
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
        if let Some(_val) = self.params() {}
        if let Some(_val) = self._params() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.profile() {}
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.compartment() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
