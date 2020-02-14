#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Dosage::Dosage;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::MedicationRequest_DispenseRequest::MedicationRequest_DispenseRequest;
use crate::model::MedicationRequest_Substitution::MedicationRequest_Substitution;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// An order or request for both supply of the medication and the instructions for
/// administration of the medication to a patient. The resource is called
/// "MedicationRequest" rather than "MedicationPrescription" or "MedicationOrder" to
/// generalize the use across inpatient and outpatient settings, including care
/// plans, etc., and to harmonize with workflow patterns.

#[derive(Debug)]
pub struct MedicationRequest<'a> {
    pub value: &'a Value,
}

impl MedicationRequest<'_> {
    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// The reason or the indication for ordering or not ordering the medication.
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

    /// Insurance plans, coverage extensions, pre-authorizations and/or pre-
    /// determinations that may be required for delivering the requested service.
    pub fn insurance(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("insurance") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for reportedBoolean
    pub fn _reported_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_reportedBoolean") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A link to a resource representing the person or set of individuals to whom the
    /// medication will be given.
    pub fn subject(&self) -> Reference {
        Reference {
            value: &self.value["subject"],
        }
    }

    /// The person who entered the order on behalf of another individual for example in
    /// the case of a verbal or a telephone order.
    pub fn recorder(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("recorder") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// A link to a resource representing an earlier order related order or
    /// prescription.
    pub fn prior_prescription(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("priorPrescription") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Indicates an actual or potential clinical issue with or between one or more
    /// active or proposed clinical actions for a patient; e.g. Drug-drug interaction,
    /// duplicate therapy, dosage alert etc.
    pub fn detected_issue(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("detectedIssue") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates the specific details for the dispense or medication supply part of a
    /// medication request (also known as a Medication Prescription or Medication
    /// Order).  Note that this information is not always sent with the order.  There
    /// may be in some settings (e.g. hospitals) institutional or system support for
    /// completing the dispense details in the pharmacy department.
    pub fn dispense_request(&self) -> Option<MedicationRequest_DispenseRequest> {
        if let Some(val) = self.value.get("dispenseRequest") {
            return Some(MedicationRequest_DispenseRequest { value: val });
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

    /// The individual, organization, or device that initiated the request and has
    /// responsibility for its activation.
    pub fn requester(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("requester") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Extensions for priority
    pub fn _priority(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_priority") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Captures the reason for the current state of the MedicationRequest.
    pub fn status_reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("statusReason") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// If true indicates that the provider is asking for the medication request not to
    /// occur.
    pub fn do_not_perform(&self) -> Option<bool> {
        if let Some(val) = self.value.get("doNotPerform") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Indicates how quickly the Medication Request should be addressed with respect to
    /// other requests.
    pub fn priority(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("priority") {
            return Some(string);
        }
        return None;
    }

    /// Identifies the medication being requested. This is a link to a resource that
    /// represents the medication which may be the details of the medication or simply
    /// an attribute carrying a code that identifies the medication from a known list of
    /// medications.
    pub fn medication_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("medicationCodeableConcept") {
            return Some(CodeableConcept { value: val });
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

    /// Include additional information (for example, patient height and weight) that
    /// supports the ordering of the medication.
    pub fn supporting_information(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("supportingInformation") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Condition or observation that supports why the medication was ordered.
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

    /// The URL pointing to an externally maintained protocol, guideline, orderset or
    /// other definition that is adhered to in whole or in part by this
    /// MedicationRequest.
    pub fn instantiates_uri(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("instantiatesUri") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The date (and perhaps time) when the prescription was initially written or
    /// authored on.
    pub fn authored_on(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("authoredOn") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for instantiatesCanonical
    pub fn _instantiates_canonical(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_instantiatesCanonical") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates if this record was captured as a secondary 'reported' record rather
    /// than as an original primary source-of-truth record.  It may also indicate the
    /// source of the report.
    pub fn reported_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("reportedBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// A plan or request that is fulfilled in whole or in part by this medication
    /// request.
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

    /// Indicates the type of medication request (for example, where the medication is
    /// expected to be consumed or administered (i.e. inpatient or outpatient)).
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

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
        }
        return None;
    }

    /// Whether the request is a proposal, plan, or an original order.
    pub fn intent(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("intent") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for instantiatesUri
    pub fn _instantiates_uri(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_instantiatesUri") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Identifiers associated with this medication request that are defined by business
    /// processes and/or used to refer to it when a direct URL reference to the resource
    /// itself is not appropriate. They are business identifiers assigned to this
    /// resource by the performer or other systems and remain constant as the resource
    /// is updated and propagates from server to server.
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

    /// A code specifying the current state of the order.  Generally, this will be
    /// active or completed state.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for intent
    pub fn _intent(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_intent") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The URL pointing to a protocol, guideline, orderset, or other definition that is
    /// adhered to in whole or in part by this MedicationRequest.
    pub fn instantiates_canonical(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("instantiatesCanonical") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates whether or not substitution can or should be part of the dispense. In
    /// some cases, substitution must happen, in other cases substitution must not
    /// happen. This block explains the prescriber's intent. If nothing is specified
    /// substitution may be done.
    pub fn substitution(&self) -> Option<MedicationRequest_Substitution> {
        if let Some(val) = self.value.get("substitution") {
            return Some(MedicationRequest_Substitution { value: val });
        }
        return None;
    }

    /// Indicates the type of performer of the administration of the medication.
    pub fn performer_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("performerType") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Indicates how the medication is to be used by the patient.
    pub fn dosage_instruction(&self) -> Option<Vec<Dosage>> {
        if let Some(Value::Array(val)) = self.value.get("dosageInstruction") {
            return Some(
                val.into_iter()
                    .map(|e| Dosage { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Links to Provenance records for past versions of this resource or fulfilling
    /// request or event resources that identify key state transitions or updates that
    /// are likely to be relevant to a user looking at the current version of the
    /// resource.
    pub fn event_history(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("eventHistory") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A shared identifier common to all requests that were authorized more or less
    /// simultaneously by a single author, representing the identifier of the
    /// requisition or prescription.
    pub fn group_identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("groupIdentifier") {
            return Some(Identifier { value: val });
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

    /// Extra information about the prescription that could not be conveyed by the other
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

    /// The Encounter during which this [x] was created or to which the creation of this
    /// record is tightly associated.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The description of the overall patte3rn of the administration of the medication
    /// to the patient.
    pub fn course_of_therapy_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("courseOfTherapyType") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Indicates if this record was captured as a secondary 'reported' record rather
    /// than as an original primary source-of-truth record.  It may also indicate the
    /// source of the report.
    pub fn reported_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("reportedReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Extensions for doNotPerform
    pub fn _do_not_perform(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_doNotPerform") {
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

    /// Extensions for authoredOn
    pub fn _authored_on(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_authoredOn") {
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

    /// The specified desired performer of the medication treatment (e.g. the performer
    /// of the medication administration).
    pub fn performer(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("performer") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Identifies the medication being requested. This is a link to a resource that
    /// represents the medication which may be the details of the medication or simply
    /// an attribute carrying a code that identifies the medication from a known list of
    /// medications.
    pub fn medication_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("medicationReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.reason_code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.insurance() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._reported_boolean() {
            _val.validate();
        }
        let _ = self.subject().validate();
        if let Some(_val) = self.recorder() {
            _val.validate();
        }
        if let Some(_val) = self.prior_prescription() {
            _val.validate();
        }
        if let Some(_val) = self.detected_issue() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.dispense_request() {
            _val.validate();
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.requester() {
            _val.validate();
        }
        if let Some(_val) = self._priority() {
            _val.validate();
        }
        if let Some(_val) = self.status_reason() {
            _val.validate();
        }
        if let Some(_val) = self.do_not_perform() {}
        if let Some(_val) = self.priority() {}
        if let Some(_val) = self.medication_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.supporting_information() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.reason_reference() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.instantiates_uri() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.authored_on() {}
        if let Some(_val) = self._instantiates_canonical() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.reported_boolean() {}
        if let Some(_val) = self.based_on() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
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
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.intent() {}
        if let Some(_val) = self._instantiates_uri() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self._intent() {
            _val.validate();
        }
        if let Some(_val) = self.instantiates_canonical() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.substitution() {
            _val.validate();
        }
        if let Some(_val) = self.performer_type() {
            _val.validate();
        }
        if let Some(_val) = self.dosage_instruction() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.event_history() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.group_identifier() {
            _val.validate();
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.note() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.encounter() {
            _val.validate();
        }
        if let Some(_val) = self.course_of_therapy_type() {
            _val.validate();
        }
        if let Some(_val) = self.reported_reference() {
            _val.validate();
        }
        if let Some(_val) = self._do_not_perform() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self._authored_on() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.performer() {
            _val.validate();
        }
        if let Some(_val) = self.medication_reference() {
            _val.validate();
        }
        return true;
    }
}
