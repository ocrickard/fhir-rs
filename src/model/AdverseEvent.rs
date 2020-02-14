#![allow(unused_imports, non_camel_case_types)]

use crate::model::AdverseEvent_SuspectEntity::AdverseEvent_SuspectEntity;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// Actual or  potential/avoided event causing unintended physical injury resulting
/// from or contributed to by medical care, a research study or other healthcare
/// setting factors that requires additional monitoring, treatment, or
/// hospitalization, or that results in death.

#[derive(Debug)]
pub struct AdverseEvent<'a> {
    pub value: &'a Value,
}

impl AdverseEvent<'_> {
    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element { value: val });
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

    /// Extensions for actuality
    pub fn _actuality(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_actuality") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The information about where the adverse event occurred.
    pub fn location(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("location") {
            return Some(Reference { value: val });
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

    /// AdverseEvent.subjectMedicalHistory.
    pub fn subject_medical_history(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("subjectMedicalHistory") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// AdverseEvent.referenceDocument.
    pub fn reference_document(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("referenceDocument") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// AdverseEvent.study.
    pub fn study(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("study") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
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

    /// Describes the entity that is suspected to have caused the adverse event.
    pub fn suspect_entity(&self) -> Option<Vec<AdverseEvent_SuspectEntity>> {
        if let Some(Value::Array(val)) = self.value.get("suspectEntity") {
            return Some(
                val.into_iter()
                    .map(|e| AdverseEvent_SuspectEntity { value: e })
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

    /// The overall type of event, intended for search and filtering purposes.
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

    /// Estimated or actual date the AdverseEvent began, in the opinion of the reporter.
    pub fn detected(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("detected") {
            return Some(string);
        }
        return None;
    }

    /// Parties that may or should contribute or have contributed information to the
    /// adverse event, which can consist of one or more activities.  Such information
    /// includes information leading to the decision to perform the activity and how to
    /// perform the activity (e.g. consultant), information that the activity itself
    /// seeks to reveal (e.g. informant of clinical history), or information about what
    /// activity was performed (e.g. informant witness).
    pub fn contributor(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("contributor") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Whether the event actually happened, or just had the potential to. Note that
    /// this is independent of whether anyone was affected or harmed or how severely.
    pub fn actuality(&self) -> Option<AdverseEventActuality> {
        if let Some(Value::String(val)) = self.value.get("actuality") {
            return Some(AdverseEventActuality::from_string(&val).unwrap());
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

    /// This element defines the specific type of event that occurred or that was
    /// prevented from occurring.
    pub fn event(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("event") {
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

    /// This subject or group impacted by the event.
    pub fn subject(&self) -> Reference {
        Reference {
            value: &self.value["subject"],
        }
    }

    /// The Encounter during which AdverseEvent was created or to which the creation of
    /// this record is tightly associated.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Describes the type of outcome from the adverse event.
    pub fn outcome(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("outcome") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Business identifiers assigned to this adverse event by the performer or other
    /// systems which remain constant as the resource is updated and propagates from
    /// server to server.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier { value: val });
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

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
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

    /// Describes the severity of the adverse event, in relation to the subject.
    /// Contrast to AdverseEvent.seriousness - a severe rash might not be serious, but a
    /// mild heart problem is.
    pub fn severity(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("severity") {
            return Some(CodeableConcept { value: val });
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

    /// Extensions for detected
    pub fn _detected(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_detected") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The date (and perhaps time) when the adverse event occurred.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// The date on which the existence of the AdverseEvent was first recorded.
    pub fn recorded_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("recordedDate") {
            return Some(string);
        }
        return None;
    }

    /// Assessment whether this event was of real importance.
    pub fn seriousness(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("seriousness") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for recordedDate
    pub fn _recorded_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_recordedDate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Includes information about the reaction that occurred as a result of exposure to
    /// a substance (for example, a drug or a chemical).
    pub fn resulting_condition(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("resultingCondition") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Information on who recorded the adverse event.  May be the patient or a
    /// practitioner.
    pub fn recorder(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("recorder") {
            return Some(Reference { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._actuality() {
            _val.validate();
        }
        if let Some(_val) = self.location() {
            _val.validate();
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.subject_medical_history() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.reference_document() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.study() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.suspect_entity() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.category() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.detected() {}
        if let Some(_val) = self.contributor() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.actuality() {}
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.event() {
            _val.validate();
        }
        if let Some(_val) = self._date() {
            _val.validate();
        }
        let _ = self.subject().validate();
        if let Some(_val) = self.encounter() {
            _val.validate();
        }
        if let Some(_val) = self.outcome() {
            _val.validate();
        }
        if let Some(_val) = self.identifier() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.severity() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self._detected() {
            _val.validate();
        }
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.recorded_date() {}
        if let Some(_val) = self.seriousness() {
            _val.validate();
        }
        if let Some(_val) = self._recorded_date() {
            _val.validate();
        }
        if let Some(_val) = self.resulting_condition() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.recorder() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum AdverseEventActuality {
    Actual,
    Potential,
}

impl AdverseEventActuality {
    pub fn from_string(string: &str) -> Option<AdverseEventActuality> {
        match string {
            "actual" => Some(AdverseEventActuality::Actual),
            "potential" => Some(AdverseEventActuality::Potential),
            _ => None,
        }
    }
}
