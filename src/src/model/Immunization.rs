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
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Describes the event of a patient being administered a vaccine or a record of an
/// immunization as reported by a patient, a clinician or another party.

#[derive(Debug)]
pub struct Immunization<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Immunization<'_> {
    pub fn new(value: &Value) -> Immunization {
        Immunization {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for expirationDate
    pub fn _expiration_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_expirationDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
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

    /// Extensions for isSubpotent
    pub fn _is_subpotent(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_isSubpotent") {
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

    /// Extensions for lotNumber
    pub fn _lot_number(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lotNumber") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for occurrenceDateTime
    pub fn _occurrence_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_occurrenceDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for occurrenceString
    pub fn _occurrence_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_occurrenceString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for primarySource
    pub fn _primary_source(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_primarySource") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for recorded
    pub fn _recorded(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_recorded") {
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

    /// The quantity of vaccine product that was administered.
    pub fn dose_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("doseQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Educational material presented to the patient (or guardian) at the time of
    /// vaccine administration.
    pub fn education(&self) -> Option<Vec<Immunization_Education>> {
        if let Some(Value::Array(val)) = self.value.get("education") {
            return Some(
                val.into_iter()
                    .map(|e| Immunization_Education {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The visit or admission or other contact between patient and health care provider
    /// the immunization was performed as part of.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// Indicates the source of the vaccine actually administered. This may be different
    /// than the patient eligibility (e.g. the patient may be eligible for a publically
    /// purchased vaccine but due to inventory issues, vaccine purchased with private
    /// funds was actually administered).
    pub fn funding_source(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("fundingSource") {
            return Some(CodeableConcept {
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

    /// A unique identifier assigned to this immunization record.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier {
                        value: Cow::Borrowed(e),
                    })
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

    /// Indication if a dose is considered to be subpotent. By default, a dose should be
    /// considered to be potent.
    pub fn is_subpotent(&self) -> Option<bool> {
        if let Some(val) = self.value.get("isSubpotent") {
            return Some(val.as_bool().unwrap());
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

    /// The service delivery location where the vaccine administration occurred.
    pub fn location(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("location") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// Name of vaccine manufacturer.
    pub fn manufacturer(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("manufacturer") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// Extra information about the immunization that is not conveyed by the other
    /// attributes.
    pub fn note(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("note") {
            return Some(
                val.into_iter()
                    .map(|e| Annotation {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// Date vaccine administered or was to be administered.
    pub fn occurrence_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("occurrenceString") {
            return Some(string);
        }
        return None;
    }

    /// The patient who either received or did not receive the immunization.
    pub fn patient(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["patient"]),
        }
    }

    /// Indicates who performed the immunization event.
    pub fn performer(&self) -> Option<Vec<Immunization_Performer>> {
        if let Some(Value::Array(val)) = self.value.get("performer") {
            return Some(
                val.into_iter()
                    .map(|e| Immunization_Performer {
                        value: Cow::Borrowed(e),
                    })
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

    /// Indicates a patient's eligibility for a funding program.
    pub fn program_eligibility(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("programEligibility") {
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

    /// The protocol (set of recommendations) being followed by the provider who
    /// administered the dose.
    pub fn protocol_applied(&self) -> Option<Vec<Immunization_ProtocolApplied>> {
        if let Some(Value::Array(val)) = self.value.get("protocolApplied") {
            return Some(
                val.into_iter()
                    .map(|e| Immunization_ProtocolApplied {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Categorical data indicating that an adverse event is associated in time to an
    /// immunization.
    pub fn reaction(&self) -> Option<Vec<Immunization_Reaction>> {
        if let Some(Value::Array(val)) = self.value.get("reaction") {
            return Some(
                val.into_iter()
                    .map(|e| Immunization_Reaction {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Reasons why the vaccine was administered.
    pub fn reason_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("reasonCode") {
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

    /// Condition, Observation or DiagnosticReport that supports why the immunization
    /// was administered.
    pub fn reason_reference(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("reasonReference") {
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

    /// The date the occurrence of the immunization was first captured in the record -
    /// potentially significantly after the occurrence of the event.
    pub fn recorded(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("recorded") {
            return Some(string);
        }
        return None;
    }

    /// The source of the data when the report of the immunization event is not based on
    /// information from the person who administered the vaccine.
    pub fn report_origin(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("reportOrigin") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The path by which the vaccine product is taken into the body.
    pub fn route(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("route") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Body site where vaccine was administered.
    pub fn site(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("site") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// Indicates the reason the immunization event was not performed.
    pub fn status_reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("statusReason") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Reason why a dose is considered to be subpotent.
    pub fn subpotent_reason(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("subpotentReason") {
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

    /// Vaccine that was administered or was to be administered.
    pub fn vaccine_code(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["vaccineCode"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._expiration_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._is_subpotent() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._lot_number() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._occurrence_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._occurrence_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._primary_source() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._recorded() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.dose_quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.education() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.encounter() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.expiration_date() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.funding_source() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.is_subpotent() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.location() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.lot_number() {}
        if let Some(_val) = self.manufacturer() {
            if !_val.validate() {
                return false;
            }
        }
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
        if let Some(_val) = self.note() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.occurrence_date_time() {}
        if let Some(_val) = self.occurrence_string() {}
        if !self.patient().validate() {
            return false;
        }
        if let Some(_val) = self.performer() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.primary_source() {}
        if let Some(_val) = self.program_eligibility() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.protocol_applied() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.reaction() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.reason_code() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.reason_reference() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.recorded() {}
        if let Some(_val) = self.report_origin() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.route() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.site() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.status_reason() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.subpotent_reason() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.vaccine_code().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ImmunizationBuilder {
    pub(crate) value: Value,
}

impl ImmunizationBuilder {
    pub fn build(&self) -> Immunization {
        Immunization {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Immunization) -> ImmunizationBuilder {
        ImmunizationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(patient: Reference, vaccine_code: CodeableConcept) -> ImmunizationBuilder {
        let mut __value: Value = json!({});
        __value["patient"] = json!(patient.value);
        __value["vaccineCode"] = json!(vaccine_code.value);
        return ImmunizationBuilder { value: __value };
    }

    pub fn _expiration_date<'a>(&'a mut self, val: Element) -> &'a mut ImmunizationBuilder {
        self.value["_expirationDate"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut ImmunizationBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _is_subpotent<'a>(&'a mut self, val: Element) -> &'a mut ImmunizationBuilder {
        self.value["_isSubpotent"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut ImmunizationBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _lot_number<'a>(&'a mut self, val: Element) -> &'a mut ImmunizationBuilder {
        self.value["_lotNumber"] = json!(val.value);
        return self;
    }

    pub fn _occurrence_date_time<'a>(&'a mut self, val: Element) -> &'a mut ImmunizationBuilder {
        self.value["_occurrenceDateTime"] = json!(val.value);
        return self;
    }

    pub fn _occurrence_string<'a>(&'a mut self, val: Element) -> &'a mut ImmunizationBuilder {
        self.value["_occurrenceString"] = json!(val.value);
        return self;
    }

    pub fn _primary_source<'a>(&'a mut self, val: Element) -> &'a mut ImmunizationBuilder {
        self.value["_primarySource"] = json!(val.value);
        return self;
    }

    pub fn _recorded<'a>(&'a mut self, val: Element) -> &'a mut ImmunizationBuilder {
        self.value["_recorded"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut ImmunizationBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut ImmunizationBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn dose_quantity<'a>(&'a mut self, val: Quantity) -> &'a mut ImmunizationBuilder {
        self.value["doseQuantity"] = json!(val.value);
        return self;
    }

    pub fn education<'a>(
        &'a mut self,
        val: Vec<Immunization_Education>,
    ) -> &'a mut ImmunizationBuilder {
        self.value["education"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn encounter<'a>(&'a mut self, val: Reference) -> &'a mut ImmunizationBuilder {
        self.value["encounter"] = json!(val.value);
        return self;
    }

    pub fn expiration_date<'a>(&'a mut self, val: &str) -> &'a mut ImmunizationBuilder {
        self.value["expirationDate"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ImmunizationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn funding_source<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ImmunizationBuilder {
        self.value["fundingSource"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ImmunizationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut ImmunizationBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut ImmunizationBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn is_subpotent<'a>(&'a mut self, val: bool) -> &'a mut ImmunizationBuilder {
        self.value["isSubpotent"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut ImmunizationBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn location<'a>(&'a mut self, val: Reference) -> &'a mut ImmunizationBuilder {
        self.value["location"] = json!(val.value);
        return self;
    }

    pub fn lot_number<'a>(&'a mut self, val: &str) -> &'a mut ImmunizationBuilder {
        self.value["lotNumber"] = json!(val);
        return self;
    }

    pub fn manufacturer<'a>(&'a mut self, val: Reference) -> &'a mut ImmunizationBuilder {
        self.value["manufacturer"] = json!(val.value);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut ImmunizationBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImmunizationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut ImmunizationBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn occurrence_date_time<'a>(&'a mut self, val: &str) -> &'a mut ImmunizationBuilder {
        self.value["occurrenceDateTime"] = json!(val);
        return self;
    }

    pub fn occurrence_string<'a>(&'a mut self, val: &str) -> &'a mut ImmunizationBuilder {
        self.value["occurrenceString"] = json!(val);
        return self;
    }

    pub fn performer<'a>(
        &'a mut self,
        val: Vec<Immunization_Performer>,
    ) -> &'a mut ImmunizationBuilder {
        self.value["performer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn primary_source<'a>(&'a mut self, val: bool) -> &'a mut ImmunizationBuilder {
        self.value["primarySource"] = json!(val);
        return self;
    }

    pub fn program_eligibility<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ImmunizationBuilder {
        self.value["programEligibility"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn protocol_applied<'a>(
        &'a mut self,
        val: Vec<Immunization_ProtocolApplied>,
    ) -> &'a mut ImmunizationBuilder {
        self.value["protocolApplied"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reaction<'a>(
        &'a mut self,
        val: Vec<Immunization_Reaction>,
    ) -> &'a mut ImmunizationBuilder {
        self.value["reaction"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reason_code<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut ImmunizationBuilder {
        self.value["reasonCode"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reason_reference<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ImmunizationBuilder {
        self.value["reasonReference"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn recorded<'a>(&'a mut self, val: &str) -> &'a mut ImmunizationBuilder {
        self.value["recorded"] = json!(val);
        return self;
    }

    pub fn report_origin<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ImmunizationBuilder {
        self.value["reportOrigin"] = json!(val.value);
        return self;
    }

    pub fn route<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ImmunizationBuilder {
        self.value["route"] = json!(val.value);
        return self;
    }

    pub fn site<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ImmunizationBuilder {
        self.value["site"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut ImmunizationBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn status_reason<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ImmunizationBuilder {
        self.value["statusReason"] = json!(val.value);
        return self;
    }

    pub fn subpotent_reason<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ImmunizationBuilder {
        self.value["subpotentReason"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut ImmunizationBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }
}
