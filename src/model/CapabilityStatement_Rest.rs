#![allow(unused_imports, non_camel_case_types)]

use crate::model::CapabilityStatement_Interaction1::CapabilityStatement_Interaction1;
use crate::model::CapabilityStatement_Operation::CapabilityStatement_Operation;
use crate::model::CapabilityStatement_Resource::CapabilityStatement_Resource;
use crate::model::CapabilityStatement_SearchParam::CapabilityStatement_SearchParam;
use crate::model::CapabilityStatement_Security::CapabilityStatement_Security;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR
/// Server for a particular version of FHIR that may be used as a statement of
/// actual server functionality or a statement of required or desired server
/// implementation.

#[derive(Debug)]
pub struct CapabilityStatement_Rest<'a> {
    pub value: &'a Value,
}

impl CapabilityStatement_Rest<'_> {
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

    /// Extensions for documentation
    pub fn _documentation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_documentation") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A specification of the restful capabilities of the solution for a specific
    /// resource type.
    pub fn resource(&self) -> Option<Vec<CapabilityStatement_Resource>> {
        if let Some(Value::Array(val)) = self.value.get("resource") {
            return Some(
                val.into_iter()
                    .map(|e| CapabilityStatement_Resource { value: e })
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

    /// Information about the system's restful capabilities that apply across all
    /// applications, such as security.
    pub fn documentation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("documentation") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for mode
    pub fn _mode(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_mode") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A specification of restful operations supported by the system.
    pub fn interaction(&self) -> Option<Vec<CapabilityStatement_Interaction1>> {
        if let Some(Value::Array(val)) = self.value.get("interaction") {
            return Some(
                val.into_iter()
                    .map(|e| CapabilityStatement_Interaction1 { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Identifies whether this portion of the statement is describing the ability to
    /// initiate or receive restful operations.
    pub fn mode(&self) -> Option<CapabilityStatement_RestMode> {
        if let Some(Value::String(val)) = self.value.get("mode") {
            return Some(CapabilityStatement_RestMode::from_string(&val).unwrap());
        }
        return None;
    }

    /// Search parameters that are supported for searching all resources for
    /// implementations to support and/or make use of - either references to ones
    /// defined in the specification, or additional ones defined for/by the
    /// implementation.
    pub fn search_param(&self) -> Option<Vec<CapabilityStatement_SearchParam>> {
        if let Some(Value::Array(val)) = self.value.get("searchParam") {
            return Some(
                val.into_iter()
                    .map(|e| CapabilityStatement_SearchParam { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Definition of an operation or a named query together with its parameters and
    /// their meaning and type.
    pub fn operation(&self) -> Option<Vec<CapabilityStatement_Operation>> {
        if let Some(Value::Array(val)) = self.value.get("operation") {
            return Some(
                val.into_iter()
                    .map(|e| CapabilityStatement_Operation { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An absolute URI which is a reference to the definition of a compartment that the
    /// system supports. The reference is to a CompartmentDefinition resource by its
    /// canonical URL .
    pub fn compartment(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("compartment") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Information about security implementation from an interface perspective - what a
    /// client needs to know.
    pub fn security(&self) -> Option<CapabilityStatement_Security> {
        if let Some(val) = self.value.get("security") {
            return Some(CapabilityStatement_Security { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._documentation() {
            _val.validate();
        }
        if let Some(_val) = self.resource() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.documentation() {}
        if let Some(_val) = self._mode() {
            _val.validate();
        }
        if let Some(_val) = self.interaction() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.mode() {}
        if let Some(_val) = self.search_param() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.operation() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.compartment() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.security() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum CapabilityStatement_RestMode {
    Client,
    Server,
}

impl CapabilityStatement_RestMode {
    pub fn from_string(string: &str) -> Option<CapabilityStatement_RestMode> {
        match string {
            "client" => Some(CapabilityStatement_RestMode::Client),
            "server" => Some(CapabilityStatement_RestMode::Server),
            _ => None,
        }
    }
}
