#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// A reference to a document of any kind for any purpose. Provides metadata about
/// the document so that the document can be discovered and managed. The scope of a
/// document is any seralized object with a mime-type, so includes formal patient
/// centric documents (CDA), cliical notes, scanned paper, and non-patient specific
/// documents like policy text.

#[derive(Debug)]
pub struct DocumentReference_Context<'a> {
    pub value: &'a Value,
}

impl DocumentReference_Context<'_> {
    /// The Patient Information as known when the document was published. May be a
    /// reference to a version specific, or contained.
    pub fn source_patient_info(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("sourcePatientInfo") {
            return Some(Reference { value: val });
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

    /// This list of codes represents the main clinical acts, such as a colonoscopy or
    /// an appendectomy, being documented. In some cases, the event is inherent in the
    /// type Code, such as a "History and Physical Report" in which the procedure being
    /// documented is necessarily a "History and Physical" act.
    pub fn event(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("event") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
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

    /// The time period over which the service that is described by the document was
    /// provided.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Describes the clinical encounter or type of care that the document content is
    /// associated with.
    pub fn encounter(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("encounter") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Related identifiers or resources associated with the DocumentReference.
    pub fn related(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("related") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The kind of facility where the patient was seen.
    pub fn facility_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("facilityType") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// This property may convey specifics about the practice setting where the content
    /// was created, often reflecting the clinical specialty.
    pub fn practice_setting(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("practiceSetting") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.source_patient_info() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.event() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.period() {
            _val.validate();
        }
        if let Some(_val) = self.encounter() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.related() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.facility_type() {
            _val.validate();
        }
        if let Some(_val) = self.practice_setting() {
            _val.validate();
        }
        return true;
    }
}
