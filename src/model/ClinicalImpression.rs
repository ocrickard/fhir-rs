#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::ClinicalImpression_Finding::ClinicalImpression_Finding;
use crate::model::ClinicalImpression_Investigation::ClinicalImpression_Investigation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// A record of a clinical assessment performed to determine what problem(s) may
/// affect the patient and before planning the treatments or management strategies
/// that are best to manage a patient's condition. Assessments are often 1:1 with a
/// clinical consultation / encounter,  but this varies greatly depending on the
/// clinical workflow. This resource is called "ClinicalImpression" rather than
/// "ClinicalAssessment" to avoid confusion with the recording of assessment tools
/// such as Apgar score.

#[derive(Debug)]
pub struct ClinicalImpression<'a> {
    pub value: &'a Value,
}

impl ClinicalImpression<'_> {
    /// Categorizes the type of clinical assessment performed.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for date
    pub fn _date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_date") {
            return Some(Element { value: val });
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

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource and that modifies the understanding of the element
    /// that contains it and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer is allowed to define an extension, there is a set of requirements
    /// that SHALL be met as part of the definition of the extension. Applications
    /// processing a resource are required to check for modifier extensions.    Modifier
    /// extensions SHALL NOT change the meaning of any elements on Resource or
    /// DomainResource (including cannot change the meaning of modifierExtension
    /// itself).
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

    /// A text summary of the investigations and the diagnosis.
    pub fn summary(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("summary") {
            return Some(string);
        }
        return None;
    }

    /// The point in time or period over which the subject was assessed.
    pub fn effective_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("effectivePeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Captures the reason for the current state of the ClinicalImpression.
    pub fn status_reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("statusReason") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// RiskAssessment expressing likely outcome.
    pub fn prognosis_reference(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("prognosisReference") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Commentary about the impression, typically recorded after the impression itself
    /// was made, though supplemental notes by the original author could also appear.
    pub fn note(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("note") {
            return Some(
                val.into_iter()
                    .map(|e| Annotation { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The Encounter during which this ClinicalImpression was created or to which the
    /// creation of this record is tightly associated.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// A list of the relevant problems/conditions for a patient.
    pub fn problem(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("problem") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A summary of the context and/or cause of the assessment - why / where it was
    /// performed, and what patient events/status prompted it.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// A reference to the last assessment that was conducted on this patient.
    /// Assessments are often/usually ongoing in nature; a care provider (practitioner
    /// or team) will make new assessments on an ongoing basis as new data arises or the
    /// patient's conditions changes.
    pub fn previous(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("previous") {
            return Some(Reference { value: val });
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

    /// Extensions for summary
    pub fn _summary(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_summary") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for effectiveDateTime
    pub fn _effective_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_effectiveDateTime") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Identifies the workflow status of the assessment.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
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

    /// Business identifiers assigned to this clinical impression by the performer or
    /// other systems which remain constant as the resource is updated and propagates
    /// from server to server.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, and nor can they
    /// have their own independent transaction scope.
    pub fn contained(&self) -> Option<Vec<ResourceList>> {
        if let Some(Value::Array(val)) = self.value.get("contained") {
            return Some(
                val.into_iter()
                    .map(|e| ResourceList { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The patient or group of individuals assessed as part of this record.
    pub fn subject(&self) -> Reference {
        Reference {
            value: &self.value["subject"],
        }
    }

    /// Reference to a specific published clinical protocol that was followed during
    /// this assessment, and/or that provides evidence in support of the diagnosis.
    pub fn protocol(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("protocol") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The clinician performing the assessment.
    pub fn assessor(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("assessor") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Specific findings or diagnoses that were considered likely or relevant to
    /// ongoing treatment.
    pub fn finding(&self) -> Option<Vec<ClinicalImpression_Finding>> {
        if let Some(Value::Array(val)) = self.value.get("finding") {
            return Some(
                val.into_iter()
                    .map(|e| ClinicalImpression_Finding { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Extensions for protocol
    pub fn _protocol(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_protocol") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// One or more sets of investigations (signs, symptoms, etc.). The actual grouping
    /// of investigations varies greatly depending on the type and context of the
    /// assessment. These investigations may include data generated during the
    /// assessment process, or data previously generated and recorded that is pertinent
    /// to the outcomes.
    pub fn investigation(&self) -> Option<Vec<ClinicalImpression_Investigation>> {
        if let Some(Value::Array(val)) = self.value.get("investigation") {
            return Some(
                val.into_iter()
                    .map(|e| ClinicalImpression_Investigation { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The point in time or period over which the subject was assessed.
    pub fn effective_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("effectiveDateTime") {
            return Some(string);
        }
        return None;
    }

    /// Estimate of likely outcome.
    pub fn prognosis_codeable_concept(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("prognosisCodeableConcept") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource. To make the use of extensions safe and manageable,
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

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Indicates when the documentation of the assessment was complete.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need not
    /// encode all the structured data, but is required to contain sufficient detail to
    /// make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub fn text(&self) -> Option<Narrative> {
        if let Some(val) = self.value.get("text") {
            return Some(Narrative { value: val });
        }
        return None;
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Information supporting the clinical impression.
    pub fn supporting_info(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("supportingInfo") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.code() {
            _val.validate();
        }
        if let Some(_val) = self._date() {
            _val.validate();
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.summary() {}
        if let Some(_val) = self.effective_period() {
            _val.validate();
        }
        if let Some(_val) = self.status_reason() {
            _val.validate();
        }
        if let Some(_val) = self.prognosis_reference() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.note() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.encounter() {
            _val.validate();
        }
        if let Some(_val) = self.problem() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.previous() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self._summary() {
            _val.validate();
        }
        if let Some(_val) = self._effective_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.subject().validate();
        if let Some(_val) = self.protocol() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.assessor() {
            _val.validate();
        }
        if let Some(_val) = self.finding() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self._protocol() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.investigation() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.effective_date_time() {}
        if let Some(_val) = self.prognosis_codeable_concept() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.supporting_info() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
