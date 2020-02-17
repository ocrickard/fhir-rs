#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Provenance_Agent::Provenance_Agent;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

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
    pub(crate) value: Cow<'a, Value>,
}

impl Provenance_Entity<'_> {
    pub fn new(value: &Value) -> Provenance_Entity {
        Provenance_Entity {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for role
    pub fn _role(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_role") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The entity is attributed to an agent to express the agent's responsibility for
    /// that entity, possibly along with other agents. This description can be
    /// understood as shorthand for saying that the agent was responsible for the
    /// activity which generated the entity.
    pub fn agent(&self) -> Option<Vec<Provenance_Agent>> {
        if let Some(Value::Array(val)) = self.value.get("agent") {
            return Some(
                val.into_iter()
                    .map(|e| Provenance_Agent {
                        value: Cow::Borrowed(e),
                    })
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
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

    /// Identity of the  Entity used. May be a logical or physical uri and maybe
    /// absolute or relative.
    pub fn what(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["what"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._role() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.agent() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.role() {}
        if !self.what().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Provenance_EntityBuilder {
    pub(crate) value: Value,
}

impl Provenance_EntityBuilder {
    pub fn build(&self) -> Provenance_Entity {
        Provenance_Entity {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Provenance_Entity) -> Provenance_EntityBuilder {
        Provenance_EntityBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(what: Reference) -> Provenance_EntityBuilder {
        let mut __value: Value = json!({});
        __value["what"] = json!(what.value);
        return Provenance_EntityBuilder { value: __value };
    }

    pub fn _role<'a>(&'a mut self, val: Element) -> &'a mut Provenance_EntityBuilder {
        self.value["_role"] = json!(val.value);
        return self;
    }

    pub fn agent<'a>(&'a mut self, val: Vec<Provenance_Agent>) -> &'a mut Provenance_EntityBuilder {
        self.value["agent"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Provenance_EntityBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Provenance_EntityBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Provenance_EntityBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn role<'a>(&'a mut self, val: Provenance_EntityRole) -> &'a mut Provenance_EntityBuilder {
        self.value["role"] = json!(val.to_string());
        return self;
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

    pub fn to_string(&self) -> String {
        match self {
            Provenance_EntityRole::Derivation => "derivation".to_string(),
            Provenance_EntityRole::Revision => "revision".to_string(),
            Provenance_EntityRole::Quotation => "quotation".to_string(),
            Provenance_EntityRole::Source => "source".to_string(),
            Provenance_EntityRole::Removal => "removal".to_string(),
        }
    }
}
