#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::QuestionnaireResponse_Item::QuestionnaireResponse_Item;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// A structured set of questions and their answers. The questions are ordered and
/// grouped into coherent subsets, corresponding to the structure of the grouping of
/// the questionnaire being responded to.

#[derive(Debug)]
pub struct QuestionnaireResponse<'a> {
    pub value: &'a Value,
}

impl QuestionnaireResponse<'_> {
    /// The position of the questionnaire response within its overall lifecycle.
    pub fn status(&self) -> Option<QuestionnaireResponseStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(QuestionnaireResponseStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// Extensions for authored
    pub fn _authored(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_authored") {
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

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The subject of the questionnaire response.  This could be a patient,
    /// organization, practitioner, device, etc.  This is who/what the answers apply to,
    /// but is not necessarily the source of information.
    pub fn subject(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subject") {
            return Some(Reference { value: val });
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

    /// The Encounter during which this questionnaire response was created or to which
    /// the creation of this record is tightly associated.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The order, proposal or plan that is fulfilled in whole or in part by this
    /// QuestionnaireResponse.  For example, a ServiceRequest seeking an intake
    /// assessment or a decision support recommendation to assess for post-partum
    /// depression.
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

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
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

    /// Person who received the answers to the questions in the QuestionnaireResponse
    /// and recorded them in the system.
    pub fn author(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("author") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// A business identifier assigned to a particular completed (or partially
    /// completed) questionnaire.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier { value: val });
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

    /// The date and/or time that this set of answers were last changed.
    pub fn authored(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("authored") {
            return Some(string);
        }
        return None;
    }

    /// The person who answered the questions about the subject.
    pub fn source(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("source") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// A procedure or observation that this questionnaire was performed as part of the
    /// execution of.  For example, the surgery a checklist was executed as part of.
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

    /// A group or question item from the original questionnaire for which answers are
    /// provided.
    pub fn item(&self) -> Option<Vec<QuestionnaireResponse_Item>> {
        if let Some(Value::Array(val)) = self.value.get("item") {
            return Some(
                val.into_iter()
                    .map(|e| QuestionnaireResponse_Item { value: e })
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

    /// The Questionnaire that defines and organizes the questions for which answers are
    /// being provided.
    pub fn questionnaire(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("questionnaire") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.status() {}
        if let Some(_val) = self._authored() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.subject() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.encounter() {
            _val.validate();
        }
        if let Some(_val) = self.based_on() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.author() {
            _val.validate();
        }
        if let Some(_val) = self.identifier() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.authored() {}
        if let Some(_val) = self.source() {
            _val.validate();
        }
        if let Some(_val) = self.part_of() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.item() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.questionnaire() {}
        return true;
    }
}

#[derive(Debug)]
pub enum QuestionnaireResponseStatus {
    InProgress,
    Completed,
    Amended,
    EnteredInError,
    Stopped,
}

impl QuestionnaireResponseStatus {
    pub fn from_string(string: &str) -> Option<QuestionnaireResponseStatus> {
        match string {
            "in-progress" => Some(QuestionnaireResponseStatus::InProgress),
            "completed" => Some(QuestionnaireResponseStatus::Completed),
            "amended" => Some(QuestionnaireResponseStatus::Amended),
            "entered-in-error" => Some(QuestionnaireResponseStatus::EnteredInError),
            "stopped" => Some(QuestionnaireResponseStatus::Stopped),
            _ => None,
        }
    }
}
