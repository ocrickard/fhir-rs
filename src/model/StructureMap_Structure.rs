#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// A Map of relationships between 2 structures that can be used to transform data.

#[derive(Debug)]
pub struct StructureMap_Structure<'a> {
    pub value: &'a Value,
}

impl StructureMap_Structure<'_> {
    /// Extensions for alias
    pub fn _alias(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_alias") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// How the referenced structure is used in this mapping.
    pub fn mode(&self) -> Option<StructureMap_StructureMode> {
        if let Some(Value::String(val)) = self.value.get("mode") {
            return Some(StructureMap_StructureMode::from_string(&val).unwrap());
        }
        return None;
    }

    /// The canonical reference to the structure.
    pub fn url(&self) -> &str {
        self.value.get("url").unwrap().as_str().unwrap()
    }

    /// Documentation that describes how the structure is used in the mapping.
    pub fn documentation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("documentation") {
            return Some(string);
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

    /// The name used for this type in the map.
    pub fn alias(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("alias") {
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._alias() {
            _val.validate();
        }
        if let Some(_val) = self.mode() {}
        let _ = self.url();
        if let Some(_val) = self.documentation() {}
        if let Some(_val) = self._documentation() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.alias() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._mode() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum StructureMap_StructureMode {
    Source,
    Queried,
    Target,
    Produced,
}

impl StructureMap_StructureMode {
    pub fn from_string(string: &str) -> Option<StructureMap_StructureMode> {
        match string {
            "source" => Some(StructureMap_StructureMode::Source),
            "queried" => Some(StructureMap_StructureMode::Queried),
            "target" => Some(StructureMap_StructureMode::Target),
            "produced" => Some(StructureMap_StructureMode::Produced),
            _ => None,
        }
    }
}
