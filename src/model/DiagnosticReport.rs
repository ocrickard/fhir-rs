#![allow(unused_imports, non_camel_case_types)]

use crate::model::Attachment::Attachment;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::DiagnosticReport_Media::DiagnosticReport_Media;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// The findings and interpretation of diagnostic  tests performed on patients,
/// groups of patients, devices, and locations, and/or specimens derived from these.
/// The report includes clinical context such as requesting and provider
/// information, and some mix of atomic results, images, textual and coded
/// interpretations, and formatted representation of diagnostic reports.

#[derive(Debug)]
pub struct DiagnosticReport<'a> {
    pub value: &'a Value,
}

impl DiagnosticReport<'_> {
    /// Extensions for conclusion
    pub fn _conclusion(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_conclusion") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Rich text representation of the entire result as issued by the diagnostic
    /// service. Multiple formats are allowed but they SHALL be semantically equivalent.
    pub fn presented_form(&self) -> Option<Vec<Attachment>> {
        if let Some(Value::Array(val)) = self.value.get("presentedForm") {
            return Some(
                val.into_iter()
                    .map(|e| Attachment { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The practitioner or organization that is responsible for the report's
    /// conclusions and interpretations.
    pub fn results_interpreter(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("resultsInterpreter") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Extensions for issued
    pub fn _issued(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_issued") {
            return Some(Element { value: val });
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
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

    /// Concise and clinically contextualized summary conclusion
    /// (interpretation/impression) of the diagnostic report.
    pub fn conclusion(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("conclusion") {
            return Some(string);
        }
        return None;
    }

    /// Details concerning a service requested.
    pub fn based_on(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("basedOn") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The time or time-period the observed values are related to. When the subject of
    /// the report is a patient, this is usually either the time of the procedure or of
    /// specimen collection(s), but very often the source of the date/time is not known,
    /// only the date/time itself.
    pub fn effective_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("effectivePeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// A code that classifies the clinical discipline, department or diagnostic service
    /// that created the report (e.g. cardiology, biochemistry, hematology, MRI). This
    /// is used for searching, sorting and display purposes.
    pub fn category(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("category") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
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

    /// The status of the diagnostic report.
    pub fn status(&self) -> Option<DiagnosticReportStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(DiagnosticReportStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// Identifiers assigned to this report by the performer or other systems.
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

    /// Details about the specimens on which this diagnostic report is based.
    pub fn specimen(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("specimen") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// One or more links to full details of any imaging performed during the diagnostic
    /// investigation. Typically, this is imaging performed by DICOM enabled modalities,
    /// but this is not required. A fully enabled PACS viewer can use this information
    /// to provide views of the source images.
    pub fn imaging_study(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("imagingStudy") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// One or more codes that represent the summary conclusion
    /// (interpretation/impression) of the diagnostic report.
    pub fn conclusion_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("conclusionCode") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The date and time that this version of the report was made available to
    /// providers, typically after the report was reviewed and verified.
    pub fn issued(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("issued") {
            return Some(string);
        }
        return None;
    }

    /// [Observations](observation.html)  that are part of this diagnostic report.
    pub fn result(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("result") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The diagnostic service that is responsible for issuing the report.
    pub fn performer(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("performer") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The time or time-period the observed values are related to. When the subject of
    /// the report is a patient, this is usually either the time of the procedure or of
    /// specimen collection(s), but very often the source of the date/time is not known,
    /// only the date/time itself.
    pub fn effective_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("effectiveDateTime") {
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

    /// A list of key images associated with this report. The images are generally
    /// created during the diagnostic process, and may be directly of the patient, or of
    /// treated specimens (i.e. slides of interest).
    pub fn media(&self) -> Option<Vec<DiagnosticReport_Media>> {
        if let Some(Value::Array(val)) = self.value.get("media") {
            return Some(
                val.into_iter()
                    .map(|e| DiagnosticReport_Media { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The healthcare event  (e.g. a patient and healthcare provider interaction) which
    /// this DiagnosticReport is about.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference { value: val });
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

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The subject of the report. Usually, but not always, this is a patient. However,
    /// diagnostic services also perform analyses on specimens collected from a variety
    /// of other sources.
    pub fn subject(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subject") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// A code or name that describes this diagnostic report.
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["code"],
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._conclusion() {
            _val.validate();
        }
        if let Some(_val) = self.presented_form() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.results_interpreter() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._issued() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self._effective_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.conclusion() {}
        if let Some(_val) = self.based_on() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.effective_period() {
            _val.validate();
        }
        if let Some(_val) = self.category() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.specimen() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.imaging_study() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.conclusion_code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.issued() {}
        if let Some(_val) = self.result() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.performer() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.effective_date_time() {}
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.media() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.encounter() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.subject() {
            _val.validate();
        }
        let _ = self.code().validate();
        return true;
    }
}

#[derive(Debug)]
pub enum DiagnosticReportStatus {
    Registered,
    Partial,
    Preliminary,
    Final,
    Amended,
    Corrected,
    Appended,
    Cancelled,
    EnteredInError,
    Unknown,
}

impl DiagnosticReportStatus {
    pub fn from_string(string: &str) -> Option<DiagnosticReportStatus> {
        match string {
            "registered" => Some(DiagnosticReportStatus::Registered),
            "partial" => Some(DiagnosticReportStatus::Partial),
            "preliminary" => Some(DiagnosticReportStatus::Preliminary),
            "final" => Some(DiagnosticReportStatus::Final),
            "amended" => Some(DiagnosticReportStatus::Amended),
            "corrected" => Some(DiagnosticReportStatus::Corrected),
            "appended" => Some(DiagnosticReportStatus::Appended),
            "cancelled" => Some(DiagnosticReportStatus::Cancelled),
            "entered-in-error" => Some(DiagnosticReportStatus::EnteredInError),
            "unknown" => Some(DiagnosticReportStatus::Unknown),
            _ => None,
        }
    }
}
