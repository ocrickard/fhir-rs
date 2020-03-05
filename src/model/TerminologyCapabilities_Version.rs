#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::TerminologyCapabilities_Filter::TerminologyCapabilities_Filter;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A TerminologyCapabilities resource documents a set of capabilities (behaviors)
/// of a FHIR Terminology Server that may be used as a statement of actual server
/// functionality or a statement of required or desired server implementation.

#[derive(Debug)]
pub struct TerminologyCapabilities_Version<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl TerminologyCapabilities_Version<'_> {
    pub fn new(value: &Value) -> TerminologyCapabilities_Version {
        TerminologyCapabilities_Version {
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

    /// Extensions for compositional
    pub fn _compositional(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_compositional") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for isDefault
    pub fn _is_default(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_isDefault") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_language") {
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

    /// Extensions for property
    pub fn _property(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_property") {
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

    /// For version-less code systems, there should be a single version with no
    /// identifier.
    pub fn code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("code") {
            return Some(string);
        }
        return None;
    }

    /// If the compositional grammar defined by the code system is supported.
    pub fn compositional(&self) -> Option<bool> {
        if let Some(val) = self.value.get("compositional") {
            return Some(val.as_bool().unwrap());
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

    /// Filter Properties supported.
    pub fn filter(&self) -> Option<Vec<TerminologyCapabilities_Filter>> {
        if let Some(Value::Array(val)) = self.value.get("filter") {
            return Some(
                val.into_iter()
                    .map(|e| TerminologyCapabilities_Filter {
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

    /// If this is the default version for this code system.
    pub fn is_default(&self) -> Option<bool> {
        if let Some(val) = self.value.get("isDefault") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Language Displays supported.
    pub fn language(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("language") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
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

    /// Properties supported for $lookup.
    pub fn property(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("property") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._compositional() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._is_default() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._property() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.code() {}
        if let Some(_val) = self.compositional() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.filter() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.is_default() {}
        if let Some(_val) = self.language() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.property() {
            _val.into_iter().for_each(|_e| {});
        }
        return true;
    }
}

#[derive(Debug)]
pub struct TerminologyCapabilities_VersionBuilder {
    pub(crate) value: Value,
}

impl TerminologyCapabilities_VersionBuilder {
    pub fn build(&self) -> TerminologyCapabilities_Version {
        TerminologyCapabilities_Version {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: TerminologyCapabilities_Version,
    ) -> TerminologyCapabilities_VersionBuilder {
        TerminologyCapabilities_VersionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> TerminologyCapabilities_VersionBuilder {
        let mut __value: Value = json!({});
        return TerminologyCapabilities_VersionBuilder { value: __value };
    }

    pub fn _code<'a>(&'a mut self, val: Element) -> &'a mut TerminologyCapabilities_VersionBuilder {
        self.value["_code"] = json!(val.value);
        return self;
    }

    pub fn _compositional<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut TerminologyCapabilities_VersionBuilder {
        self.value["_compositional"] = json!(val.value);
        return self;
    }

    pub fn _is_default<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut TerminologyCapabilities_VersionBuilder {
        self.value["_isDefault"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut TerminologyCapabilities_VersionBuilder {
        self.value["_language"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _property<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut TerminologyCapabilities_VersionBuilder {
        self.value["_property"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn code<'a>(&'a mut self, val: &str) -> &'a mut TerminologyCapabilities_VersionBuilder {
        self.value["code"] = json!(val);
        return self;
    }

    pub fn compositional<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut TerminologyCapabilities_VersionBuilder {
        self.value["compositional"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut TerminologyCapabilities_VersionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn filter<'a>(
        &'a mut self,
        val: Vec<TerminologyCapabilities_Filter>,
    ) -> &'a mut TerminologyCapabilities_VersionBuilder {
        self.value["filter"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut TerminologyCapabilities_VersionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn is_default<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut TerminologyCapabilities_VersionBuilder {
        self.value["isDefault"] = json!(val);
        return self;
    }

    pub fn language<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut TerminologyCapabilities_VersionBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut TerminologyCapabilities_VersionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn property<'a>(
        &'a mut self,
        val: Vec<&str>,
    ) -> &'a mut TerminologyCapabilities_VersionBuilder {
        self.value["property"] = json!(val);
        return self;
    }
}
