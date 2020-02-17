#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The parameters to the module. This collection specifies both the input and
/// output parameters. Input parameters are provided by the caller as part of the
/// $evaluate operation. Output parameters are included in the GuidanceResponse.

#[derive(Debug)]
pub struct ParameterDefinition<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ParameterDefinition<'_> {
    pub fn new(value: &Value) -> ParameterDefinition {
        ParameterDefinition {
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

    /// Extensions for max
    pub fn _max(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_max") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for min
    pub fn _min(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_min") {
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

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for use
    pub fn _use(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_use") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A brief discussion of what the parameter is for and how it is used by the
    /// module.
    pub fn documentation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("documentation") {
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

    /// The maximum number of times this element is permitted to appear in the request
    /// or response.
    pub fn max(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("max") {
            return Some(string);
        }
        return None;
    }

    /// The minimum number of times this parameter SHALL appear in the request or
    /// response.
    pub fn min(&self) -> Option<i64> {
        if let Some(val) = self.value.get("min") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// The name of the parameter used to allow access to the value of the parameter in
    /// evaluation contexts.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// If specified, this indicates a profile that the input data must conform to, or
    /// that the output data will conform to.
    pub fn profile(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("profile") {
            return Some(string);
        }
        return None;
    }

    /// The type of the parameter.
    pub fn fhir_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("type") {
            return Some(string);
        }
        return None;
    }

    /// Whether the parameter is input or output for the module.
    pub fn fhir_use(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("use") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._documentation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._max() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._min() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._use() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.documentation() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.max() {}
        if let Some(_val) = self.min() {}
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.profile() {}
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.fhir_use() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ParameterDefinitionBuilder {
    pub(crate) value: Value,
}

impl ParameterDefinitionBuilder {
    pub fn build(&self) -> ParameterDefinition {
        ParameterDefinition {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ParameterDefinition) -> ParameterDefinitionBuilder {
        ParameterDefinitionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ParameterDefinitionBuilder {
        let mut __value: Value = json!({});
        return ParameterDefinitionBuilder { value: __value };
    }

    pub fn _documentation<'a>(&'a mut self, val: Element) -> &'a mut ParameterDefinitionBuilder {
        self.value["_documentation"] = json!(val.value);
        return self;
    }

    pub fn _max<'a>(&'a mut self, val: Element) -> &'a mut ParameterDefinitionBuilder {
        self.value["_max"] = json!(val.value);
        return self;
    }

    pub fn _min<'a>(&'a mut self, val: Element) -> &'a mut ParameterDefinitionBuilder {
        self.value["_min"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut ParameterDefinitionBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut ParameterDefinitionBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn _use<'a>(&'a mut self, val: Element) -> &'a mut ParameterDefinitionBuilder {
        self.value["_use"] = json!(val.value);
        return self;
    }

    pub fn documentation<'a>(&'a mut self, val: &str) -> &'a mut ParameterDefinitionBuilder {
        self.value["documentation"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ParameterDefinitionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ParameterDefinitionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn max<'a>(&'a mut self, val: &str) -> &'a mut ParameterDefinitionBuilder {
        self.value["max"] = json!(val);
        return self;
    }

    pub fn min<'a>(&'a mut self, val: i64) -> &'a mut ParameterDefinitionBuilder {
        self.value["min"] = json!(val);
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut ParameterDefinitionBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn profile<'a>(&'a mut self, val: &str) -> &'a mut ParameterDefinitionBuilder {
        self.value["profile"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: &str) -> &'a mut ParameterDefinitionBuilder {
        self.value["type"] = json!(val);
        return self;
    }

    pub fn fhir_use<'a>(&'a mut self, val: &str) -> &'a mut ParameterDefinitionBuilder {
        self.value["use"] = json!(val);
        return self;
    }
}
