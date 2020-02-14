#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::DeviceRequest_Parameter::DeviceRequest_Parameter;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::Timing::Timing;
use serde_json::value::Value;

/// Represents a request for a patient to employ a medical device. The device may be
/// an implantable device, or an external assistive device, such as a walker.

#[derive(Debug)]
pub struct DeviceRequest<'a> {
    pub value: &'a Value,
}

impl DeviceRequest<'_> {
    /// The details of the device to be used.
    pub fn code_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("codeReference") {
            return Some(Reference { value: val });
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

    /// The individual who initiated the request and has responsibility for its
    /// activation.
    pub fn requester(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("requester") {
            return Some(Reference { value: val });
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

    /// The timing schedule for the use of the device. The Schedule data type allows
    /// many different expressions, for example. "Every 8 hours"; "Three times a day";
    /// "1/2 an hour before breakfast for 10 days from 23-Dec 2011:"; "15 Oct 2013, 17
    /// Oct 2013 and 1 Nov 2013".
    pub fn occurrence_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("occurrenceDateTime") {
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

    /// Indicates how quickly the {{title}} should be addressed with respect to other
    /// requests.
    pub fn priority(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("priority") {
            return Some(string);
        }
        return None;
    }

    /// The timing schedule for the use of the device. The Schedule data type allows
    /// many different expressions, for example. "Every 8 hours"; "Three times a day";
    /// "1/2 an hour before breakfast for 10 days from 23-Dec 2011:"; "15 Oct 2013, 17
    /// Oct 2013 and 1 Nov 2013".
    pub fn occurrence_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("occurrenceTiming") {
            return Some(Timing { value: val });
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

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Details about this request that were not represented at all or sufficiently in
    /// one of the attributes provided in a class. These may include for example a
    /// comment, an instruction, or a note associated with the statement.
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

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
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

    /// The URL pointing to a FHIR-defined protocol, guideline, orderset or other
    /// definition that is adhered to in whole or in part by this DeviceRequest.
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

    /// The details of the device to be used.
    pub fn code_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("codeCodeableConcept") {
            return Some(CodeableConcept { value: val });
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

    /// Extensions for intent
    pub fn _intent(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_intent") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Identifiers assigned to this order by the orderer or by the receiver.
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

    /// The patient who will use the device.
    pub fn subject(&self) -> Reference {
        Reference {
            value: &self.value["subject"],
        }
    }

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// The URL pointing to an externally maintained protocol, guideline, orderset or
    /// other definition that is adhered to in whole or in part by this DeviceRequest.
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

    /// Composite request this is part of.
    pub fn group_identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("groupIdentifier") {
            return Some(Identifier { value: val });
        }
        return None;
    }

    /// The desired performer for doing the diagnostic testing.
    pub fn performer(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("performer") {
            return Some(Reference { value: val });
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

    /// An encounter that provides additional context in which this request is made.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference { value: val });
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

    /// Additional clinical information about the patient that may influence the request
    /// fulfilment.  For example, this may include where on the subject's body the
    /// device will be used (i.e. the target site).
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

    /// The request takes the place of the referenced completed or terminated
    /// request(s).
    pub fn prior_request(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("priorRequest") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// The timing schedule for the use of the device. The Schedule data type allows
    /// many different expressions, for example. "Every 8 hours"; "Three times a day";
    /// "1/2 an hour before breakfast for 10 days from 23-Dec 2011:"; "15 Oct 2013, 17
    /// Oct 2013 and 1 Nov 2013".
    pub fn occurrence_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("occurrencePeriod") {
            return Some(Period { value: val });
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

    /// Extensions for authoredOn
    pub fn _authored_on(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_authoredOn") {
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

    /// The status of the request.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
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

    /// Whether the request is a proposal, plan, an original order or a reflex order.
    pub fn intent(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("intent") {
            return Some(string);
        }
        return None;
    }

    /// Desired type of performer for doing the diagnostic testing.
    pub fn performer_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("performerType") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Reason or justification for the use of this device.
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

    /// Reason or justification for the use of this device.
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

    /// Specific parameters for the ordered item.  For example, the prism value for
    /// lenses.
    pub fn parameter(&self) -> Option<Vec<DeviceRequest_Parameter>> {
        if let Some(Value::Array(val)) = self.value.get("parameter") {
            return Some(
                val.into_iter()
                    .map(|e| DeviceRequest_Parameter { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.code_reference() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.requester() {
            _val.validate();
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.relevant_history() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.occurrence_date_time() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.priority() {}
        if let Some(_val) = self.occurrence_timing() {
            _val.validate();
        }
        if let Some(_val) = self.authored_on() {}
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.note() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.instantiates_canonical() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.code_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self._priority() {
            _val.validate();
        }
        if let Some(_val) = self._intent() {
            _val.validate();
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.subject().validate();
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.instantiates_uri() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.insurance() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.group_identifier() {
            _val.validate();
        }
        if let Some(_val) = self.performer() {
            _val.validate();
        }
        if let Some(_val) = self.based_on() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.encounter() {
            _val.validate();
        }
        if let Some(_val) = self._occurrence_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.supporting_info() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.prior_request() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._instantiates_uri() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.occurrence_period() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self._authored_on() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.intent() {}
        if let Some(_val) = self.performer_type() {
            _val.validate();
        }
        if let Some(_val) = self.reason_code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.reason_reference() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.parameter() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
