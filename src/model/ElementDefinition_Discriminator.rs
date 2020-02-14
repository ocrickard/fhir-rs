#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// Captures constraints on each element within the resource, profile, or extension.

#[derive(Debug)]
pub struct ElementDefinition_Discriminator<'a> {
    pub value: &'a Value,
}

impl ElementDefinition_Discriminator<'_> {
    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A FHIRPath expression, using [the simple subset of
    /// FHIRPath](fhirpath.html#simple), that is used to identify the element on which
    /// discrimination is based.
    pub fn path(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("path") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for path
    pub fn _path(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_path") {
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

    /// How the element value is interpreted when discrimination is evaluated.
    pub fn fhir_type(&self) -> Option<ElementDefinition_DiscriminatorType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(ElementDefinition_DiscriminatorType::from_string(&val).unwrap());
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._type() {
            _val.validate();
        }
        if let Some(_val) = self.path() {}
        if let Some(_val) = self._path() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

#[derive(Debug)]
pub enum ElementDefinition_DiscriminatorType {
    Value,
    Exists,
    Pattern,
    FhirType,
    Profile,
}

impl ElementDefinition_DiscriminatorType {
    pub fn from_string(string: &str) -> Option<ElementDefinition_DiscriminatorType> {
        match string {
            "value" => Some(ElementDefinition_DiscriminatorType::Value),
            "exists" => Some(ElementDefinition_DiscriminatorType::Exists),
            "pattern" => Some(ElementDefinition_DiscriminatorType::Pattern),
            "type" => Some(ElementDefinition_DiscriminatorType::FhirType),
            "profile" => Some(ElementDefinition_DiscriminatorType::Profile),
            _ => None,
        }
    }
}
