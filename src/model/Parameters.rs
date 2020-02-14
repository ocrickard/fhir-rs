#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Meta::Meta;
use crate::model::Parameters_Parameter::Parameters_Parameter;
use serde_json::value::Value;

/// This resource is a non-persisted resource used to pass information into and back
/// from an [operation](operations.html). It has no other use, and there is no
/// RESTful endpoint associated with it.

#[derive(Debug)]
pub struct Parameters<'a> {
    pub value: &'a Value,
}

impl Parameters<'_> {
    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
        }
        return None;
    }

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// A parameter passed to or received from the operation.
    pub fn parameter(&self) -> Option<Vec<Parameters_Parameter>> {
        if let Some(Value::Array(val)) = self.value.get("parameter") {
            return Some(
                val.into_iter()
                    .map(|e| Parameters_Parameter { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub fn implicit_rules(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("implicitRules") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.parameter() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        return true;
    }
}
