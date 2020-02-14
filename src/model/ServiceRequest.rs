#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Quantity::Quantity;
use crate::model::Range::Range;
use crate::model::Ratio::Ratio;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::Timing::Timing;
use serde_json::value::Value;

/// A record of a request for service such as diagnostic investigations, treatments,
/// or operations to be performed.

#[derive(Debug)]
pub struct ServiceRequest<'a> {
    pub value: &'a Value,
}

impl ServiceRequest<'_> {
    /// Indicates another resource that provides a justification for why this service is
    /// being requested.   May relate to the resources referred to in `supportingInfo`.
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

    /// Any other notes and comments made about the service request. For example,
    /// internal billing notes.
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

    /// Whether the request is a proposal, plan, an original order or a reflex order.
    pub fn intent(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("intent") {
            return Some(string);
        }
        return None;
    }

    /// The URL pointing to an externally maintained protocol, guideline, orderset or
    /// other definition that is adhered to in whole or in part by this ServiceRequest.
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

    /// Additional details and instructions about the how the services are to be
    /// delivered.   For example, and order for a urinary catheter may have an order
    /// detail for an external or indwelling catheter, or an order for a bandage may
    /// require additional instructions specifying how the bandage should be applied.
    pub fn order_detail(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("orderDetail") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// An amount of service being requested which can be a quantity ( for example
    /// $1,500 home modification), a ratio ( for example, 20 half day visits per month),
    /// or a range (2.0 to 1.8 Gy per fraction).
    pub fn quantity_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantityQuantity") {
            return Some(Quantity { value: val });
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

    /// The date/time at which the requested service should occur.
    pub fn occurrence_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("occurrenceDateTime") {
            return Some(string);
        }
        return None;
    }

    /// When the request transitioned to being actionable.
    pub fn authored_on(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("authoredOn") {
            return Some(string);
        }
        return None;
    }

    /// The status of the order.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// If a CodeableConcept is present, it indicates the pre-condition for performing
    /// the service.  For example "pain", "on flare-up", etc.
    pub fn as_needed_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("asNeededBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The URL pointing to a FHIR-defined protocol, guideline, orderset or other
    /// definition that is adhered to in whole or in part by this ServiceRequest.
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

    /// Indicates how quickly the ServiceRequest should be addressed with respect to
    /// other requests.
    pub fn priority(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("priority") {
            return Some(string);
        }
        return None;
    }

    /// The date/time at which the requested service should occur.
    pub fn occurrence_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("occurrenceTiming") {
            return Some(Timing { value: val });
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

    /// The preferred location(s) where the procedure should actually happen in coded or
    /// free text form. E.g. at home or nursing day care center.
    pub fn location_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("locationCode") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A reference to the the preferred location(s) where the procedure should actually
    /// happen. E.g. at home or nursing day care center.
    pub fn location_reference(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("locationReference") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
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

    /// One or more specimens that the laboratory procedure will use.
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

    /// Plan/proposal/order fulfilled by this request.
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

    /// A code that identifies a particular service (i.e., procedure, diagnostic
    /// investigation, or panel of investigations) that have been requested.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Set this to true if the record is saying that the service/procedure should NOT
    /// be performed.
    pub fn do_not_perform(&self) -> Option<bool> {
        if let Some(val) = self.value.get("doNotPerform") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The request takes the place of the referenced completed or terminated
    /// request(s).
    pub fn replaces(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("replaces") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for patientInstruction
    pub fn _patient_instruction(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patientInstruction") {
            return Some(Element { value: val });
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

    /// Insurance plans, coverage extensions, pre-authorizations and/or pre-
    /// determinations that may be needed for delivering the requested service.
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

    /// An encounter that provides additional information about the healthcare context
    /// in which this request is made.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// An amount of service being requested which can be a quantity ( for example
    /// $1,500 home modification), a ratio ( for example, 20 half day visits per month),
    /// or a range (2.0 to 1.8 Gy per fraction).
    pub fn quantity_ratio(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("quantityRatio") {
            return Some(Ratio { value: val });
        }
        return None;
    }

    /// Instructions in terms that are understood by the patient or consumer.
    pub fn patient_instruction(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("patientInstruction") {
            return Some(string);
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

    /// Extensions for doNotPerform
    pub fn _do_not_perform(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_doNotPerform") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Additional clinical information about the patient or specimen that may influence
    /// the services or their interpretations.     This information includes diagnosis,
    /// clinical findings and other observations.  In laboratory ordering these are
    /// typically referred to as "ask at order entry questions (AOEs)".  This includes
    /// observations explicitly requested by the producer (filler) to provide context or
    /// supporting information needed to complete the order. For example,  reporting the
    /// amount of inspired oxygen for blood gas measurements.
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

    /// A shared identifier common to all service requests that were authorized more or
    /// less simultaneously by a single author, representing the composite or group
    /// identifier.
    pub fn requisition(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("requisition") {
            return Some(Identifier { value: val });
        }
        return None;
    }

    /// An amount of service being requested which can be a quantity ( for example
    /// $1,500 home modification), a ratio ( for example, 20 half day visits per month),
    /// or a range (2.0 to 1.8 Gy per fraction).
    pub fn quantity_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("quantityRange") {
            return Some(Range { value: val });
        }
        return None;
    }

    /// The desired performer for doing the requested service.  For example, the
    /// surgeon, dermatopathologist, endoscopist, etc.
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

    /// On whom or what the service is to be performed. This is usually a human patient,
    /// but can also be requested on animals, groups of humans or animals, devices such
    /// as dialysis machines, or even locations (typically for environmental scans).
    pub fn subject(&self) -> Reference {
        Reference {
            value: &self.value["subject"],
        }
    }

    /// Key events in the history of the request.
    pub fn relevant_history(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("relevantHistory") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Anatomic location where the procedure should be performed. This is the target
    /// site.
    pub fn body_site(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("bodySite") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Identifiers assigned to this order instance by the orderer and/or the receiver
    /// and/or order fulfiller.
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

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A code that classifies the service for searching, sorting and display purposes
    /// (e.g. "Surgical Procedure").
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

    /// The date/time at which the requested service should occur.
    pub fn occurrence_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("occurrencePeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Desired type of performer for doing the requested service.
    pub fn performer_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("performerType") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// If a CodeableConcept is present, it indicates the pre-condition for performing
    /// the service.  For example "pain", "on flare-up", etc.
    pub fn as_needed_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("asNeededCodeableConcept") {
            return Some(CodeableConcept { value: val });
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

    /// An explanation or justification for why this service is being requested in coded
    /// or textual form.   This is often for billing purposes.  May relate to the
    /// resources referred to in `supportingInfo`.
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

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
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

    /// Extensions for asNeededBoolean
    pub fn _as_needed_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_asNeededBoolean") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The individual who initiated the request and has responsibility for its
    /// activation.
    pub fn requester(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("requester") {
            return Some(Reference { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.reason_reference() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.note() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.intent() {}
        if let Some(_val) = self.instantiates_uri() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.order_detail() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._priority() {
            _val.validate();
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.quantity_quantity() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.occurrence_date_time() {}
        if let Some(_val) = self.authored_on() {}
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.as_needed_boolean() {}
        if let Some(_val) = self.instantiates_canonical() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.priority() {}
        if let Some(_val) = self.occurrence_timing() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.location_code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.location_reference() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.specimen() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.based_on() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.code() {
            _val.validate();
        }
        if let Some(_val) = self.do_not_perform() {}
        if let Some(_val) = self.replaces() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._patient_instruction() {
            _val.validate();
        }
        if let Some(_val) = self._authored_on() {
            _val.validate();
        }
        if let Some(_val) = self.insurance() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.encounter() {
            _val.validate();
        }
        if let Some(_val) = self.quantity_ratio() {
            _val.validate();
        }
        if let Some(_val) = self.patient_instruction() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self._do_not_perform() {
            _val.validate();
        }
        if let Some(_val) = self.supporting_info() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.requisition() {
            _val.validate();
        }
        if let Some(_val) = self.quantity_range() {
            _val.validate();
        }
        if let Some(_val) = self.performer() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.subject().validate();
        if let Some(_val) = self.relevant_history() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._intent() {
            _val.validate();
        }
        if let Some(_val) = self.body_site() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.category() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.occurrence_period() {
            _val.validate();
        }
        if let Some(_val) = self.performer_type() {
            _val.validate();
        }
        if let Some(_val) = self.as_needed_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self._instantiates_uri() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.reason_code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self._occurrence_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._as_needed_boolean() {
            _val.validate();
        }
        if let Some(_val) = self.requester() {
            _val.validate();
        }
        return true;
    }
}
