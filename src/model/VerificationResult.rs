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
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Describes validation requirements, source(s), status and dates for one or more
/// elements.

#[derive(Debug)]
pub struct VerificationResult<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl VerificationResult<'_> {
    pub fn new(value: &Value) -> VerificationResult {
        VerificationResult {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for lastPerformed
    pub fn _last_performed(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lastPerformed") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for nextScheduled
    pub fn _next_scheduled(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_nextScheduled") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for statusDate
    pub fn _status_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_statusDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for targetLocation
    pub fn _target_location(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_targetLocation") {
            return Some(
                val.into_iter()
                    .map(|e| Element {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Information about the entity attesting to information.
    pub fn attestation(&self) -> Option<VerificationResult_Attestation> {
        if let Some(val) = self.value.get("attestation") {
            return Some(VerificationResult_Attestation {
                value: Cow::Borrowed(val),
            });
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
                    .map(|e| ResourceList {
                        value: Cow::Borrowed(e),
                    })
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The result if validation fails (fatal; warning; record only; none).
    pub fn failure_action(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("failureAction") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Frequency of revalidation.
    pub fn frequency(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("frequency") {
            return Some(Timing {
                value: Cow::Borrowed(val),
            });
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
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

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta {
                value: Cow::Borrowed(val),
            });
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The frequency with which the target must be validated (none; initial; periodic).
    pub fn need(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("need") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// Information about the primary source(s) involved in validation.
    pub fn primary_source(&self) -> Option<Vec<VerificationResult_PrimarySource>> {
        if let Some(Value::Array(val)) = self.value.get("primarySource") {
            return Some(
                val.into_iter()
                    .map(|e| VerificationResult_PrimarySource {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// When the validation status was updated.
    pub fn status_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("statusDate") {
            return Some(string);
        }
        return None;
    }

    /// A resource that was validated.
    pub fn target(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("target") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need not
    /// encode all the structured data, but is required to contain sufficient detail to
    /// make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub fn text(&self) -> Option<Narrative> {
        if let Some(val) = self.value.get("text") {
            return Some(Narrative {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The primary process by which the target is validated (edit check; value set;
    /// primary source; multiple sources; standalone; in context).
    pub fn validation_process(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("validationProcess") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// What the target is validated against (nothing; primary source; multiple
    /// sources).
    pub fn validation_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("validationType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Information about the entity validating information.
    pub fn validator(&self) -> Option<Vec<VerificationResult_Validator>> {
        if let Some(Value::Array(val)) = self.value.get("validator") {
            return Some(
                val.into_iter()
                    .map(|e| VerificationResult_Validator {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._last_performed() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._next_scheduled() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._target_location() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.attestation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.failure_action() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.frequency() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.last_performed() {}
        if let Some(_val) = self.meta() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.need() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.next_scheduled() {}
        if let Some(_val) = self.primary_source() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.status_date() {}
        if let Some(_val) = self.target() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.target_location() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.validation_process() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.validation_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.validator() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct VerificationResultBuilder {
    pub(crate) value: Value,
}

impl VerificationResultBuilder {
    pub fn build(&self) -> VerificationResult {
        VerificationResult {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: VerificationResult) -> VerificationResultBuilder {
        VerificationResultBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> VerificationResultBuilder {
        let mut __value: Value = json!({});
        return VerificationResultBuilder { value: __value };
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut VerificationResultBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut VerificationResultBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _last_performed<'a>(&'a mut self, val: Element) -> &'a mut VerificationResultBuilder {
        self.value["_lastPerformed"] = json!(val.value);
        return self;
    }

    pub fn _next_scheduled<'a>(&'a mut self, val: Element) -> &'a mut VerificationResultBuilder {
        self.value["_nextScheduled"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut VerificationResultBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _status_date<'a>(&'a mut self, val: Element) -> &'a mut VerificationResultBuilder {
        self.value["_statusDate"] = json!(val.value);
        return self;
    }

    pub fn _target_location<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut VerificationResultBuilder {
        self.value["_targetLocation"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn attestation<'a>(
        &'a mut self,
        val: VerificationResult_Attestation,
    ) -> &'a mut VerificationResultBuilder {
        self.value["attestation"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut VerificationResultBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut VerificationResultBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn failure_action<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut VerificationResultBuilder {
        self.value["failureAction"] = json!(val.value);
        return self;
    }

    pub fn frequency<'a>(&'a mut self, val: Timing) -> &'a mut VerificationResultBuilder {
        self.value["frequency"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut VerificationResultBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut VerificationResultBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut VerificationResultBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn last_performed<'a>(&'a mut self, val: &str) -> &'a mut VerificationResultBuilder {
        self.value["lastPerformed"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut VerificationResultBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut VerificationResultBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn need<'a>(&'a mut self, val: CodeableConcept) -> &'a mut VerificationResultBuilder {
        self.value["need"] = json!(val.value);
        return self;
    }

    pub fn next_scheduled<'a>(&'a mut self, val: &str) -> &'a mut VerificationResultBuilder {
        self.value["nextScheduled"] = json!(val);
        return self;
    }

    pub fn primary_source<'a>(
        &'a mut self,
        val: Vec<VerificationResult_PrimarySource>,
    ) -> &'a mut VerificationResultBuilder {
        self.value["primarySource"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut VerificationResultBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn status_date<'a>(&'a mut self, val: &str) -> &'a mut VerificationResultBuilder {
        self.value["statusDate"] = json!(val);
        return self;
    }

    pub fn target<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut VerificationResultBuilder {
        self.value["target"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn target_location<'a>(&'a mut self, val: Vec<&str>) -> &'a mut VerificationResultBuilder {
        self.value["targetLocation"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut VerificationResultBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn validation_process<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut VerificationResultBuilder {
        self.value["validationProcess"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn validation_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut VerificationResultBuilder {
        self.value["validationType"] = json!(val.value);
        return self;
    }

    pub fn validator<'a>(
        &'a mut self,
        val: Vec<VerificationResult_Validator>,
    ) -> &'a mut VerificationResultBuilder {
        self.value["validator"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
