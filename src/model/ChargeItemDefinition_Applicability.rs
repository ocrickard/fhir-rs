#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The ChargeItemDefinition resource provides the properties that apply to the
/// (billing) codes necessary to calculate costs and prices. The properties may
/// differ largely depending on type and realm, therefore this resource gives only a
/// rough structure and requires profiling for each type of billing code system.

#[derive(Debug)]
pub struct ChargeItemDefinition_Applicability<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ChargeItemDefinition_Applicability<'_> {
    pub fn new(value: &Value) -> ChargeItemDefinition_Applicability {
        ChargeItemDefinition_Applicability {
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

    /// Extensions for expression
    pub fn _expression(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_expression") {
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

    /// A brief, natural language description of the condition that effectively
    /// communicates the intended semantics.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// An expression that returns true or false, indicating whether the condition is
    /// satisfied. When using FHIRPath expressions, the %context environment variable
    /// must be replaced at runtime with the ChargeItem resource to which this
    /// definition is applied.
    pub fn expression(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("expression") {
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

    /// The media type of the language for the expression, e.g. "text/cql" for Clinical
    /// Query Language expressions or "text/fhirpath" for FHIRPath expressions.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
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
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._expression() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.expression() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ChargeItemDefinition_ApplicabilityBuilder {
    pub(crate) value: Value,
}

impl ChargeItemDefinition_ApplicabilityBuilder {
    pub fn build(&self) -> ChargeItemDefinition_Applicability {
        ChargeItemDefinition_Applicability {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: ChargeItemDefinition_Applicability,
    ) -> ChargeItemDefinition_ApplicabilityBuilder {
        ChargeItemDefinition_ApplicabilityBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ChargeItemDefinition_ApplicabilityBuilder {
        let mut __value: Value = json!({});
        return ChargeItemDefinition_ApplicabilityBuilder { value: __value };
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ChargeItemDefinition_ApplicabilityBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _expression<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ChargeItemDefinition_ApplicabilityBuilder {
        self.value["_expression"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ChargeItemDefinition_ApplicabilityBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ChargeItemDefinition_ApplicabilityBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn expression<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ChargeItemDefinition_ApplicabilityBuilder {
        self.value["expression"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ChargeItemDefinition_ApplicabilityBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ChargeItemDefinition_ApplicabilityBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn language<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ChargeItemDefinition_ApplicabilityBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ChargeItemDefinition_ApplicabilityBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
