#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Immunization_Education::Immunization_Education;
use crate::model::Immunization_Performer::Immunization_Performer;
use crate::model::Immunization_ProtocolApplied::Immunization_ProtocolApplied;
use crate::model::Immunization_Reaction::Immunization_Reaction;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// Describes the event of a patient being administered a vaccine or a record of an
/// immunization as reported by a patient, a clinician or another party.

#[derive(Debug)]
pub struct Immunization<'a> {
    pub value: &'a Value,
}

impl Immunization<'_> {
    /// The path by which the vaccine product is taken into the body.
    pub fn route(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("route") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// A unique identifier assigned to this immunization record.
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

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
        }
        return None;
    }

    /// Extensions for isSubpotent
    pub fn _is_subpotent(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_isSubpotent") {
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

    /// Date vaccine administered or was to be administered.
    pub fn occurrence_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("occurrenceString") {
            return Some(string);
        }
        return None;
    }

    /// Indicates the reason the immunization event was not performed.
    pub fn status_reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("statusReason") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Date vaccine administered or was to be administered.
    pub fn occurrence_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("occurrenceDateTime") {
            return Some(string);
        }
        return None;
    }

    /// The date the occurrence of the immunization was first captured in the record -
    /// potentially significantly after the occurrence of the event.
    pub fn recorded(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("recorded") {
            return Some(string);
        }
        return None;
    }

    /// Body site where vaccine was administered.
    pub fn site(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("site") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Indicates a patient's eligibility for a funding program.
    pub fn program_eligibility(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("programEligibility") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for occurrenceString
    pub fn _occurrence_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_occurrenceString") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for recorded
    pub fn _recorded(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_recorded") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The protocol (set of recommendations) being followed by the provider who
    /// administered the dose.
    pub fn protocol_applied(&self) -> Option<Vec<Immunization_ProtocolApplied>> {
        if let Some(Value::Array(val)) = self.value.get("protocolApplied") {
            return Some(
                val.into_iter()
                    .map(|e| Immunization_ProtocolApplied { value: e })
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

    /// Lot number of the  vaccine product.
    pub fn lot_number(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("lotNumber") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for lotNumber
    pub fn _lot_number(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lotNumber") {
            return Some(Element { value: val });
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

    /// Extensions for occurrenceDateTime
    pub fn _occurrence_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_occurrenceDateTime") {
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

    /// Vaccine that was administered or was to be administered.
    pub fn vaccine_code(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["vaccineCode"],
        }
    }

    /// Extensions for primarySource
    pub fn _primary_source(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_primarySource") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for expirationDate
    pub fn _expiration_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_expirationDate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The quantity of vaccine product that was administered.
    pub fn dose_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("doseQuantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// Reason why a dose is considered to be subpotent.
    pub fn subpotent_reason(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("subpotentReason") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
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

    /// Reasons why the vaccine was administered.
    pub fn reason_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("reasonCode") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indication if a dose is considered to be subpotent. By default, a dose should be
    /// considered to be potent.
    pub fn is_subpotent(&self) -> Option<bool> {
        if let Some(val) = self.value.get("isSubpotent") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Categorical data indicating that an adverse event is associated in time to an
    /// immunization.
    pub fn reaction(&self) -> Option<Vec<Immunization_Reaction>> {
        if let Some(Value::Array(val)) = self.value.get("reaction") {
            return Some(
                val.into_iter()
                    .map(|e| Immunization_Reaction { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An indication that the content of the record is based on information from the
    /// person who administered the vaccine. This reflects the context under which the
    /// data was originally recorded.
    pub fn primary_source(&self) -> Option<bool> {
        if let Some(val) = self.value.get("primarySource") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The source of the data when the report of the immunization event is not based on
    /// information from the person who administered the vaccine.
    pub fn report_origin(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("reportOrigin") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Name of vaccine manufacturer.
    pub fn manufacturer(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("manufacturer") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Indicates who performed the immunization event.
    pub fn performer(&self) -> Option<Vec<Immunization_Performer>> {
        if let Some(Value::Array(val)) = self.value.get("performer") {
            return Some(
                val.into_iter()
                    .map(|e| Immunization_Performer { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Indicates the source of the vaccine actually administered. This may be different
    /// than the patient eligibility (e.g. the patient may be eligible for a publically
    /// purchased vaccine but due to inventory issues, vaccine purchased with private
    /// funds was actually administered).
    pub fn funding_source(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("fundingSource") {
            return Some(CodeableConcept { value: val });
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

    /// The patient who either received or did not receive the immunization.
    pub fn patient(&self) -> Reference {
        Reference {
            value: &self.value["patient"],
        }
    }

    /// The service delivery location where the vaccine administration occurred.
    pub fn location(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("location") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Date vaccine batch expires.
    pub fn expiration_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("expirationDate") {
            return Some(string);
        }
        return None;
    }

    /// Extra information about the immunization that is not conveyed by the other
    /// attributes.
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

    /// Condition, Observation or DiagnosticReport that supports why the immunization
    /// was administered.
    pub fn reason_reference(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("reasonReference") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Educational material presented to the patient (or guardian) at the time of
    /// vaccine administration.
    pub fn education(&self) -> Option<Vec<Immunization_Education>> {
        if let Some(Value::Array(val)) = self.value.get("education") {
            return Some(
                val.into_iter()
                    .map(|e| Immunization_Education { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates the current status of the immunization event.
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

    /// The visit or admission or other contact between patient and health care provider
    /// the immunization was performed as part of.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.route() {
            _val.validate();
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self._is_subpotent() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.occurrence_string() {}
        if let Some(_val) = self.status_reason() {
            _val.validate();
        }
        if let Some(_val) = self.occurrence_date_time() {}
        if let Some(_val) = self.recorded() {}
        if let Some(_val) = self.site() {
            _val.validate();
        }
        if let Some(_val) = self.program_eligibility() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._occurrence_string() {
            _val.validate();
        }
        if let Some(_val) = self._recorded() {
            _val.validate();
        }
        if let Some(_val) = self.protocol_applied() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.lot_number() {}
        if let Some(_val) = self._lot_number() {
            _val.validate();
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self._occurrence_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        let _ = self.vaccine_code().validate();
        if let Some(_val) = self._primary_source() {
            _val.validate();
        }
        if let Some(_val) = self._expiration_date() {
            _val.validate();
        }
        if let Some(_val) = self.dose_quantity() {
            _val.validate();
        }
        if let Some(_val) = self.subpotent_reason() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.reason_code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.is_subpotent() {}
        if let Some(_val) = self.reaction() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.primary_source() {}
        if let Some(_val) = self.report_origin() {
            _val.validate();
        }
        if let Some(_val) = self.manufacturer() {
            _val.validate();
        }
        if let Some(_val) = self.performer() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.funding_source() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.patient().validate();
        if let Some(_val) = self.location() {
            _val.validate();
        }
        if let Some(_val) = self.expiration_date() {}
        if let Some(_val) = self.note() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.reason_reference() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.education() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.encounter() {
            _val.validate();
        }
        return true;
    }
}
