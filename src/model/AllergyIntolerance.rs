#![allow(unused_imports, non_camel_case_types)]

use crate::model::Age::Age;
use crate::model::AllergyIntolerance_Reaction::AllergyIntolerance_Reaction;
use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Range::Range;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// Risk of harmful or undesirable, physiological response which is unique to an
/// individual and associated with exposure to a substance.

#[derive(Debug)]
pub struct AllergyIntolerance<'a> {
    pub value: &'a Value,
}

impl AllergyIntolerance<'_> {
    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// Estimated or actual date,  date-time, or age when allergy or intolerance was
    /// identified.
    pub fn onset_age(&self) -> Option<Age> {
        if let Some(val) = self.value.get("onsetAge") {
            return Some(Age { value: val });
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

    /// The recordedDate represents when this particular AllergyIntolerance record was
    /// created in the system, which is often a system-generated date.
    pub fn recorded_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("recordedDate") {
            return Some(string);
        }
        return None;
    }

    /// Individual who recorded the record and takes responsibility for its content.
    pub fn recorder(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("recorder") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Estimated or actual date,  date-time, or age when allergy or intolerance was
    /// identified.
    pub fn onset_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("onsetString") {
            return Some(string);
        }
        return None;
    }

    /// The patient who has the allergy or intolerance.
    pub fn patient(&self) -> Reference {
        Reference {
            value: &self.value["patient"],
        }
    }

    /// Extensions for onsetDateTime
    pub fn _onset_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_onsetDateTime") {
            return Some(Element { value: val });
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

    /// The source of the information about the allergy that is recorded.
    pub fn asserter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("asserter") {
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

    /// The clinical status of the allergy or intolerance.
    pub fn clinical_status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("clinicalStatus") {
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

    /// Extensions for criticality
    pub fn _criticality(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_criticality") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for onsetString
    pub fn _onset_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_onsetString") {
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

    /// Estimated or actual date,  date-time, or age when allergy or intolerance was
    /// identified.
    pub fn onset_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("onsetPeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Identification of the underlying physiological mechanism for the reaction risk.
    pub fn fhir_type(&self) -> Option<AllergyIntoleranceType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(AllergyIntoleranceType::from_string(&val).unwrap());
        }
        return None;
    }

    /// Represents the date and/or time of the last known occurrence of a reaction
    /// event.
    pub fn last_occurrence(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("lastOccurrence") {
            return Some(string);
        }
        return None;
    }

    /// Assertion about certainty associated with the propensity, or potential risk, of
    /// a reaction to the identified substance (including pharmaceutical product).
    pub fn verification_status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("verificationStatus") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Code for an allergy or intolerance statement (either a positive or a
    /// negated/excluded statement).  This may be a code for a substance or
    /// pharmaceutical product that is considered to be responsible for the adverse
    /// reaction risk (e.g., "Latex"), an allergy or intolerance condition (e.g., "Latex
    /// allergy"), or a negated/excluded code for a specific substance or class (e.g.,
    /// "No latex allergy") or a general or categorical negated statement (e.g.,  "No
    /// known allergy", "No known drug allergies").  Note: the substance for a specific
    /// reaction may be different from the substance identified as the cause of the
    /// risk, but it must be consistent with it. For instance, it may be a more specific
    /// substance (e.g. a brand medication) or a composite product that includes the
    /// identified substance. It must be clinically safe to only process the 'code' and
    /// ignore the 'reaction.substance'.  If a receiving system is unable to confirm
    /// that AllergyIntolerance.reaction.substance falls within the semantic scope of
    /// AllergyIntolerance.code, then the receiving system should ignore
    /// AllergyIntolerance.reaction.substance.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Estimated or actual date,  date-time, or age when allergy or intolerance was
    /// identified.
    pub fn onset_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("onsetDateTime") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for lastOccurrence
    pub fn _last_occurrence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lastOccurrence") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Details about each adverse reaction event linked to exposure to the identified
    /// substance.
    pub fn reaction(&self) -> Option<Vec<AllergyIntolerance_Reaction>> {
        if let Some(Value::Array(val)) = self.value.get("reaction") {
            return Some(
                val.into_iter()
                    .map(|e| AllergyIntolerance_Reaction { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The encounter when the allergy or intolerance was asserted.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Business identifiers assigned to this AllergyIntolerance by the performer or
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

    /// Extensions for category
    pub fn _category(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_category") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Estimated or actual date,  date-time, or age when allergy or intolerance was
    /// identified.
    pub fn onset_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("onsetRange") {
            return Some(Range { value: val });
        }
        return None;
    }

    /// Additional narrative about the propensity for the Adverse Reaction, not captured
    /// in other fields.
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

    /// Estimate of the potential clinical harm, or seriousness, of the reaction to the
    /// identified substance.
    pub fn criticality(&self) -> Option<AllergyIntoleranceCriticality> {
        if let Some(Value::String(val)) = self.value.get("criticality") {
            return Some(AllergyIntoleranceCriticality::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.onset_age() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.recorded_date() {}
        if let Some(_val) = self.recorder() {
            _val.validate();
        }
        if let Some(_val) = self.onset_string() {}
        let _ = self.patient().validate();
        if let Some(_val) = self._onset_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.asserter() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.clinical_status() {
            _val.validate();
        }
        if let Some(_val) = self._recorded_date() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._criticality() {
            _val.validate();
        }
        if let Some(_val) = self._onset_string() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.onset_period() {
            _val.validate();
        }
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.last_occurrence() {}
        if let Some(_val) = self.verification_status() {
            _val.validate();
        }
        if let Some(_val) = self.code() {
            _val.validate();
        }
        if let Some(_val) = self._type() {
            _val.validate();
        }
        if let Some(_val) = self.onset_date_time() {}
        if let Some(_val) = self._last_occurrence() {
            _val.validate();
        }
        if let Some(_val) = self.reaction() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.encounter() {
            _val.validate();
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._category() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.onset_range() {
            _val.validate();
        }
        if let Some(_val) = self.note() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.criticality() {}
        return true;
    }
}

#[derive(Debug)]
pub enum AllergyIntoleranceType {
    Allergy,
    Intolerance,
}

impl AllergyIntoleranceType {
    pub fn from_string(string: &str) -> Option<AllergyIntoleranceType> {
        match string {
            "allergy" => Some(AllergyIntoleranceType::Allergy),
            "intolerance" => Some(AllergyIntoleranceType::Intolerance),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum AllergyIntoleranceCriticality {
    Low,
    High,
    UnableToAssess,
}

impl AllergyIntoleranceCriticality {
    pub fn from_string(string: &str) -> Option<AllergyIntoleranceCriticality> {
        match string {
            "low" => Some(AllergyIntoleranceCriticality::Low),
            "high" => Some(AllergyIntoleranceCriticality::High),
            "unable-to-assess" => Some(AllergyIntoleranceCriticality::UnableToAssess),
            _ => None,
        }
    }
}
