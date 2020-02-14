#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::MessageHeader_Destination::MessageHeader_Destination;
use crate::model::MessageHeader_Response::MessageHeader_Response;
use crate::model::MessageHeader_Source::MessageHeader_Source;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// The header for a message exchange that is either requesting or responding to an
/// action.  The reference(s) that are the subject of the action as well as other
/// information related to the action are typically transmitted in a bundle in which
/// the MessageHeader resource instance is the first resource in the bundle.

#[derive(Debug)]
pub struct MessageHeader<'a> {
    pub value: &'a Value,
}

impl MessageHeader<'_> {
    /// Information about the message that this message is a response to.  Only present
    /// if this message is a response.
    pub fn response(&self) -> Option<MessageHeader_Response> {
        if let Some(val) = self.value.get("response") {
            return Some(MessageHeader_Response { value: val });
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

    /// The logical author of the message - the person or device that decided the
    /// described event should happen. When there is more than one candidate, pick the
    /// most proximal to the MessageHeader. Can provide other authors in extensions.
    pub fn author(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("author") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The person or device that performed the data entry leading to this message. When
    /// there is more than one candidate, pick the most proximal to the message. Can
    /// provide other enterers in extensions.
    pub fn enterer(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("enterer") {
            return Some(Reference { value: val });
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

    /// Extensions for eventUri
    pub fn _event_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_eventUri") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The destination application which the message is intended for.
    pub fn destination(&self) -> Option<Vec<MessageHeader_Destination>> {
        if let Some(Value::Array(val)) = self.value.get("destination") {
            return Some(
                val.into_iter()
                    .map(|e| MessageHeader_Destination { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Code that identifies the event this message represents and connects it with its
    /// definition. Events defined as part of the FHIR specification have the system
    /// value "http://terminology.hl7.org/CodeSystem/message-events".  Alternatively uri
    /// to the EventDefinition.
    pub fn event_coding(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("eventCoding") {
            return Some(Coding { value: val });
        }
        return None;
    }

    /// The source application from which this message originated.
    pub fn source(&self) -> MessageHeader_Source {
        MessageHeader_Source {
            value: &self.value["source"],
        }
    }

    /// Permanent link to the MessageDefinition for this message.
    pub fn definition(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("definition") {
            return Some(string);
        }
        return None;
    }

    /// The person or organization that accepts overall responsibility for the contents
    /// of the message. The implication is that the message event happened under the
    /// policies of the responsible party.
    pub fn responsible(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("responsible") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Code that identifies the event this message represents and connects it with its
    /// definition. Events defined as part of the FHIR specification have the system
    /// value "http://terminology.hl7.org/CodeSystem/message-events".  Alternatively uri
    /// to the EventDefinition.
    pub fn event_uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("eventUri") {
            return Some(string);
        }
        return None;
    }

    /// Identifies the sending system to allow the use of a trust relationship.
    pub fn sender(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("sender") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Coded indication of the cause for the event - indicates  a reason for the
    /// occurrence of the event that is a focus of this message.
    pub fn reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("reason") {
            return Some(CodeableConcept { value: val });
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

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
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

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The actual data of the message - a reference to the root/focus class of the
    /// event.
    pub fn focus(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("focus") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.response() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.author() {
            _val.validate();
        }
        if let Some(_val) = self.enterer() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._event_uri() {
            _val.validate();
        }
        if let Some(_val) = self.destination() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.event_coding() {
            _val.validate();
        }
        let _ = self.source().validate();
        if let Some(_val) = self.definition() {}
        if let Some(_val) = self.responsible() {
            _val.validate();
        }
        if let Some(_val) = self.event_uri() {}
        if let Some(_val) = self.sender() {
            _val.validate();
        }
        if let Some(_val) = self.reason() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.focus() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
