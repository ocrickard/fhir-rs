#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::RequestGroup_Action::RequestGroup_Action;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// A group of related requests that can be used to capture intended activities that
/// have inter-dependencies such as "give this medication after that one".

#[derive(Debug)]
pub struct RequestGroup<'a> {
    pub value: &'a Value,
}

impl RequestGroup<'_> {
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

    /// The actions, if any, produced by the evaluation of the artifact.
    pub fn action(&self) -> Option<Vec<RequestGroup_Action>> {
        if let Some(Value::Array(val)) = self.value.get("action") {
            return Some(
                val.into_iter()
                    .map(|e| RequestGroup_Action { value: e })
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

    /// Allows a service to provide a unique, business identifier for the request.
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

    /// Describes the reason for the request group in coded or textual form.
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

    /// The current state of the request. For request groups, the status reflects the
    /// status of all the requests in the group.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// Indicates the level of authority/intentionality associated with the request and
    /// where the request fits into the workflow chain.
    pub fn intent(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("intent") {
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

    /// Indicates when the request group was created.
    pub fn authored_on(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("authoredOn") {
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

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A code that identifies what the overall request group is.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The subject for which the request group was created.
    pub fn subject(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subject") {
            return Some(Reference { value: val });
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

    /// A shared identifier common to all requests that were authorized more or less
    /// simultaneously by a single author, representing the identifier of the
    /// requisition, prescription or similar form.
    pub fn group_identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("groupIdentifier") {
            return Some(Identifier { value: val });
        }
        return None;
    }

    /// Indicates how quickly the request should be addressed with respect to other
    /// requests.
    pub fn priority(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("priority") {
            return Some(string);
        }
        return None;
    }

    /// Provides a mechanism to communicate additional information about the response.
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

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element { value: val });
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

    /// Indicates another resource whose existence justifies this request group.
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

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
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

    /// Completed or terminated request(s) whose function is taken by this new request.
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Describes the context of the request group, if any.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// A plan, proposal or order that is fulfilled in whole or in part by this request.
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

    /// Provides a reference to the author of the request group.
    pub fn author(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("author") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// A URL referencing an externally defined protocol, guideline, orderset or other
    /// definition that is adhered to in whole or in part by this request.
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

    /// A canonical URL referencing a FHIR-defined protocol, guideline, orderset or
    /// other definition that is adhered to in whole or in part by this request.
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._instantiates_canonical() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.action() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.reason_code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.intent() {}
        if let Some(_val) = self._intent() {
            _val.validate();
        }
        if let Some(_val) = self.authored_on() {}
        if let Some(_val) = self._instantiates_uri() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.code() {
            _val.validate();
        }
        if let Some(_val) = self.subject() {
            _val.validate();
        }
        if let Some(_val) = self._authored_on() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.group_identifier() {
            _val.validate();
        }
        if let Some(_val) = self.priority() {}
        if let Some(_val) = self.note() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self._priority() {
            _val.validate();
        }
        if let Some(_val) = self.reason_reference() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.replaces() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.encounter() {
            _val.validate();
        }
        if let Some(_val) = self.based_on() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.author() {
            _val.validate();
        }
        if let Some(_val) = self.instantiates_uri() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.instantiates_canonical() {
            _val.into_iter().for_each(|_e| {});
        }
        return true;
    }
}
