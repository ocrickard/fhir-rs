#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The CodeSystem resource is used to declare the existence of and describe a code
/// system or code system supplement and its key properties, and optionally define a
/// part or all of its content.

#[derive(Debug)]
pub struct CodeSystem_Filter<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl CodeSystem_Filter<'_> {
    pub fn new(value: &Value) -> CodeSystem_Filter {
        CodeSystem_Filter {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for code
    pub fn _code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_code") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for operator
    pub fn _operator(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_operator") {
            return Some(
                val.into_iter()
                    .map(|e| Element {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// The code that identifies this filter when it is used as a filter in
    /// [[[ValueSet]]].compose.include.filter.
    pub fn code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("code") {
            return Some(string);
        }
        return None;
    }

    /// A description of how or why the filter is used.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
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

    /// A list of operators that can be used with the filter.
    pub fn operator(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("operator") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A description of what the value for the filter should be.
    pub fn value(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("value") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._operator() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._value() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.code() {}
        if let Some(_val) = self.description() {}
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
        if let Some(_val) = self.operator() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.value() {}
        return true;
    }
}

#[derive(Debug)]
pub struct CodeSystem_FilterBuilder {
    pub(crate) value: Value,
}

impl CodeSystem_FilterBuilder {
    pub fn build(&self) -> CodeSystem_Filter {
        CodeSystem_Filter {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: CodeSystem_Filter) -> CodeSystem_FilterBuilder {
        CodeSystem_FilterBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> CodeSystem_FilterBuilder {
        let mut __value: Value = json!({});
        return CodeSystem_FilterBuilder { value: __value };
    }

    pub fn _code<'a>(&'a mut self, val: Element) -> &'a mut CodeSystem_FilterBuilder {
        self.value["_code"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut CodeSystem_FilterBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _operator<'a>(&'a mut self, val: Vec<Element>) -> &'a mut CodeSystem_FilterBuilder {
        self.value["_operator"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _value<'a>(&'a mut self, val: Element) -> &'a mut CodeSystem_FilterBuilder {
        self.value["_value"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(&'a mut self, val: &str) -> &'a mut CodeSystem_FilterBuilder {
        self.value["code"] = json!(val);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut CodeSystem_FilterBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut CodeSystem_FilterBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut CodeSystem_FilterBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CodeSystem_FilterBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn operator<'a>(&'a mut self, val: Vec<&str>) -> &'a mut CodeSystem_FilterBuilder {
        self.value["operator"] = json!(val);
        return self;
    }

    pub fn value<'a>(&'a mut self, val: &str) -> &'a mut CodeSystem_FilterBuilder {
        self.value["value"] = json!(val);
        return self;
    }
}
