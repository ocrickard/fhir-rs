#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::StructureMap_Input::StructureMap_Input;
use crate::model::StructureMap_Rule::StructureMap_Rule;
use serde_json::value::Value;

/// A Map of relationships between 2 structures that can be used to transform data.

#[derive(Debug)]
pub struct StructureMap_Group<'a> {
    pub value: &'a Value,
}

impl StructureMap_Group<'_> {
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

    /// Another group that this group adds rules to.
    pub fn extends(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("extends") {
            return Some(string);
        }
        return None;
    }

    /// A unique name for the group for the convenience of human readers.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
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

    /// If this is the default rule set to apply for the source type or this combination
    /// of types.
    pub fn type_mode(&self) -> Option<StructureMap_GroupTypeMode> {
        if let Some(Value::String(val)) = self.value.get("typeMode") {
            return Some(StructureMap_GroupTypeMode::from_string(&val).unwrap());
        }
        return None;
    }

    /// Extensions for extends
    pub fn _extends(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_extends") {
            return Some(Element { value: val });
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

    /// Extensions for typeMode
    pub fn _type_mode(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_typeMode") {
            return Some(Element { value: val });
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

    /// Additional supporting documentation that explains the purpose of the group and
    /// the types of mappings within it.
    pub fn documentation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("documentation") {
            return Some(string);
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

    /// A name assigned to an instance of data. The instance must be provided when the
    /// mapping is invoked.
    pub fn input(&self) -> Vec<StructureMap_Input> {
        self.value
            .get("input")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| StructureMap_Input { value: e })
            .collect::<Vec<_>>()
    }

    /// Transform Rule from source to target.
    pub fn rule(&self) -> Vec<StructureMap_Rule> {
        self.value
            .get("rule")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| StructureMap_Rule { value: e })
            .collect::<Vec<_>>()
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extends() {}
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.type_mode() {}
        if let Some(_val) = self._extends() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._type_mode() {
            _val.validate();
        }
        if let Some(_val) = self._documentation() {
            _val.validate();
        }
        if let Some(_val) = self.documentation() {}
        if let Some(_val) = self._name() {
            _val.validate();
        }
        let _ = self.input().into_iter().for_each(|e| {
            e.validate();
        });
        let _ = self.rule().into_iter().for_each(|e| {
            e.validate();
        });
        return true;
    }
}

#[derive(Debug)]
pub enum StructureMap_GroupTypeMode {
    None,
    Types,
    TypeAndTypes,
}

impl StructureMap_GroupTypeMode {
    pub fn from_string(string: &str) -> Option<StructureMap_GroupTypeMode> {
        match string {
            "none" => Some(StructureMap_GroupTypeMode::None),
            "types" => Some(StructureMap_GroupTypeMode::Types),
            "type-and-types" => Some(StructureMap_GroupTypeMode::TypeAndTypes),
            _ => None,
        }
    }
}
