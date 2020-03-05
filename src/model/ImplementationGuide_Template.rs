#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used to
/// gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.

#[derive(Debug)]
pub struct ImplementationGuide_Template<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ImplementationGuide_Template<'_> {
    pub fn new(value: &Value) -> ImplementationGuide_Template {
        ImplementationGuide_Template {
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

    /// Extensions for scope
    pub fn _scope(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_scope") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for source
    pub fn _source(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_source") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Type of template specified.
    pub fn code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("code") {
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

    /// The scope in which the template applies.
    pub fn scope(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("scope") {
            return Some(string);
        }
        return None;
    }

    /// The source location for the template.
    pub fn source(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("source") {
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
        if let Some(_val) = self._scope() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._source() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.code() {}
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
        if let Some(_val) = self.scope() {}
        if let Some(_val) = self.source() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ImplementationGuide_TemplateBuilder {
    pub(crate) value: Value,
}

impl ImplementationGuide_TemplateBuilder {
    pub fn build(&self) -> ImplementationGuide_Template {
        ImplementationGuide_Template {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ImplementationGuide_Template) -> ImplementationGuide_TemplateBuilder {
        ImplementationGuide_TemplateBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ImplementationGuide_TemplateBuilder {
        let mut __value: Value = json!({});
        return ImplementationGuide_TemplateBuilder { value: __value };
    }

    pub fn _code<'a>(&'a mut self, val: Element) -> &'a mut ImplementationGuide_TemplateBuilder {
        self.value["_code"] = json!(val.value);
        return self;
    }

    pub fn _scope<'a>(&'a mut self, val: Element) -> &'a mut ImplementationGuide_TemplateBuilder {
        self.value["_scope"] = json!(val.value);
        return self;
    }

    pub fn _source<'a>(&'a mut self, val: Element) -> &'a mut ImplementationGuide_TemplateBuilder {
        self.value["_source"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuide_TemplateBuilder {
        self.value["code"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImplementationGuide_TemplateBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuide_TemplateBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImplementationGuide_TemplateBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn scope<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuide_TemplateBuilder {
        self.value["scope"] = json!(val);
        return self;
    }

    pub fn source<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuide_TemplateBuilder {
        self.value["source"] = json!(val);
        return self;
    }
}
