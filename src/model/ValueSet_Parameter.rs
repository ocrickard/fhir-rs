#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [[[CodeSystem]]] definitions and their use in [coded
/// elements](terminologies.html).

#[derive(Debug)]
pub struct ValueSet_Parameter<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ValueSet_Parameter<'_> {
    pub fn new(value: &Value) -> ValueSet_Parameter {
        ValueSet_Parameter {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// Extensions for valueBoolean
    pub fn _value_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueBoolean") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueCode
    pub fn _value_code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueCode") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueDateTime
    pub fn _value_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueDateTime") {
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

    /// Extensions for valueUri
    pub fn _value_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueUri") {
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

    /// Name of the input parameter to the $expand operation; may be a server-assigned
    /// name for additional default or other server-supplied parameters used to control
    /// the expansion process.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// The value of the parameter.
    pub fn value_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("valueBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The value of the parameter.
    pub fn value_code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueCode") {
            return Some(string);
        }
        return None;
    }

    /// The value of the parameter.
    pub fn value_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueDateTime") {
            return Some(string);
        }
        return None;
    }

    /// The value of the parameter.
    pub fn value_decimal(&self) -> Option<f64> {
        if let Some(val) = self.value.get("valueDecimal") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The value of the parameter.
    pub fn value_integer(&self) -> Option<f64> {
        if let Some(val) = self.value.get("valueInteger") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The value of the parameter.
    pub fn value_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueString") {
            return Some(string);
        }
        return None;
    }

    /// The value of the parameter.
    pub fn value_uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueUri") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_boolean() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_decimal() {
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
        if let Some(_val) = self._value_uri() {
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
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.value_boolean() {}
        if let Some(_val) = self.value_code() {}
        if let Some(_val) = self.value_date_time() {}
        if let Some(_val) = self.value_decimal() {}
        if let Some(_val) = self.value_integer() {}
        if let Some(_val) = self.value_string() {}
        if let Some(_val) = self.value_uri() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ValueSet_ParameterBuilder {
    pub(crate) value: Value,
}

impl ValueSet_ParameterBuilder {
    pub fn build(&self) -> ValueSet_Parameter {
        ValueSet_Parameter {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ValueSet_Parameter) -> ValueSet_ParameterBuilder {
        ValueSet_ParameterBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ValueSet_ParameterBuilder {
        let mut __value: Value = json!({});
        return ValueSet_ParameterBuilder { value: __value };
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut ValueSet_ParameterBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _value_boolean<'a>(&'a mut self, val: Element) -> &'a mut ValueSet_ParameterBuilder {
        self.value["_valueBoolean"] = json!(val.value);
        return self;
    }

    pub fn _value_code<'a>(&'a mut self, val: Element) -> &'a mut ValueSet_ParameterBuilder {
        self.value["_valueCode"] = json!(val.value);
        return self;
    }

    pub fn _value_date_time<'a>(&'a mut self, val: Element) -> &'a mut ValueSet_ParameterBuilder {
        self.value["_valueDateTime"] = json!(val.value);
        return self;
    }

    pub fn _value_decimal<'a>(&'a mut self, val: Element) -> &'a mut ValueSet_ParameterBuilder {
        self.value["_valueDecimal"] = json!(val.value);
        return self;
    }

    pub fn _value_integer<'a>(&'a mut self, val: Element) -> &'a mut ValueSet_ParameterBuilder {
        self.value["_valueInteger"] = json!(val.value);
        return self;
    }

    pub fn _value_string<'a>(&'a mut self, val: Element) -> &'a mut ValueSet_ParameterBuilder {
        self.value["_valueString"] = json!(val.value);
        return self;
    }

    pub fn _value_uri<'a>(&'a mut self, val: Element) -> &'a mut ValueSet_ParameterBuilder {
        self.value["_valueUri"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ValueSet_ParameterBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ValueSet_ParameterBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ValueSet_ParameterBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut ValueSet_ParameterBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn value_boolean<'a>(&'a mut self, val: bool) -> &'a mut ValueSet_ParameterBuilder {
        self.value["valueBoolean"] = json!(val);
        return self;
    }

    pub fn value_code<'a>(&'a mut self, val: &str) -> &'a mut ValueSet_ParameterBuilder {
        self.value["valueCode"] = json!(val);
        return self;
    }

    pub fn value_date_time<'a>(&'a mut self, val: &str) -> &'a mut ValueSet_ParameterBuilder {
        self.value["valueDateTime"] = json!(val);
        return self;
    }

    pub fn value_decimal<'a>(&'a mut self, val: f64) -> &'a mut ValueSet_ParameterBuilder {
        self.value["valueDecimal"] = json!(val);
        return self;
    }

    pub fn value_integer<'a>(&'a mut self, val: f64) -> &'a mut ValueSet_ParameterBuilder {
        self.value["valueInteger"] = json!(val);
        return self;
    }

    pub fn value_string<'a>(&'a mut self, val: &str) -> &'a mut ValueSet_ParameterBuilder {
        self.value["valueString"] = json!(val);
        return self;
    }

    pub fn value_uri<'a>(&'a mut self, val: &str) -> &'a mut ValueSet_ParameterBuilder {
        self.value["valueUri"] = json!(val);
        return self;
    }
}
