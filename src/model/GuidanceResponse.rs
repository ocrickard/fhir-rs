#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::DataRequirement::DataRequirement;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// A guidance response is the formal response to a guidance request, including any
/// output parameters returned by the evaluation, as well as the description of any
/// proposed actions to be taken.

#[derive(Debug)]
pub struct GuidanceResponse<'a> {
    pub value: &'a Value,
}

impl GuidanceResponse<'_> {
    /// Extensions for moduleCanonical
    pub fn _module_canonical(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_moduleCanonical") {
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

    /// Allows a service to provide  unique, business identifiers for the response.
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

    /// An identifier, CodeableConcept or canonical reference to the guidance that was
    /// requested.
    pub fn module_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("moduleCodeableConcept") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// If the evaluation could not be completed due to lack of information, or
    /// additional information would potentially result in a more accurate response,
    /// this element will a description of the data required in order to proceed with
    /// the evaluation. A subsequent request to the service should include this data.
    pub fn data_requirement(&self) -> Option<Vec<DataRequirement>> {
        if let Some(Value::Array(val)) = self.value.get("dataRequirement") {
            return Some(
                val.into_iter()
                    .map(|e| DataRequirement { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An identifier, CodeableConcept or canonical reference to the guidance that was
    /// requested.
    pub fn module_uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("moduleUri") {
            return Some(string);
        }
        return None;
    }

    /// The actions, if any, produced by the evaluation of the artifact.
    pub fn result(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("result") {
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

    /// An identifier, CodeableConcept or canonical reference to the guidance that was
    /// requested.
    pub fn module_canonical(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("moduleCanonical") {
            return Some(string);
        }
        return None;
    }

    /// Indicates the reason the request was initiated. This is typically provided as a
    /// parameter to the evaluation and echoed by the service, although for some use
    /// cases, such as subscription- or event-based scenarios, it may provide an
    /// indication of the cause for the response.
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

    /// The patient for which the request was processed.
    pub fn subject(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subject") {
            return Some(Reference { value: val });
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

    /// Indicates when the guidance response was processed.
    pub fn occurrence_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("occurrenceDateTime") {
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

    /// Describes the reason for the guidance response in coded or textual form.
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

    /// Messages resulting from the evaluation of the artifact or artifacts. As part of
    /// evaluating the request, the engine may produce informational or warning
    /// messages. These messages will be provided by this element.
    pub fn evaluation_message(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("evaluationMessage") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The encounter during which this response was created or to which the creation of
    /// this record is tightly associated.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference { value: val });
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

    /// Provides a reference to the device that performed the guidance.
    pub fn performer(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("performer") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Extensions for moduleUri
    pub fn _module_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_moduleUri") {
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

    /// The status of the response. If the evaluation is completed successfully, the
    /// status will indicate success. However, in order to complete the evaluation, the
    /// engine may require more information. In this case, the status will be data-
    /// required, and the response will contain a description of the additional
    /// required information. If the evaluation completed successfully, but the engine
    /// determines that a potentially more accurate response could be provided if more
    /// data was available, the status will be data-requested, and the response will
    /// contain a description of the additional requested information.
    pub fn status(&self) -> Option<GuidanceResponseStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(GuidanceResponseStatus::from_string(&val).unwrap());
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

    /// The output parameters of the evaluation, if any. Many modules will result in the
    /// return of specific resources such as procedure or communication requests that
    /// are returned as part of the operation result. However, modules may define
    /// specific outputs that would be returned as the result of the evaluation, and
    /// these would be returned in this element.
    pub fn output_parameters(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("outputParameters") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The identifier of the request associated with this response. If an identifier
    /// was given as part of the request, it will be reproduced here to enable the
    /// requester to more easily identify the response in a multi-request scenario.
    pub fn request_identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("requestIdentifier") {
            return Some(Identifier { value: val });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._module_canonical() {
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
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.module_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self.data_requirement() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.module_uri() {}
        if let Some(_val) = self.result() {
            _val.validate();
        }
        if let Some(_val) = self._occurrence_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.module_canonical() {}
        if let Some(_val) = self.reason_reference() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.subject() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.occurrence_date_time() {}
        if let Some(_val) = self.contained() {
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
        if let Some(_val) = self.evaluation_message() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.encounter() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.performer() {
            _val.validate();
        }
        if let Some(_val) = self._module_uri() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.output_parameters() {
            _val.validate();
        }
        if let Some(_val) = self.request_identifier() {
            _val.validate();
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum GuidanceResponseStatus {
    Success,
    DataRequested,
    DataRequired,
    InProgress,
    Failure,
    EnteredInError,
}

impl GuidanceResponseStatus {
    pub fn from_string(string: &str) -> Option<GuidanceResponseStatus> {
        match string {
            "success" => Some(GuidanceResponseStatus::Success),
            "data-requested" => Some(GuidanceResponseStatus::DataRequested),
            "data-required" => Some(GuidanceResponseStatus::DataRequired),
            "in-progress" => Some(GuidanceResponseStatus::InProgress),
            "failure" => Some(GuidanceResponseStatus::Failure),
            "entered-in-error" => Some(GuidanceResponseStatus::EnteredInError),
            _ => None,
        }
    }
}
