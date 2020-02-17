#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A record of a healthcare consumer’s  choices, which permits or denies identified
/// recipient(s) or recipient role(s) to perform one or more actions within a given
/// policy context, for specific purposes and periods of time.

#[derive(Debug)]
pub struct Consent_Policy<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Consent_Policy<'_> {
    pub fn new(value: &Value) -> Consent_Policy {
        Consent_Policy {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for authority
    pub fn _authority(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_authority") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for uri
    pub fn _uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_uri") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Entity or Organization having regulatory jurisdiction or accountability for
    /// enforcing policies pertaining to Consent Directives.
    pub fn authority(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("authority") {
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

    /// The references to the policies that are included in this consent scope. Policies
    /// may be organizational, but are often defined jurisdictionally, or in law.
    pub fn uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("uri") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._authority() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._uri() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.authority() {}
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
        if let Some(_val) = self.uri() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Consent_PolicyBuilder {
    pub(crate) value: Value,
}

impl Consent_PolicyBuilder {
    pub fn build(&self) -> Consent_Policy {
        Consent_Policy {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Consent_Policy) -> Consent_PolicyBuilder {
        Consent_PolicyBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Consent_PolicyBuilder {
        let mut __value: Value = json!({});
        return Consent_PolicyBuilder { value: __value };
    }

    pub fn _authority<'a>(&'a mut self, val: Element) -> &'a mut Consent_PolicyBuilder {
        self.value["_authority"] = json!(val.value);
        return self;
    }

    pub fn _uri<'a>(&'a mut self, val: Element) -> &'a mut Consent_PolicyBuilder {
        self.value["_uri"] = json!(val.value);
        return self;
    }

    pub fn authority<'a>(&'a mut self, val: &str) -> &'a mut Consent_PolicyBuilder {
        self.value["authority"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Consent_PolicyBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Consent_PolicyBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Consent_PolicyBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn uri<'a>(&'a mut self, val: &str) -> &'a mut Consent_PolicyBuilder {
        self.value["uri"] = json!(val);
        return self;
    }
}
