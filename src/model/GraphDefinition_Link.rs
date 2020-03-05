#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::GraphDefinition_Target::GraphDefinition_Target;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A formal computable definition of a graph of resources - that is, a coherent set
/// of resources that form a graph by following references. The Graph Definition
/// resource defines a set and makes rules about the set.

#[derive(Debug)]
pub struct GraphDefinition_Link<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl GraphDefinition_Link<'_> {
    pub fn new(value: &Value) -> GraphDefinition_Link {
        GraphDefinition_Link {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// Extensions for path
    pub fn _path(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_path") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for sliceName
    pub fn _slice_name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sliceName") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Information about why this link is of interest in this graph definition.
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

    /// Maximum occurrences for this link.
    pub fn max(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("max") {
            return Some(string);
        }
        return None;
    }

    /// Minimum occurrences for this link.
    pub fn min(&self) -> Option<i64> {
        if let Some(val) = self.value.get("min") {
            return Some(val.as_i64().unwrap());
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

    /// A FHIR expression that identifies one of FHIR References to other resources.
    pub fn path(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("path") {
            return Some(string);
        }
        return None;
    }

    /// Which slice (if profiled).
    pub fn slice_name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("sliceName") {
            return Some(string);
        }
        return None;
    }

    /// Potential target for the link.
    pub fn target(&self) -> Option<Vec<GraphDefinition_Target>> {
        if let Some(Value::Array(val)) = self.value.get("target") {
            return Some(
                val.into_iter()
                    .map(|e| GraphDefinition_Target {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
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
        if let Some(_val) = self._path() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._slice_name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.max() {}
        if let Some(_val) = self.min() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.path() {}
        if let Some(_val) = self.slice_name() {}
        if let Some(_val) = self.target() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct GraphDefinition_LinkBuilder {
    pub(crate) value: Value,
}

impl GraphDefinition_LinkBuilder {
    pub fn build(&self) -> GraphDefinition_Link {
        GraphDefinition_Link {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: GraphDefinition_Link) -> GraphDefinition_LinkBuilder {
        GraphDefinition_LinkBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> GraphDefinition_LinkBuilder {
        let mut __value: Value = json!({});
        return GraphDefinition_LinkBuilder { value: __value };
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut GraphDefinition_LinkBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _max<'a>(&'a mut self, val: Element) -> &'a mut GraphDefinition_LinkBuilder {
        self.value["_max"] = json!(val.value);
        return self;
    }

    pub fn _min<'a>(&'a mut self, val: Element) -> &'a mut GraphDefinition_LinkBuilder {
        self.value["_min"] = json!(val.value);
        return self;
    }

    pub fn _path<'a>(&'a mut self, val: Element) -> &'a mut GraphDefinition_LinkBuilder {
        self.value["_path"] = json!(val.value);
        return self;
    }

    pub fn _slice_name<'a>(&'a mut self, val: Element) -> &'a mut GraphDefinition_LinkBuilder {
        self.value["_sliceName"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut GraphDefinition_LinkBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut GraphDefinition_LinkBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut GraphDefinition_LinkBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn max<'a>(&'a mut self, val: &str) -> &'a mut GraphDefinition_LinkBuilder {
        self.value["max"] = json!(val);
        return self;
    }

    pub fn min<'a>(&'a mut self, val: i64) -> &'a mut GraphDefinition_LinkBuilder {
        self.value["min"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut GraphDefinition_LinkBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn path<'a>(&'a mut self, val: &str) -> &'a mut GraphDefinition_LinkBuilder {
        self.value["path"] = json!(val);
        return self;
    }

    pub fn slice_name<'a>(&'a mut self, val: &str) -> &'a mut GraphDefinition_LinkBuilder {
        self.value["sliceName"] = json!(val);
        return self;
    }

    pub fn target<'a>(
        &'a mut self,
        val: Vec<GraphDefinition_Target>,
    ) -> &'a mut GraphDefinition_LinkBuilder {
        self.value["target"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
