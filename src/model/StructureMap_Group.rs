#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::StructureMap_Input::StructureMap_Input;
use crate::model::StructureMap_Rule::StructureMap_Rule;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A Map of relationships between 2 structures that can be used to transform data.

#[derive(Debug)]
pub struct StructureMap_Group<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl StructureMap_Group<'_> {
    pub fn new(value: &Value) -> StructureMap_Group {
        StructureMap_Group {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for documentation
    pub fn _documentation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_documentation") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for extends
    pub fn _extends(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_extends") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for typeMode
    pub fn _type_mode(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_typeMode") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
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

    /// Another group that this group adds rules to.
    pub fn extends(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("extends") {
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

    /// A name assigned to an instance of data. The instance must be provided when the
    /// mapping is invoked.
    pub fn input(&self) -> Vec<StructureMap_Input> {
        self.value
            .get("input")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| StructureMap_Input {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
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

    /// A unique name for the group for the convenience of human readers.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Transform Rule from source to target.
    pub fn rule(&self) -> Vec<StructureMap_Rule> {
        self.value
            .get("rule")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| StructureMap_Rule {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
    }

    /// If this is the default rule set to apply for the source type or this combination
    /// of types.
    pub fn type_mode(&self) -> Option<StructureMap_GroupTypeMode> {
        if let Some(Value::String(val)) = self.value.get("typeMode") {
            return Some(StructureMap_GroupTypeMode::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._documentation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._extends() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type_mode() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.documentation() {}
        if let Some(_val) = self.extends() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if !self
            .input()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.name() {}
        if !self
            .rule()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
        if let Some(_val) = self.type_mode() {}
        return true;
    }
}

#[derive(Debug)]
pub struct StructureMap_GroupBuilder {
    pub(crate) value: Value,
}

impl StructureMap_GroupBuilder {
    pub fn build(&self) -> StructureMap_Group {
        StructureMap_Group {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: StructureMap_Group) -> StructureMap_GroupBuilder {
        StructureMap_GroupBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        input: Vec<StructureMap_Input>,
        rule: Vec<StructureMap_Rule>,
    ) -> StructureMap_GroupBuilder {
        let mut __value: Value = json!({});
        __value["input"] = json!(input.into_iter().map(|e| e.value).collect::<Vec<_>>());
        __value["rule"] = json!(rule.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return StructureMap_GroupBuilder { value: __value };
    }

    pub fn _documentation<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_GroupBuilder {
        self.value["_documentation"] = json!(val.value);
        return self;
    }

    pub fn _extends<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_GroupBuilder {
        self.value["_extends"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_GroupBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _type_mode<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_GroupBuilder {
        self.value["_typeMode"] = json!(val.value);
        return self;
    }

    pub fn documentation<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_GroupBuilder {
        self.value["documentation"] = json!(val);
        return self;
    }

    pub fn extends<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_GroupBuilder {
        self.value["extends"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut StructureMap_GroupBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_GroupBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut StructureMap_GroupBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_GroupBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn type_mode<'a>(
        &'a mut self,
        val: StructureMap_GroupTypeMode,
    ) -> &'a mut StructureMap_GroupBuilder {
        self.value["typeMode"] = json!(val.to_string());
        return self;
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

    pub fn to_string(&self) -> String {
        match self {
            StructureMap_GroupTypeMode::None => "none".to_string(),
            StructureMap_GroupTypeMode::Types => "types".to_string(),
            StructureMap_GroupTypeMode::TypeAndTypes => "type-and-types".to_string(),
        }
    }
}
