#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// Captures constraints on each element within the resource, profile, or extension.

#[derive(Debug)]
pub struct ElementDefinition_Type<'a> {
    pub value: &'a Value,
}

impl ElementDefinition_Type<'_> {
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

    /// Extensions for versioning
    pub fn _versioning(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_versioning") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Identifies a profile structure or implementation Guide that applies to the
    /// datatype this element refers to. If any profiles are specified, then the content
    /// must conform to at least one of them. The URL can be a local reference - to a
    /// contained StructureDefinition, or a reference to another StructureDefinition or
    /// Implementation Guide by a canonical URL. When an implementation guide is
    /// specified, the type SHALL conform to at least one profile defined in the
    /// implementation guide.
    pub fn profile(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("profile") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for code
    pub fn _code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_code") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Whether this reference needs to be version specific or version independent, or
    /// whether either can be used.
    pub fn versioning(&self) -> Option<ElementDefinition_TypeVersioning> {
        if let Some(Value::String(val)) = self.value.get("versioning") {
            return Some(ElementDefinition_TypeVersioning::from_string(&val).unwrap());
        }
        return None;
    }

    /// URL of Data type or Resource that is a(or the) type used for this element.
    /// References are URLs that are relative to http://hl7.org/fhir/StructureDefinition
    /// e.g. "string" is a reference to http://hl7.org/fhir/StructureDefinition/string.
    /// Absolute URLs are only allowed in logical models.
    pub fn code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("code") {
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

    /// Used when the type is "Reference" or "canonical", and identifies a profile
    /// structure or implementation Guide that applies to the target of the reference
    /// this element refers to. If any profiles are specified, then the content must
    /// conform to at least one of them. The URL can be a local reference - to a
    /// contained StructureDefinition, or a reference to another StructureDefinition or
    /// Implementation Guide by a canonical URL. When an implementation guide is
    /// specified, the target resource SHALL conform to at least one profile defined in
    /// the implementation guide.
    pub fn target_profile(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("targetProfile") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
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

    /// Extensions for aggregation
    pub fn _aggregation(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_aggregation") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._versioning() {
            _val.validate();
        }
        if let Some(_val) = self.profile() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self._code() {
            _val.validate();
        }
        if let Some(_val) = self.versioning() {}
        if let Some(_val) = self.code() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.target_profile() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._aggregation() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

#[derive(Debug)]
pub enum ElementDefinition_TypeVersioning {
    Either,
    Independent,
    Specific,
}

impl ElementDefinition_TypeVersioning {
    pub fn from_string(string: &str) -> Option<ElementDefinition_TypeVersioning> {
        match string {
            "either" => Some(ElementDefinition_TypeVersioning::Either),
            "independent" => Some(ElementDefinition_TypeVersioning::Independent),
            "specific" => Some(ElementDefinition_TypeVersioning::Specific),
            _ => None,
        }
    }
}
