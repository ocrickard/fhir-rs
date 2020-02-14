#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Provenance_Agent::Provenance_Agent;
use crate::model::Reference::Reference;
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
pub struct Provenance_Entity<'a> {
    pub value: &'a Value,
}

impl Provenance_Entity<'_> {
    /// The entity is attributed to an agent to express the agent's responsibility for
    /// that entity, possibly along with other agents. This description can be
    /// understood as shorthand for saying that the agent was responsible for the
    /// activity which generated the entity.
    pub fn agent(&self) -> Option<Vec<Provenance_Agent>> {
        if let Some(Value::Array(val)) = self.value.get("agent") {
            return Some(
                val.into_iter()
                    .map(|e| Provenance_Agent { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element and that modifies the understanding of the element in
    /// which it is contained and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer can define an extension, there is a set of requirements that SHALL
    /// be met as part of the definition of the extension. Applications processing a
    /// resource are required to check for modifier extensions.    Modifier extensions
    /// SHALL NOT change the meaning of any elements on Resource or DomainResource
    /// (including cannot change the meaning of modifierExtension itself).
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

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element. To make the use of extensions safe and manageable,
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

    /// How the entity was used during the activity.
    pub fn role(&self) -> Option<Provenance_EntityRole> {
        if let Some(Value::String(val)) = self.value.get("role") {
            return Some(Provenance_EntityRole::from_string(&val).unwrap());
        }
        return None;
    }

    /// Extensions for role
    pub fn _role(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_role") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Identity of the  Entity used. May be a logical or physical uri and maybe
    /// absolute or relative.
    pub fn what(&self) -> Reference {
        Reference {
            value: &self.value["what"],
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.agent() {
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
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.role() {}
        if let Some(_val) = self._role() {
            _val.validate();
        }
        let _ = self.what().validate();
        return true;
    }
}

#[derive(Debug)]
pub enum Provenance_EntityRole {
    Derivation,
    Revision,
    Quotation,
    Source,
    Removal,
}

impl Provenance_EntityRole {
    pub fn from_string(string: &str) -> Option<Provenance_EntityRole> {
        match string {
            "derivation" => Some(Provenance_EntityRole::Derivation),
            "revision" => Some(Provenance_EntityRole::Revision),
            "quotation" => Some(Provenance_EntityRole::Quotation),
            "source" => Some(Provenance_EntityRole::Source),
            "removal" => Some(Provenance_EntityRole::Removal),
            _ => None,
        }
    }
}
