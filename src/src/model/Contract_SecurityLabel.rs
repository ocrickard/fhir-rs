#![allow(unused_imports, non_camel_case_types)]

use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.

#[derive(Debug)]
pub struct Contract_SecurityLabel<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Contract_SecurityLabel<'_> {
    pub fn new(value: &Value) -> Contract_SecurityLabel {
        Contract_SecurityLabel {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for number
    pub fn _number(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_number") {
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

    /// Security label privacy tag that species the applicable privacy and security
    /// policies governing this term and/or term elements.
    pub fn category(&self) -> Option<Vec<Coding>> {
        if let Some(Value::Array(val)) = self.value.get("category") {
            return Some(
                val.into_iter()
                    .map(|e| Coding {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Security label privacy tag that species the level of confidentiality protection
    /// required for this term and/or term elements.
    pub fn classification(&self) -> Coding {
        Coding {
            value: Cow::Borrowed(&self.value["classification"]),
        }
    }

    /// Security label privacy tag that species the manner in which term and/or term
    /// elements are to be protected.
    pub fn control(&self) -> Option<Vec<Coding>> {
        if let Some(Value::Array(val)) = self.value.get("control") {
            return Some(
                val.into_iter()
                    .map(|e| Coding {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// Number used to link this term or term element to the applicable Security Label.
    pub fn number(&self) -> Option<Vec<u64>> {
        if let Some(Value::Array(val)) = self.value.get("number") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_u64().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._number() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.category() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.classification().validate() {
            return false;
        }
        if let Some(_val) = self.control() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
        if let Some(_val) = self.number() {
            _val.into_iter().for_each(|_e| {});
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Contract_SecurityLabelBuilder {
    pub(crate) value: Value,
}

impl Contract_SecurityLabelBuilder {
    pub fn build(&self) -> Contract_SecurityLabel {
        Contract_SecurityLabel {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Contract_SecurityLabel) -> Contract_SecurityLabelBuilder {
        Contract_SecurityLabelBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(classification: Coding) -> Contract_SecurityLabelBuilder {
        let mut __value: Value = json!({});
        __value["classification"] = json!(classification.value);
        return Contract_SecurityLabelBuilder { value: __value };
    }

    pub fn _number<'a>(&'a mut self, val: Vec<Element>) -> &'a mut Contract_SecurityLabelBuilder {
        self.value["_number"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn category<'a>(&'a mut self, val: Vec<Coding>) -> &'a mut Contract_SecurityLabelBuilder {
        self.value["category"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn control<'a>(&'a mut self, val: Vec<Coding>) -> &'a mut Contract_SecurityLabelBuilder {
        self.value["control"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Contract_SecurityLabelBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Contract_SecurityLabelBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Contract_SecurityLabelBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn number<'a>(&'a mut self, val: Vec<u64>) -> &'a mut Contract_SecurityLabelBuilder {
        self.value["number"] = json!(val);
        return self;
    }
}
