#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Communication_Payload::Communication_Payload;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// An occurrence of information being transmitted; e.g. an alert that was sent to a
/// responsible provider, a public health agency that was notified about a
/// reportable condition.

#[derive(Debug)]
pub struct Communication<'a> {
    pub value: &'a Value,
}

impl Communication<'_> {
    /// The status of the transmission.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
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

    /// The Encounter during which this Communication was created or to which the
    /// creation of this record is tightly associated.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
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

    /// Extensions for sent
    pub fn _sent(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sent") {
            return Some(Element { value: val });
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

    /// The entity (e.g. person, organization, clinical information system, care team or
    /// device) which was the target of the communication. If receipts need to be
    /// tracked by an individual, a separate resource instance will need to be created
    /// for each recipient.  Multiple recipient communications are intended where either
    /// receipts are not tracked (e.g. a mass mail-out) or a receipt is captured in
    /// aggregate (all emails confirmed received by a particular time).
    pub fn recipient(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("recipient") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The type of message conveyed such as alert, notification, reminder, instruction,
    /// etc.
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

    /// The URL pointing to a FHIR-defined protocol, guideline, orderset or other
    /// definition that is adhered to in whole or in part by this Communication.
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

    /// Characterizes how quickly the planned or in progress communication must be
    /// addressed. Includes concepts such as stat, urgent, routine.
    pub fn priority(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("priority") {
            return Some(string);
        }
        return None;
    }

    /// Additional notes or commentary about the communication by the sender, receiver
    /// or other interested parties.
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

    /// Text, attachment(s), or resource(s) that was communicated to the recipient.
    pub fn payload(&self) -> Option<Vec<Communication_Payload>> {
        if let Some(Value::Array(val)) = self.value.get("payload") {
            return Some(
                val.into_iter()
                    .map(|e| Communication_Payload { value: e })
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

    /// The time when this communication was sent.
    pub fn sent(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("sent") {
            return Some(string);
        }
        return None;
    }

    /// The entity (e.g. person, organization, clinical information system, or device)
    /// which was the source of the communication.
    pub fn sender(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("sender") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Extensions for received
    pub fn _received(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_received") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Indicates another resource whose existence justifies this communication.
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

    /// Part of this action.
    pub fn part_of(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("partOf") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An order, proposal or plan fulfilled in whole or in part by this Communication.
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

    /// Captures the reason for the current state of the Communication.
    pub fn status_reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("statusReason") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The time when this communication arrived at the destination.
    pub fn received(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("received") {
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

    /// The URL pointing to an externally maintained protocol, guideline, orderset or
    /// other definition that is adhered to in whole or in part by this Communication.
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

    /// Prior communication that this communication is in response to.
    pub fn in_response_to(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("inResponseTo") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The reason or justification for the communication.
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

    /// Business identifiers assigned to this communication by the performer or other
    /// systems which remain constant as the resource is updated and propagates from
    /// server to server.
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
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

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
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

    /// A channel that was used for this communication (e.g. email, fax).
    pub fn medium(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("medium") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Description of the purpose/content, similar to a subject line in an email.
    pub fn topic(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("topic") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Other resources that pertain to this communication and to which this
    /// communication should be associated.
    pub fn about(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("about") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The patient or group that was the focus of this communication.
    pub fn subject(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subject") {
            return Some(Reference { value: val });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self._instantiates_uri() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.encounter() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self._sent() {
            _val.validate();
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.recipient() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.category() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.instantiates_canonical() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.priority() {}
        if let Some(_val) = self.note() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.payload() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.sent() {}
        if let Some(_val) = self.sender() {
            _val.validate();
        }
        if let Some(_val) = self._received() {
            _val.validate();
        }
        if let Some(_val) = self.reason_reference() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.part_of() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.based_on() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.status_reason() {
            _val.validate();
        }
        if let Some(_val) = self.received() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.instantiates_uri() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.in_response_to() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.reason_code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self._priority() {
            _val.validate();
        }
        if let Some(_val) = self.medium() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.topic() {
            _val.validate();
        }
        if let Some(_val) = self.about() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.subject() {
            _val.validate();
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
