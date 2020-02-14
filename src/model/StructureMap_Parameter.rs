#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// A Map of relationships between 2 structures that can be used to transform data.

#[derive(Debug)]
pub struct StructureMap_Parameter<'a> {
    pub value: &'a Value,
}

impl StructureMap_Parameter<'_> {
    /// Extensions for valueId
    pub fn _value_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueId") {
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

    /// Parameter value - variable or literal.
    pub fn value_integer(&self) -> Option<f64> {
        if let Some(val) = self.value.get("valueInteger") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Parameter value - variable or literal.
    pub fn value_decimal(&self) -> Option<f64> {
        if let Some(val) = self.value.get("valueDecimal") {
            return Some(val.as_f64().unwrap());
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

    /// Extensions for valueString
    pub fn _value_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueString") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Parameter value - variable or literal.
    pub fn value_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueString") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for valueBoolean
    pub fn _value_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueBoolean") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Parameter value - variable or literal.
    pub fn value_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueId") {
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

    /// Extensions for valueInteger
    pub fn _value_integer(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueInteger") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for valueDecimal
    pub fn _value_decimal(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueDecimal") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Parameter value - variable or literal.
    pub fn value_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("valueBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._value_id() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.value_integer() {}
        if let Some(_val) = self.value_decimal() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._value_string() {
            _val.validate();
        }
        if let Some(_val) = self.value_string() {}
        if let Some(_val) = self._value_boolean() {
            _val.validate();
        }
        if let Some(_val) = self.value_id() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._value_integer() {
            _val.validate();
        }
        if let Some(_val) = self._value_decimal() {
            _val.validate();
        }
        if let Some(_val) = self.value_boolean() {}
        return true;
    }
}
