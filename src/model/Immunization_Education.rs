#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Describes the event of a patient being administered a vaccine or a record of an
/// immunization as reported by a patient, a clinician or another party.

#[derive(Debug)]
pub struct Immunization_Education<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Immunization_Education<'_> {
    pub fn new(value: &Value) -> Immunization_Education {
        Immunization_Education {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for documentType
    pub fn _document_type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_documentType") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for presentationDate
    pub fn _presentation_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_presentationDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for publicationDate
    pub fn _publication_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_publicationDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for reference
    pub fn _reference(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_reference") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifier of the material presented to the patient.
    pub fn document_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("documentType") {
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

    /// Date the educational material was given to the patient.
    pub fn presentation_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("presentationDate") {
            return Some(string);
        }
        return None;
    }

    /// Date the educational material was published.
    pub fn publication_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("publicationDate") {
            return Some(string);
        }
        return None;
    }

    /// Reference pointer to the educational material given to the patient if the
    /// information was on line.
    pub fn reference(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("reference") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._document_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._presentation_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._publication_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.document_type() {}
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
        if let Some(_val) = self.presentation_date() {}
        if let Some(_val) = self.publication_date() {}
        if let Some(_val) = self.reference() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Immunization_EducationBuilder {
    pub(crate) value: Value,
}

impl Immunization_EducationBuilder {
    pub fn build(&self) -> Immunization_Education {
        Immunization_Education {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Immunization_Education) -> Immunization_EducationBuilder {
        Immunization_EducationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Immunization_EducationBuilder {
        let mut __value: Value = json!({});
        return Immunization_EducationBuilder { value: __value };
    }

    pub fn _document_type<'a>(&'a mut self, val: Element) -> &'a mut Immunization_EducationBuilder {
        self.value["_documentType"] = json!(val.value);
        return self;
    }

    pub fn _presentation_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Immunization_EducationBuilder {
        self.value["_presentationDate"] = json!(val.value);
        return self;
    }

    pub fn _publication_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Immunization_EducationBuilder {
        self.value["_publicationDate"] = json!(val.value);
        return self;
    }

    pub fn _reference<'a>(&'a mut self, val: Element) -> &'a mut Immunization_EducationBuilder {
        self.value["_reference"] = json!(val.value);
        return self;
    }

    pub fn document_type<'a>(&'a mut self, val: &str) -> &'a mut Immunization_EducationBuilder {
        self.value["documentType"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Immunization_EducationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Immunization_EducationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Immunization_EducationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn presentation_date<'a>(&'a mut self, val: &str) -> &'a mut Immunization_EducationBuilder {
        self.value["presentationDate"] = json!(val);
        return self;
    }

    pub fn publication_date<'a>(&'a mut self, val: &str) -> &'a mut Immunization_EducationBuilder {
        self.value["publicationDate"] = json!(val);
        return self;
    }

    pub fn reference<'a>(&'a mut self, val: &str) -> &'a mut Immunization_EducationBuilder {
        self.value["reference"] = json!(val);
        return self;
    }
}
