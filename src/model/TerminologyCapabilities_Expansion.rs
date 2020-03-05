#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::TerminologyCapabilities_Parameter::TerminologyCapabilities_Parameter;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A TerminologyCapabilities resource documents a set of capabilities (behaviors)
/// of a FHIR Terminology Server that may be used as a statement of actual server
/// functionality or a statement of required or desired server implementation.

#[derive(Debug)]
pub struct TerminologyCapabilities_Expansion<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl TerminologyCapabilities_Expansion<'_> {
    pub fn new(value: &Value) -> TerminologyCapabilities_Expansion {
        TerminologyCapabilities_Expansion {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for hierarchical
    pub fn _hierarchical(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_hierarchical") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for incomplete
    pub fn _incomplete(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_incomplete") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for paging
    pub fn _paging(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_paging") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for textFilter
    pub fn _text_filter(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_textFilter") {
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

    /// Whether the server can return nested value sets.
    pub fn hierarchical(&self) -> Option<bool> {
        if let Some(val) = self.value.get("hierarchical") {
            return Some(val.as_bool().unwrap());
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

    /// Allow request for incomplete expansions?
    pub fn incomplete(&self) -> Option<bool> {
        if let Some(val) = self.value.get("incomplete") {
            return Some(val.as_bool().unwrap());
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

    /// Whether the server supports paging on expansion.
    pub fn paging(&self) -> Option<bool> {
        if let Some(val) = self.value.get("paging") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Supported expansion parameter.
    pub fn parameter(&self) -> Option<Vec<TerminologyCapabilities_Parameter>> {
        if let Some(Value::Array(val)) = self.value.get("parameter") {
            return Some(
                val.into_iter()
                    .map(|e| TerminologyCapabilities_Parameter {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Documentation about text searching works.
    pub fn text_filter(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("textFilter") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._hierarchical() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._incomplete() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._paging() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._text_filter() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.hierarchical() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.incomplete() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.paging() {}
        if let Some(_val) = self.parameter() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text_filter() {}
        return true;
    }
}

#[derive(Debug)]
pub struct TerminologyCapabilities_ExpansionBuilder {
    pub(crate) value: Value,
}

impl TerminologyCapabilities_ExpansionBuilder {
    pub fn build(&self) -> TerminologyCapabilities_Expansion {
        TerminologyCapabilities_Expansion {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: TerminologyCapabilities_Expansion,
    ) -> TerminologyCapabilities_ExpansionBuilder {
        TerminologyCapabilities_ExpansionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> TerminologyCapabilities_ExpansionBuilder {
        let mut __value: Value = json!({});
        return TerminologyCapabilities_ExpansionBuilder { value: __value };
    }

    pub fn _hierarchical<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut TerminologyCapabilities_ExpansionBuilder {
        self.value["_hierarchical"] = json!(val.value);
        return self;
    }

    pub fn _incomplete<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut TerminologyCapabilities_ExpansionBuilder {
        self.value["_incomplete"] = json!(val.value);
        return self;
    }

    pub fn _paging<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut TerminologyCapabilities_ExpansionBuilder {
        self.value["_paging"] = json!(val.value);
        return self;
    }

    pub fn _text_filter<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut TerminologyCapabilities_ExpansionBuilder {
        self.value["_textFilter"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut TerminologyCapabilities_ExpansionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn hierarchical<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut TerminologyCapabilities_ExpansionBuilder {
        self.value["hierarchical"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut TerminologyCapabilities_ExpansionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn incomplete<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut TerminologyCapabilities_ExpansionBuilder {
        self.value["incomplete"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut TerminologyCapabilities_ExpansionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn paging<'a>(&'a mut self, val: bool) -> &'a mut TerminologyCapabilities_ExpansionBuilder {
        self.value["paging"] = json!(val);
        return self;
    }

    pub fn parameter<'a>(
        &'a mut self,
        val: Vec<TerminologyCapabilities_Parameter>,
    ) -> &'a mut TerminologyCapabilities_ExpansionBuilder {
        self.value["parameter"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text_filter<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut TerminologyCapabilities_ExpansionBuilder {
        self.value["textFilter"] = json!(val);
        return self;
    }
}
