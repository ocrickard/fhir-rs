#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::StructureMap_Parameter::StructureMap_Parameter;
use serde_json::value::Value;

/// A Map of relationships between 2 structures that can be used to transform data.

#[derive(Debug)]
pub struct StructureMap_Target<'a> {
    pub value: &'a Value,
}

impl StructureMap_Target<'_> {
    /// How to interpret the context.
    pub fn context_type(&self) -> Option<StructureMap_TargetContextType> {
        if let Some(Value::String(val)) = self.value.get("contextType") {
            return Some(StructureMap_TargetContextType::from_string(&val).unwrap());
        }
        return None;
    }

    /// Field to create in the context.
    pub fn element(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("element") {
            return Some(string);
        }
        return None;
    }

    /// Internal rule reference for shared list items.
    pub fn list_rule_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("listRuleId") {
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

    /// Extensions for element
    pub fn _element(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_element") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Type or variable this rule applies to.
    pub fn context(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("context") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for listRuleId
    pub fn _list_rule_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_listRuleId") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// How the data is copied / created.
    pub fn transform(&self) -> Option<StructureMap_TargetTransform> {
        if let Some(Value::String(val)) = self.value.get("transform") {
            return Some(StructureMap_TargetTransform::from_string(&val).unwrap());
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

    /// Named context for field, if desired, and a field is specified.
    pub fn variable(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("variable") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for contextType
    pub fn _context_type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_contextType") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Parameters to the transform.
    pub fn parameter(&self) -> Option<Vec<StructureMap_Parameter>> {
        if let Some(Value::Array(val)) = self.value.get("parameter") {
            return Some(
                val.into_iter()
                    .map(|e| StructureMap_Parameter { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for context
    pub fn _context(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_context") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for transform
    pub fn _transform(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_transform") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for listMode
    pub fn _list_mode(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_listMode") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for variable
    pub fn _variable(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_variable") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.context_type() {}
        if let Some(_val) = self.element() {}
        if let Some(_val) = self.list_rule_id() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._element() {
            _val.validate();
        }
        if let Some(_val) = self.context() {}
        if let Some(_val) = self._list_rule_id() {
            _val.validate();
        }
        if let Some(_val) = self.transform() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.variable() {}
        if let Some(_val) = self._context_type() {
            _val.validate();
        }
        if let Some(_val) = self.parameter() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._context() {
            _val.validate();
        }
        if let Some(_val) = self._transform() {
            _val.validate();
        }
        if let Some(_val) = self._list_mode() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._variable() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum StructureMap_TargetContextType {
    FhirType,
    Variable,
}

impl StructureMap_TargetContextType {
    pub fn from_string(string: &str) -> Option<StructureMap_TargetContextType> {
        match string {
            "type" => Some(StructureMap_TargetContextType::FhirType),
            "variable" => Some(StructureMap_TargetContextType::Variable),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum StructureMap_TargetTransform {
    Create,
    Copy,
    Truncate,
    Escape,
    Cast,
    Append,
    Translate,
    Reference,
    DateOp,
    Uuid,
    Pointer,
    Evaluate,
    Cc,
    C,
    Qty,
    Id,
    Cp,
}

impl StructureMap_TargetTransform {
    pub fn from_string(string: &str) -> Option<StructureMap_TargetTransform> {
        match string {
            "create" => Some(StructureMap_TargetTransform::Create),
            "copy" => Some(StructureMap_TargetTransform::Copy),
            "truncate" => Some(StructureMap_TargetTransform::Truncate),
            "escape" => Some(StructureMap_TargetTransform::Escape),
            "cast" => Some(StructureMap_TargetTransform::Cast),
            "append" => Some(StructureMap_TargetTransform::Append),
            "translate" => Some(StructureMap_TargetTransform::Translate),
            "reference" => Some(StructureMap_TargetTransform::Reference),
            "dateOp" => Some(StructureMap_TargetTransform::DateOp),
            "uuid" => Some(StructureMap_TargetTransform::Uuid),
            "pointer" => Some(StructureMap_TargetTransform::Pointer),
            "evaluate" => Some(StructureMap_TargetTransform::Evaluate),
            "cc" => Some(StructureMap_TargetTransform::Cc),
            "c" => Some(StructureMap_TargetTransform::C),
            "qty" => Some(StructureMap_TargetTransform::Qty),
            "id" => Some(StructureMap_TargetTransform::Id),
            "cp" => Some(StructureMap_TargetTransform::Cp),
            _ => None,
        }
    }
}
