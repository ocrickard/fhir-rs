#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::Timing::Timing;
use crate::model::VerificationResult_Attestation::VerificationResult_Attestation;
use crate::model::VerificationResult_PrimarySource::VerificationResult_PrimarySource;
use crate::model::VerificationResult_Validator::VerificationResult_Validator;
use serde_json::value::Value;

/// Describes validation requirements, source(s), status and dates for one or more
/// elements.

#[derive(Debug)]
pub struct VerificationResult<'a> {
    pub value: &'a Value,
}

impl VerificationResult<'_> {
    /// Information about the primary source(s) involved in validation.
    pub fn primary_source(&self) -> Option<Vec<VerificationResult_PrimarySource>> {
        if let Some(Value::Array(val)) = self.value.get("primarySource") {
            return Some(
                val.into_iter()
                    .map(|e| VerificationResult_PrimarySource { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Information about the entity attesting to information.
    pub fn attestation(&self) -> Option<VerificationResult_Attestation> {
        if let Some(val) = self.value.get("attestation") {
            return Some(VerificationResult_Attestation { value: val });
        }
        return None;
    }

    /// The date/time validation was last completed (including failed validations).
    pub fn last_performed(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("lastPerformed") {
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

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
        }
        return None;
    }

    /// What the target is validated against (nothing; primary source; multiple
    /// sources).
    pub fn validation_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("validationType") {
            return Some(CodeableConcept { value: val });
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for nextScheduled
    pub fn _next_scheduled(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_nextScheduled") {
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

    /// The date when target is next validated, if appropriate.
    pub fn next_scheduled(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("nextScheduled") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for lastPerformed
    pub fn _last_performed(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lastPerformed") {
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

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The fhirpath location(s) within the resource that was validated.
    pub fn target_location(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("targetLocation") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The frequency with which the target must be validated (none; initial; periodic).
    pub fn need(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("need") {
            return Some(CodeableConcept { value: val });
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

    /// Extensions for targetLocation
    pub fn _target_location(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_targetLocation") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Information about the entity validating information.
    pub fn validator(&self) -> Option<Vec<VerificationResult_Validator>> {
        if let Some(Value::Array(val)) = self.value.get("validator") {
            return Some(
                val.into_iter()
                    .map(|e| VerificationResult_Validator { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// When the validation status was updated.
    pub fn status_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("statusDate") {
            return Some(string);
        }
        return None;
    }

    /// The primary process by which the target is validated (edit check; value set;
    /// primary source; multiple sources; standalone; in context).
    pub fn validation_process(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("validationProcess") {
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

    /// Frequency of revalidation.
    pub fn frequency(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("frequency") {
            return Some(Timing { value: val });
        }
        return None;
    }

    /// The validation status of the target (attested; validated; in process; requires
    /// revalidation; validation failed; revalidation failed).
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for statusDate
    pub fn _status_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_statusDate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A resource that was validated.
    pub fn target(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("target") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The result if validation fails (fatal; warning; record only; none).
    pub fn failure_action(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("failureAction") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.primary_source() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.attestation() {
            _val.validate();
        }
        if let Some(_val) = self.last_performed() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.validation_type() {
            _val.validate();
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self._next_scheduled() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.next_scheduled() {}
        if let Some(_val) = self._last_performed() {
            _val.validate();
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.target_location() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.need() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._target_location() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.validator() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.status_date() {}
        if let Some(_val) = self.validation_process() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.frequency() {
            _val.validate();
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self._status_date() {
            _val.validate();
        }
        if let Some(_val) = self.target() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.failure_action() {
            _val.validate();
        }
        return true;
    }
}
