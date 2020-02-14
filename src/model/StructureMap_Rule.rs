#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::StructureMap_Dependent::StructureMap_Dependent;
use crate::model::StructureMap_Source::StructureMap_Source;
use crate::model::StructureMap_Target::StructureMap_Target;
use serde_json::value::Value;

/// A Map of relationships between 2 structures that can be used to transform data.

#[derive(Debug)]
pub struct StructureMap_Rule<'a> {
    pub value: &'a Value,
}

impl StructureMap_Rule<'_> {
    /// Content to create because of this mapping rule.
    pub fn target(&self) -> Option<Vec<StructureMap_Target>> {
        if let Some(Value::Array(val)) = self.value.get("target") {
            return Some(
                val.into_iter()
                    .map(|e| StructureMap_Target { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Documentation for this instance of data.
    pub fn documentation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("documentation") {
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

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Name of the rule for internal references.
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

    /// Source inputs to the mapping.
    pub fn source(&self) -> Vec<StructureMap_Source> {
        self.value
            .get("source")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| StructureMap_Source { value: e })
            .collect::<Vec<_>>()
    }

    /// Rules contained in this rule.
    pub fn rule(&self) -> Option<Vec<StructureMap_Rule>> {
        if let Some(Value::Array(val)) = self.value.get("rule") {
            return Some(
                val.into_iter()
                    .map(|e| StructureMap_Rule { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Which other rules to apply in the context of this rule.
    pub fn dependent(&self) -> Option<Vec<StructureMap_Dependent>> {
        if let Some(Value::Array(val)) = self.value.get("dependent") {
            return Some(
                val.into_iter()
                    .map(|e| StructureMap_Dependent { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.target() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.documentation() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._documentation() {
            _val.validate();
        }
        if let Some(_val) = self._name() {
            _val.validate();
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.source().into_iter().for_each(|e| {
            e.validate();
        });
        if let Some(_val) = self.rule() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.dependent() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
