#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This resource provides the adjudication details from the processing of a Claim
/// resource.

#[derive(Debug)]
pub struct ClaimResponse_ProcessNote<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ClaimResponse_ProcessNote<'_> {
    pub fn new(value: &Value) -> ClaimResponse_ProcessNote {
        ClaimResponse_ProcessNote {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for number
    pub fn _number(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_number") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
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

    /// A code to define the language used in the text of the note.
    pub fn language(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("language") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// A number to uniquely identify a note entry.
    pub fn number(&self) -> Option<i64> {
        if let Some(val) = self.value.get("number") {
            return Some(val.as_i64().unwrap());
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
    pub fn fhir_type(&self) -> Option<ClaimResponse_ProcessNoteType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(ClaimResponse_ProcessNoteType::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._number() {
            if !_val.validate() {
                return false;
            }
        }
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
        if let Some(_val) = self.language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.number() {}
        if let Some(_val) = self.text() {}
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ClaimResponse_ProcessNoteBuilder {
    pub(crate) value: Value,
}

impl ClaimResponse_ProcessNoteBuilder {
    pub fn build(&self) -> ClaimResponse_ProcessNote {
        ClaimResponse_ProcessNote {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ClaimResponse_ProcessNote) -> ClaimResponse_ProcessNoteBuilder {
        ClaimResponse_ProcessNoteBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ClaimResponse_ProcessNoteBuilder {
        let mut __value: Value = json!({});
        return ClaimResponse_ProcessNoteBuilder { value: __value };
    }

    pub fn _number<'a>(&'a mut self, val: Element) -> &'a mut ClaimResponse_ProcessNoteBuilder {
        self.value["_number"] = json!(val.value);
        return self;
    }

    pub fn _text<'a>(&'a mut self, val: Element) -> &'a mut ClaimResponse_ProcessNoteBuilder {
        self.value["_text"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut ClaimResponse_ProcessNoteBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ClaimResponse_ProcessNoteBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ClaimResponse_ProcessNoteBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn language<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ClaimResponse_ProcessNoteBuilder {
        self.value["language"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ClaimResponse_ProcessNoteBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn number<'a>(&'a mut self, val: i64) -> &'a mut ClaimResponse_ProcessNoteBuilder {
        self.value["number"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: &str) -> &'a mut ClaimResponse_ProcessNoteBuilder {
        self.value["text"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: ClaimResponse_ProcessNoteType,
    ) -> &'a mut ClaimResponse_ProcessNoteBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum ClaimResponse_ProcessNoteType {
    Display,
    Print,
    Printoper,
}

impl ClaimResponse_ProcessNoteType {
    pub fn from_string(string: &str) -> Option<ClaimResponse_ProcessNoteType> {
        match string {
            "display" => Some(ClaimResponse_ProcessNoteType::Display),
            "print" => Some(ClaimResponse_ProcessNoteType::Print),
            "printoper" => Some(ClaimResponse_ProcessNoteType::Printoper),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ClaimResponse_ProcessNoteType::Display => "display".to_string(),
            ClaimResponse_ProcessNoteType::Print => "print".to_string(),
            ClaimResponse_ProcessNoteType::Printoper => "printoper".to_string(),
        }
    }
}
