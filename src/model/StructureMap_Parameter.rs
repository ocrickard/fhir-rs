#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A Map of relationships between 2 structures that can be used to transform data.

#[derive(Debug)]
pub struct StructureMap_Parameter<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl StructureMap_Parameter<'_> {
    pub fn new(value: &Value) -> StructureMap_Parameter {
        StructureMap_Parameter {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for valueBoolean
    pub fn _value_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueBoolean") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueDecimal
    pub fn _value_decimal(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueDecimal") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueId
    pub fn _value_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueId") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueInteger
    pub fn _value_integer(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueInteger") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueString
    pub fn _value_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
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

    /// Parameter value - variable or literal.
    pub fn value_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("valueBoolean") {
            return Some(val.as_bool().unwrap());
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

    /// Parameter value - variable or literal.
    pub fn value_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueId") {
            return Some(string);
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
    pub fn value_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueString") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._value_boolean() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_decimal() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_id() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_integer() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.value_boolean() {}
        if let Some(_val) = self.value_decimal() {}
        if let Some(_val) = self.value_id() {}
        if let Some(_val) = self.value_integer() {}
        if let Some(_val) = self.value_string() {}
        return true;
    }
}

#[derive(Debug)]
pub struct StructureMap_ParameterBuilder {
    pub(crate) value: Value,
}

impl StructureMap_ParameterBuilder {
    pub fn build(&self) -> StructureMap_Parameter {
        StructureMap_Parameter {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: StructureMap_Parameter) -> StructureMap_ParameterBuilder {
        StructureMap_ParameterBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> StructureMap_ParameterBuilder {
        let mut __value: Value = json!({});
        return StructureMap_ParameterBuilder { value: __value };
    }

    pub fn _value_boolean<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_ParameterBuilder {
        self.value["_valueBoolean"] = json!(val.value);
        return self;
    }

    pub fn _value_decimal<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_ParameterBuilder {
        self.value["_valueDecimal"] = json!(val.value);
        return self;
    }

    pub fn _value_id<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_ParameterBuilder {
        self.value["_valueId"] = json!(val.value);
        return self;
    }

    pub fn _value_integer<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_ParameterBuilder {
        self.value["_valueInteger"] = json!(val.value);
        return self;
    }

    pub fn _value_string<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_ParameterBuilder {
        self.value["_valueString"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut StructureMap_ParameterBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_ParameterBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut StructureMap_ParameterBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn value_boolean<'a>(&'a mut self, val: bool) -> &'a mut StructureMap_ParameterBuilder {
        self.value["valueBoolean"] = json!(val);
        return self;
    }

    pub fn value_decimal<'a>(&'a mut self, val: f64) -> &'a mut StructureMap_ParameterBuilder {
        self.value["valueDecimal"] = json!(val);
        return self;
    }

    pub fn value_id<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_ParameterBuilder {
        self.value["valueId"] = json!(val);
        return self;
    }

    pub fn value_integer<'a>(&'a mut self, val: f64) -> &'a mut StructureMap_ParameterBuilder {
        self.value["valueInteger"] = json!(val);
        return self;
    }

    pub fn value_string<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_ParameterBuilder {
        self.value["valueString"] = json!(val);
        return self;
    }
}
