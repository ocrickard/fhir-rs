#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A container for a collection of resources.

#[derive(Debug)]
pub struct Bundle_Link<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Bundle_Link<'_> {
    pub fn new(value: &Value) -> Bundle_Link {
        Bundle_Link {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for relation
    pub fn _relation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_relation") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for url
    pub fn _url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_url") {
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

    /// A name which details the functional use for this link - see
    /// [http://www.iana.org/assignments/link-relations/link-relations.xhtml#link-relati
    /// ons-1](http://www.iana.org/assignments/link-relations/link-relations.xhtml#link-
    /// relations-1).
    pub fn relation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("relation") {
            return Some(string);
        }
        return None;
    }

    /// The reference details for the link.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._relation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._url() {
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
        if let Some(_val) = self.relation() {}
        if let Some(_val) = self.url() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Bundle_LinkBuilder {
    pub(crate) value: Value,
}

impl Bundle_LinkBuilder {
    pub fn build(&self) -> Bundle_Link {
        Bundle_Link {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Bundle_Link) -> Bundle_LinkBuilder {
        Bundle_LinkBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Bundle_LinkBuilder {
        let mut __value: Value = json!({});
        return Bundle_LinkBuilder { value: __value };
    }

    pub fn _relation<'a>(&'a mut self, val: Element) -> &'a mut Bundle_LinkBuilder {
        self.value["_relation"] = json!(val.value);
        return self;
    }

    pub fn _url<'a>(&'a mut self, val: Element) -> &'a mut Bundle_LinkBuilder {
        self.value["_url"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Bundle_LinkBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Bundle_LinkBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Bundle_LinkBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn relation<'a>(&'a mut self, val: &str) -> &'a mut Bundle_LinkBuilder {
        self.value["relation"] = json!(val);
        return self;
    }

    pub fn url<'a>(&'a mut self, val: &str) -> &'a mut Bundle_LinkBuilder {
        self.value["url"] = json!(val);
        return self;
    }
}
