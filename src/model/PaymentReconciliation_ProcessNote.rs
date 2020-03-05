#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This resource provides the details including amount of a payment and allocates
/// the payment items being paid.

#[derive(Debug)]
pub struct PaymentReconciliation_ProcessNote<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl PaymentReconciliation_ProcessNote<'_> {
    pub fn new(value: &Value) -> PaymentReconciliation_ProcessNote {
        PaymentReconciliation_ProcessNote {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for text
    pub fn _text(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_text") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
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

    /// The explanation or description associated with the processing.
    pub fn text(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("text") {
            return Some(string);
        }
        return None;
    }

    /// The business purpose of the note text.
    pub fn fhir_type(&self) -> Option<PaymentReconciliation_ProcessNoteType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(PaymentReconciliation_ProcessNoteType::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
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
        if let Some(_val) = self.text() {}
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct PaymentReconciliation_ProcessNoteBuilder {
    pub(crate) value: Value,
}

impl PaymentReconciliation_ProcessNoteBuilder {
    pub fn build(&self) -> PaymentReconciliation_ProcessNote {
        PaymentReconciliation_ProcessNote {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: PaymentReconciliation_ProcessNote,
    ) -> PaymentReconciliation_ProcessNoteBuilder {
        PaymentReconciliation_ProcessNoteBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> PaymentReconciliation_ProcessNoteBuilder {
        let mut __value: Value = json!({});
        return PaymentReconciliation_ProcessNoteBuilder { value: __value };
    }

    pub fn _text<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut PaymentReconciliation_ProcessNoteBuilder {
        self.value["_text"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut PaymentReconciliation_ProcessNoteBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut PaymentReconciliation_ProcessNoteBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut PaymentReconciliation_ProcessNoteBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut PaymentReconciliation_ProcessNoteBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: &str) -> &'a mut PaymentReconciliation_ProcessNoteBuilder {
        self.value["text"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: PaymentReconciliation_ProcessNoteType,
    ) -> &'a mut PaymentReconciliation_ProcessNoteBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum PaymentReconciliation_ProcessNoteType {
    Display,
    Print,
    Printoper,
}

impl PaymentReconciliation_ProcessNoteType {
    pub fn from_string(string: &str) -> Option<PaymentReconciliation_ProcessNoteType> {
        match string {
            "display" => Some(PaymentReconciliation_ProcessNoteType::Display),
            "print" => Some(PaymentReconciliation_ProcessNoteType::Print),
            "printoper" => Some(PaymentReconciliation_ProcessNoteType::Printoper),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            PaymentReconciliation_ProcessNoteType::Display => "display".to_string(),
            PaymentReconciliation_ProcessNoteType::Print => "print".to_string(),
            PaymentReconciliation_ProcessNoteType::Printoper => "printoper".to_string(),
        }
    }
}
