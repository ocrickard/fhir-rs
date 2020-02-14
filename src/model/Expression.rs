#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// A expression that is evaluated in a specified context and returns a value. The
/// context of use of the expression must specify the context in which the
/// expression is evaluated, and how the result of the expression is used.

#[derive(Debug)]
pub struct Expression<'a> {
    pub value: &'a Value,
}

impl Expression<'_> {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
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

    /// Extensions for reference
    pub fn _reference(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_reference") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The media type of the language for the expression.
    pub fn language(&self) -> Option<ExpressionLanguage> {
        if let Some(Value::String(val)) = self.value.get("language") {
            return Some(ExpressionLanguage::from_string(&val).unwrap());
        }
        return None;
    }

    /// An expression in the specified language that returns a value.
    pub fn expression(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("expression") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for expression
    pub fn _expression(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_expression") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A short name assigned to the expression to allow for multiple reuse of the
    /// expression in the context where it is defined.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A URI that defines where the expression is found.
    pub fn reference(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("reference") {
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.description() {}
        if let Some(_val) = self._reference() {
            _val.validate();
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self._name() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.expression() {}
        if let Some(_val) = self._expression() {
            _val.validate();
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.reference() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

#[derive(Debug)]
pub enum ExpressionLanguage {
    TextCql,
    TextFhirpath,
    ApplicationXFhirQuery,
}

impl ExpressionLanguage {
    pub fn from_string(string: &str) -> Option<ExpressionLanguage> {
        match string {
            "text/cql" => Some(ExpressionLanguage::TextCql),
            "text/fhirpath" => Some(ExpressionLanguage::TextFhirpath),
            "application/x-fhir-query" => Some(ExpressionLanguage::ApplicationXFhirQuery),
            _ => None,
        }
    }
}
