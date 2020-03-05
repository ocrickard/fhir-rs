#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Captures constraints on each element within the resource, profile, or extension.

#[derive(Debug)]
pub struct ElementDefinition_Mapping<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ElementDefinition_Mapping<'_> {
    pub fn new(value: &Value) -> ElementDefinition_Mapping {
        ElementDefinition_Mapping {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for comment
    pub fn _comment(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_comment") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for identity
    pub fn _identity(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_identity") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for map
    pub fn _map(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_map") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Comments that provide information about the mapping or its use.
    pub fn comment(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("comment") {
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

    /// An internal reference to the definition of a mapping.
    pub fn identity(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("identity") {
            return Some(string);
        }
        return None;
    }

    /// Identifies the computable language in which mapping.map is expressed.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// Expresses what part of the target specification corresponds to this element.
    pub fn map(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("map") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._comment() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._identity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._map() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.comment() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identity() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.map() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ElementDefinition_MappingBuilder {
    pub(crate) value: Value,
}

impl ElementDefinition_MappingBuilder {
    pub fn build(&self) -> ElementDefinition_Mapping {
        ElementDefinition_Mapping {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ElementDefinition_Mapping) -> ElementDefinition_MappingBuilder {
        ElementDefinition_MappingBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ElementDefinition_MappingBuilder {
        let mut __value: Value = json!({});
        return ElementDefinition_MappingBuilder { value: __value };
    }

    pub fn _comment<'a>(&'a mut self, val: Element) -> &'a mut ElementDefinition_MappingBuilder {
        self.value["_comment"] = json!(val.value);
        return self;
    }

    pub fn _identity<'a>(&'a mut self, val: Element) -> &'a mut ElementDefinition_MappingBuilder {
        self.value["_identity"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut ElementDefinition_MappingBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _map<'a>(&'a mut self, val: Element) -> &'a mut ElementDefinition_MappingBuilder {
        self.value["_map"] = json!(val.value);
        return self;
    }

    pub fn comment<'a>(&'a mut self, val: &str) -> &'a mut ElementDefinition_MappingBuilder {
        self.value["comment"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ElementDefinition_MappingBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ElementDefinition_MappingBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identity<'a>(&'a mut self, val: &str) -> &'a mut ElementDefinition_MappingBuilder {
        self.value["identity"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut ElementDefinition_MappingBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn map<'a>(&'a mut self, val: &str) -> &'a mut ElementDefinition_MappingBuilder {
        self.value["map"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ElementDefinition_MappingBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
