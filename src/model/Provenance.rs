#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Provenance_Agent::Provenance_Agent;
use crate::model::Provenance_Entity::Provenance_Entity;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::Signature::Signature;
use serde_json::value::Value;

/// Provenance of a resource is a record that describes entities and processes
/// involved in producing and delivering or otherwise influencing that resource.
/// Provenance provides a critical foundation for assessing authenticity, enabling
/// trust, and allowing reproducibility. Provenance assertions are a form of
/// contextual metadata and can themselves become important records with their own
/// provenance. Provenance statement indicates clinical significance in terms of
/// confidence in authenticity, reliability, and trustworthiness, integrity, and
/// stage in lifecycle (e.g. Document Completion - has the artifact been legally
/// authenticated), all of which may impact security, privacy, and trust policies.

#[derive(Debug)]
pub struct Provenance<'a> {
    pub value: &'a Value,
}

impl Provenance<'_> {
    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
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

    /// The Reference(s) that were generated or updated by  the activity described in
    /// this resource. A provenance can point to more than one target if multiple
    /// resources were created/updated by the same activity.
    pub fn target(&self) -> Vec<Reference> {
        self.value
            .get("target")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| Reference { value: e })
            .collect::<Vec<_>>()
    }

    /// The instant of time at which the activity was recorded.
    pub fn recorded(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("recorded") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for occurredDateTime
    pub fn _occurred_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_occurredDateTime") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The period during which the activity occurred.
    pub fn occurred_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("occurredDateTime") {
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

    /// The period during which the activity occurred.
    pub fn occurred_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("occurredPeriod") {
            return Some(Period { value: val });
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

    /// Extensions for recorded
    pub fn _recorded(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_recorded") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Policy or plan the activity was defined by. Typically, a single activity may
    /// have multiple applicable policy documents, such as patient consent, guarantor
    /// funding, etc.
    pub fn policy(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("policy") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for policy
    pub fn _policy(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_policy") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Where the activity occurred, if relevant.
    pub fn location(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("location") {
            return Some(Reference { value: val });
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

    /// The reason that the activity was taking place.
    pub fn reason(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("reason") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An activity is something that occurs over a period of time and acts upon or with
    /// entities; it may include consuming, processing, transforming, modifying,
    /// relocating, using, or generating entities.
    pub fn activity(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("activity") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// An actor taking a role in an activity  for which it can be assigned some degree
    /// of responsibility for the activity taking place.
    pub fn agent(&self) -> Vec<Provenance_Agent> {
        self.value
            .get("agent")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| Provenance_Agent { value: e })
            .collect::<Vec<_>>()
    }

    /// An entity used in this activity.
    pub fn entity(&self) -> Option<Vec<Provenance_Entity>> {
        if let Some(Value::Array(val)) = self.value.get("entity") {
            return Some(
                val.into_iter()
                    .map(|e| Provenance_Entity { value: e })
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

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A digital signature on the target Reference(s). The signer should match a
    /// Provenance.agent. The purpose of the signature is indicated.
    pub fn signature(&self) -> Option<Vec<Signature>> {
        if let Some(Value::Array(val)) = self.value.get("signature") {
            return Some(
                val.into_iter()
                    .map(|e| Signature { value: e })
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        let _ = self.target().into_iter().for_each(|e| {
            e.validate();
        });
        if let Some(_val) = self.recorded() {}
        if let Some(_val) = self._occurred_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.occurred_date_time() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.occurred_period() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self._recorded() {
            _val.validate();
        }
        if let Some(_val) = self.policy() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self._policy() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.location() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.reason() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.activity() {
            _val.validate();
        }
        let _ = self.agent().into_iter().for_each(|e| {
            e.validate();
        });
        if let Some(_val) = self.entity() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
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
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.signature() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
