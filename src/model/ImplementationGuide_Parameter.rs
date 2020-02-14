#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used to
/// gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.

#[derive(Debug)]
pub struct ImplementationGuide_Parameter<'a> {
    pub value: &'a Value,
}

impl ImplementationGuide_Parameter<'_> {
    /// apply | path-resource | path-pages | path-tx-cache | expansion-parameter | rule-
    /// broken-links | generate-xml | generate-json | generate-turtle | html-
    /// template.
    pub fn code(&self) -> Option<ImplementationGuide_ParameterCode> {
        if let Some(Value::String(val)) = self.value.get("code") {
            return Some(ImplementationGuide_ParameterCode::from_string(&val).unwrap());
        }
        return None;
    }

    /// Extensions for value
    pub fn _value(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_value") {
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

    /// Extensions for code
    pub fn _code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_code") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Value for named type.
    pub fn value(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("value") {
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
        if let Some(_val) = self.code() {}
        if let Some(_val) = self._value() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._code() {
            _val.validate();
        }
        if let Some(_val) = self.value() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

#[derive(Debug)]
pub enum ImplementationGuide_ParameterCode {
    Apply,
    PathResource,
    PathPages,
    PathTxCache,
    ExpansionParameter,
    RuleBrokenLinks,
    GenerateXml,
    GenerateJson,
    GenerateTurtle,
    HtmlTemplate,
}

impl ImplementationGuide_ParameterCode {
    pub fn from_string(string: &str) -> Option<ImplementationGuide_ParameterCode> {
        match string {
            "apply" => Some(ImplementationGuide_ParameterCode::Apply),
            "path-resource" => Some(ImplementationGuide_ParameterCode::PathResource),
            "path-pages" => Some(ImplementationGuide_ParameterCode::PathPages),
            "path-tx-cache" => Some(ImplementationGuide_ParameterCode::PathTxCache),
            "expansion-parameter" => Some(ImplementationGuide_ParameterCode::ExpansionParameter),
            "rule-broken-links" => Some(ImplementationGuide_ParameterCode::RuleBrokenLinks),
            "generate-xml" => Some(ImplementationGuide_ParameterCode::GenerateXml),
            "generate-json" => Some(ImplementationGuide_ParameterCode::GenerateJson),
            "generate-turtle" => Some(ImplementationGuide_ParameterCode::GenerateTurtle),
            "html-template" => Some(ImplementationGuide_ParameterCode::HtmlTemplate),
            _ => None,
        }
    }
}
