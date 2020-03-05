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
pub struct ValueSet_Filter<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ValueSet_Filter<'_> {
    pub fn new(value: &Value) -> ValueSet_Filter {
        ValueSet_Filter {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for op
    pub fn _op(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_op") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for property
    pub fn _property(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_property") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for value
    pub fn _value(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_value") {
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

    /// The kind of operation to perform as a part of the filter criteria.
    pub fn op(&self) -> Option<ValueSet_FilterOp> {
        if let Some(Value::String(val)) = self.value.get("op") {
            return Some(ValueSet_FilterOp::from_string(&val).unwrap());
        }
        return None;
    }

    /// A code that identifies a property or a filter defined in the code system.
    pub fn property(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("property") {
            return Some(string);
        }
        return None;
    }

    /// The match value may be either a code defined by the system, or a string value,
    /// which is a regex match on the literal string of the property value  (if the
    /// filter represents a property defined in CodeSystem) or of the system filter
    /// value (if the filter represents a filter defined in CodeSystem) when the
    /// operation is 'regex', or one of the values (true and false), when the operation
    /// is 'exists'.
    pub fn value(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("value") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._op() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._property() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value() {
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
        if let Some(_val) = self.op() {}
        if let Some(_val) = self.property() {}
        if let Some(_val) = self.value() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ValueSet_FilterBuilder {
    pub(crate) value: Value,
}

impl ValueSet_FilterBuilder {
    pub fn build(&self) -> ValueSet_Filter {
        ValueSet_Filter {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ValueSet_Filter) -> ValueSet_FilterBuilder {
        ValueSet_FilterBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ValueSet_FilterBuilder {
        let mut __value: Value = json!({});
        return ValueSet_FilterBuilder { value: __value };
    }

    pub fn _op<'a>(&'a mut self, val: Element) -> &'a mut ValueSet_FilterBuilder {
        self.value["_op"] = json!(val.value);
        return self;
    }

    pub fn _property<'a>(&'a mut self, val: Element) -> &'a mut ValueSet_FilterBuilder {
        self.value["_property"] = json!(val.value);
        return self;
    }

    pub fn _value<'a>(&'a mut self, val: Element) -> &'a mut ValueSet_FilterBuilder {
        self.value["_value"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ValueSet_FilterBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ValueSet_FilterBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ValueSet_FilterBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn op<'a>(&'a mut self, val: ValueSet_FilterOp) -> &'a mut ValueSet_FilterBuilder {
        self.value["op"] = json!(val.to_string());
        return self;
    }

    pub fn property<'a>(&'a mut self, val: &str) -> &'a mut ValueSet_FilterBuilder {
        self.value["property"] = json!(val);
        return self;
    }

    pub fn value<'a>(&'a mut self, val: &str) -> &'a mut ValueSet_FilterBuilder {
        self.value["value"] = json!(val);
        return self;
    }
}

#[derive(Debug)]
pub enum ValueSet_FilterOp {
    Equal,
    IsA,
    DescendentOf,
    IsNotA,
    Regex,
    In,
    NotIn,
    Generalizes,
    Exists,
}

impl ValueSet_FilterOp {
    pub fn from_string(string: &str) -> Option<ValueSet_FilterOp> {
        match string {
            "=" => Some(ValueSet_FilterOp::Equal),
            "is-a" => Some(ValueSet_FilterOp::IsA),
            "descendent-of" => Some(ValueSet_FilterOp::DescendentOf),
            "is-not-a" => Some(ValueSet_FilterOp::IsNotA),
            "regex" => Some(ValueSet_FilterOp::Regex),
            "in" => Some(ValueSet_FilterOp::In),
            "not-in" => Some(ValueSet_FilterOp::NotIn),
            "generalizes" => Some(ValueSet_FilterOp::Generalizes),
            "exists" => Some(ValueSet_FilterOp::Exists),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ValueSet_FilterOp::Equal => "=".to_string(),
            ValueSet_FilterOp::IsA => "is-a".to_string(),
            ValueSet_FilterOp::DescendentOf => "descendent-of".to_string(),
            ValueSet_FilterOp::IsNotA => "is-not-a".to_string(),
            ValueSet_FilterOp::Regex => "regex".to_string(),
            ValueSet_FilterOp::In => "in".to_string(),
            ValueSet_FilterOp::NotIn => "not-in".to_string(),
            ValueSet_FilterOp::Generalizes => "generalizes".to_string(),
            ValueSet_FilterOp::Exists => "exists".to_string(),
        }
    }
}
