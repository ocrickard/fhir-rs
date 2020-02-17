#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Dosage::Dosage;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::MedicationDispense_Performer::MedicationDispense_Performer;
use crate::model::MedicationDispense_Substitution::MedicationDispense_Substitution;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Indicates that a medication product is to be or has been dispensed for a named
/// person/patient.  This includes a description of the medication product (supply)
/// provided and the instructions for administering the medication.  The medication
/// dispense is the result of a pharmacy system responding to a medication order.

#[derive(Debug)]
pub struct MedicationDispense<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicationDispense<'_> {
    pub fn new(value: &Value) -> MedicationDispense {
        MedicationDispense {
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

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for whenHandedOver
    pub fn _when_handed_over(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_whenHandedOver") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for whenPrepared
    pub fn _when_prepared(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_whenPrepared") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates the medication order that is being dispensed against.
    pub fn authorizing_prescription(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("authorizingPrescription") {
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

    /// Indicates the type of medication dispense (for example, where the medication is
    /// expected to be consumed or administered (i.e. inpatient or outpatient)).
    pub fn category(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("category") {
            return Some(CodeableConcept {
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

    /// The encounter or episode of care that establishes the context for this event.
    pub fn context(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("context") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The amount of medication expressed as a timing amount.
    pub fn days_supply(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("daysSupply") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identification of the facility/location where the medication was shipped to, as
    /// part of the dispense event.
    pub fn destination(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("destination") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates an actual or potential clinical issue with or between one or more
    /// active or proposed clinical actions for a patient; e.g. drug-drug interaction,
    /// duplicate therapy, dosage alert etc.
    pub fn detected_issue(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("detectedIssue") {
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

    /// Indicates how the medication is to be used by the patient.
    pub fn dosage_instruction(&self) -> Option<Vec<Dosage>> {
        if let Some(Value::Array(val)) = self.value.get("dosageInstruction") {
            return Some(
                val.into_iter()
                    .map(|e| Dosage {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A summary of the events of interest that have occurred, such as when the
    /// dispense was verified.
    pub fn event_history(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("eventHistory") {
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Identifiers associated with this Medication Dispense that are defined by
    /// business processes and/or used to refer to it when a direct URL reference to the
    /// resource itself is not appropriate. They are business identifiers assigned to
    /// this resource by the performer or other systems and remain constant as the
    /// resource is updated and propagates from server to server.
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The principal physical location where the dispense was performed.
    pub fn location(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("location") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifies the medication being administered. This is either a link to a
    /// resource representing the details of the medication or a simple attribute
    /// carrying a code that identifies the medication from a known list of medications.
    pub fn medication_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("medicationCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifies the medication being administered. This is either a link to a
    /// resource representing the details of the medication or a simple attribute
    /// carrying a code that identifies the medication from a known list of medications.
    pub fn medication_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("medicationReference") {
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

    /// Extra information about the dispense that could not be conveyed in the other
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

    /// The procedure that trigger the dispense.
    pub fn part_of(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("partOf") {
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

    /// Indicates who or what performed the event.
    pub fn performer(&self) -> Option<Vec<MedicationDispense_Performer>> {
        if let Some(Value::Array(val)) = self.value.get("performer") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationDispense_Performer {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The amount of medication that has been dispensed. Includes unit of measure.
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifies the person who picked up the medication.  This will usually be a
    /// patient or their caregiver, but some cases exist where it can be a healthcare
    /// professional.
    pub fn receiver(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("receiver") {
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

    /// A code specifying the state of the set of dispense events.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// Indicates the reason why a dispense was not performed.
    pub fn status_reason_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("statusReasonCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates the reason why a dispense was not performed.
    pub fn status_reason_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("statusReasonReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A link to a resource representing the person or the group to whom the medication
    /// will be given.
    pub fn subject(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subject") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates whether or not substitution was made as part of the dispense.  In some
    /// cases, substitution will be expected but does not happen, in other cases
    /// substitution is not expected but does happen.  This block explains what
    /// substitution did or did not happen and why.  If nothing is specified,
    /// substitution was not done.
    pub fn substitution(&self) -> Option<MedicationDispense_Substitution> {
        if let Some(val) = self.value.get("substitution") {
            return Some(MedicationDispense_Substitution {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Additional information that supports the medication being dispensed.
    pub fn supporting_information(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("supportingInformation") {
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

    /// Indicates the type of dispensing event that is performed. For example, Trial
    /// Fill, Completion of Trial, Partial Fill, Emergency Fill, Samples, etc.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The time the dispensed product was provided to the patient or their
    /// representative.
    pub fn when_handed_over(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("whenHandedOver") {
            return Some(string);
        }
        return None;
    }

    /// The time when the dispensed product was packaged and reviewed.
    pub fn when_prepared(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("whenPrepared") {
            return Some(string);
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
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._when_handed_over() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._when_prepared() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.authorizing_prescription() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.category() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.context() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.days_supply() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.destination() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.detected_issue() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.dosage_instruction() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.event_history() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.location() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.medication_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.medication_reference() {
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
        if let Some(_val) = self.part_of() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.performer() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.receiver() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.status_reason_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status_reason_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.subject() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.substitution() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.supporting_information() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.when_handed_over() {}
        if let Some(_val) = self.when_prepared() {}
        return true;
    }
}

#[derive(Debug)]
pub struct MedicationDispenseBuilder {
    pub(crate) value: Value,
}

impl MedicationDispenseBuilder {
    pub fn build(&self) -> MedicationDispense {
        MedicationDispense {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: MedicationDispense) -> MedicationDispenseBuilder {
        MedicationDispenseBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MedicationDispenseBuilder {
        let mut __value: Value = json!({});
        return MedicationDispenseBuilder { value: __value };
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut MedicationDispenseBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut MedicationDispenseBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut MedicationDispenseBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _when_handed_over<'a>(&'a mut self, val: Element) -> &'a mut MedicationDispenseBuilder {
        self.value["_whenHandedOver"] = json!(val.value);
        return self;
    }

    pub fn _when_prepared<'a>(&'a mut self, val: Element) -> &'a mut MedicationDispenseBuilder {
        self.value["_whenPrepared"] = json!(val.value);
        return self;
    }

    pub fn authorizing_prescription<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut MedicationDispenseBuilder {
        self.value["authorizingPrescription"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn category<'a>(&'a mut self, val: CodeableConcept) -> &'a mut MedicationDispenseBuilder {
        self.value["category"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut MedicationDispenseBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn context<'a>(&'a mut self, val: Reference) -> &'a mut MedicationDispenseBuilder {
        self.value["context"] = json!(val.value);
        return self;
    }

    pub fn days_supply<'a>(&'a mut self, val: Quantity) -> &'a mut MedicationDispenseBuilder {
        self.value["daysSupply"] = json!(val.value);
        return self;
    }

    pub fn destination<'a>(&'a mut self, val: Reference) -> &'a mut MedicationDispenseBuilder {
        self.value["destination"] = json!(val.value);
        return self;
    }

    pub fn detected_issue<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut MedicationDispenseBuilder {
        self.value["detectedIssue"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn dosage_instruction<'a>(
        &'a mut self,
        val: Vec<Dosage>,
    ) -> &'a mut MedicationDispenseBuilder {
        self.value["dosageInstruction"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn event_history<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut MedicationDispenseBuilder {
        self.value["eventHistory"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut MedicationDispenseBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicationDispenseBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut MedicationDispenseBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut MedicationDispenseBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut MedicationDispenseBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn location<'a>(&'a mut self, val: Reference) -> &'a mut MedicationDispenseBuilder {
        self.value["location"] = json!(val.value);
        return self;
    }

    pub fn medication_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicationDispenseBuilder {
        self.value["medicationCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn medication_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut MedicationDispenseBuilder {
        self.value["medicationReference"] = json!(val.value);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut MedicationDispenseBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationDispenseBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut MedicationDispenseBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn part_of<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut MedicationDispenseBuilder {
        self.value["partOf"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn performer<'a>(
        &'a mut self,
        val: Vec<MedicationDispense_Performer>,
    ) -> &'a mut MedicationDispenseBuilder {
        self.value["performer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn quantity<'a>(&'a mut self, val: Quantity) -> &'a mut MedicationDispenseBuilder {
        self.value["quantity"] = json!(val.value);
        return self;
    }

    pub fn receiver<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut MedicationDispenseBuilder {
        self.value["receiver"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut MedicationDispenseBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn status_reason_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicationDispenseBuilder {
        self.value["statusReasonCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn status_reason_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut MedicationDispenseBuilder {
        self.value["statusReasonReference"] = json!(val.value);
        return self;
    }

    pub fn subject<'a>(&'a mut self, val: Reference) -> &'a mut MedicationDispenseBuilder {
        self.value["subject"] = json!(val.value);
        return self;
    }

    pub fn substitution<'a>(
        &'a mut self,
        val: MedicationDispense_Substitution,
    ) -> &'a mut MedicationDispenseBuilder {
        self.value["substitution"] = json!(val.value);
        return self;
    }

    pub fn supporting_information<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut MedicationDispenseBuilder {
        self.value["supportingInformation"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut MedicationDispenseBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut MedicationDispenseBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }

    pub fn when_handed_over<'a>(&'a mut self, val: &str) -> &'a mut MedicationDispenseBuilder {
        self.value["whenHandedOver"] = json!(val);
        return self;
    }

    pub fn when_prepared<'a>(&'a mut self, val: &str) -> &'a mut MedicationDispenseBuilder {
        self.value["whenPrepared"] = json!(val);
        return self;
    }
}
