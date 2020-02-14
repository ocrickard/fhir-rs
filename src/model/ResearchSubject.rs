#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// A physical entity which is the primary unit of operational and/or administrative
/// interest in a study.

#[derive(Debug)]
pub struct ResearchSubject<'a> {
    pub value: &'a Value,
}

impl ResearchSubject<'_> {
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

    /// The name of the arm in the study the subject actually followed as part of this
    /// study.
    pub fn actual_arm(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("actualArm") {
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
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

    /// Extensions for actualArm
    pub fn _actual_arm(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_actualArm") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The record of the person or animal who is involved in the study.
    pub fn individual(&self) -> Reference {
        Reference {
            value: &self.value["individual"],
        }
    }

    /// A record of the patient's informed agreement to participate in the study.
    pub fn consent(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("consent") {
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

    /// Identifiers assigned to this research subject for a study.
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

    /// Extensions for assignedArm
    pub fn _assigned_arm(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_assignedArm") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Reference to the study the subject is participating in.
    pub fn study(&self) -> Reference {
        Reference {
            value: &self.value["study"],
        }
    }

    /// The dates the subject began and ended their participation in the study.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// The name of the arm in the study the subject is expected to follow as part of
    /// this study.
    pub fn assigned_arm(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("assignedArm") {
            return Some(string);
        }
        return None;
    }

    /// The current state of the subject.
    pub fn status(&self) -> Option<ResearchSubjectStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(ResearchSubjectStatus::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.actual_arm() {}
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._actual_arm() {
            _val.validate();
        }
        let _ = self.individual().validate();
        if let Some(_val) = self.consent() {
            _val.validate();
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._assigned_arm() {
            _val.validate();
        }
        let _ = self.study().validate();
        if let Some(_val) = self.period() {
            _val.validate();
        }
        if let Some(_val) = self.assigned_arm() {}
        if let Some(_val) = self.status() {}
        return true;
    }
}

#[derive(Debug)]
pub enum ResearchSubjectStatus {
    Candidate,
    Eligible,
    FollowUp,
    Ineligible,
    NotRegistered,
    OffStudy,
    OnStudy,
    OnStudyIntervention,
    OnStudyObservation,
    PendingOnStudy,
    PotentialCandidate,
    Screening,
    Withdrawn,
}

impl ResearchSubjectStatus {
    pub fn from_string(string: &str) -> Option<ResearchSubjectStatus> {
        match string {
            "candidate" => Some(ResearchSubjectStatus::Candidate),
            "eligible" => Some(ResearchSubjectStatus::Eligible),
            "follow-up" => Some(ResearchSubjectStatus::FollowUp),
            "ineligible" => Some(ResearchSubjectStatus::Ineligible),
            "not-registered" => Some(ResearchSubjectStatus::NotRegistered),
            "off-study" => Some(ResearchSubjectStatus::OffStudy),
            "on-study" => Some(ResearchSubjectStatus::OnStudy),
            "on-study-intervention" => Some(ResearchSubjectStatus::OnStudyIntervention),
            "on-study-observation" => Some(ResearchSubjectStatus::OnStudyObservation),
            "pending-on-study" => Some(ResearchSubjectStatus::PendingOnStudy),
            "potential-candidate" => Some(ResearchSubjectStatus::PotentialCandidate),
            "screening" => Some(ResearchSubjectStatus::Screening),
            "withdrawn" => Some(ResearchSubjectStatus::Withdrawn),
            _ => None,
        }
    }
}
