#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
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
pub struct Provenance_Agent<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Provenance_Agent<'_> {
    pub fn new(value: &Value) -> Provenance_Agent {
        Provenance_Agent {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// The individual, device, or organization for whom the change was made.
    pub fn on_behalf_of(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("onBehalfOf") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The function of the agent with respect to the activity. The security role
    /// enabling the agent with respect to the activity.
    pub fn role(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("role") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The participation the agent had with respect to the activity.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The individual, device or organization that participated in the event.
    pub fn who(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["who"]),
        }
    }

    pub fn validate(&self) -> bool {
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
        if let Some(_val) = self.on_behalf_of() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.role() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.who().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Provenance_AgentBuilder {
    pub(crate) value: Value,
}

impl Provenance_AgentBuilder {
    pub fn build(&self) -> Provenance_Agent {
        Provenance_Agent {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Provenance_Agent) -> Provenance_AgentBuilder {
        Provenance_AgentBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(who: Reference) -> Provenance_AgentBuilder {
        let mut __value: Value = json!({});
        __value["who"] = json!(who.value);
        return Provenance_AgentBuilder { value: __value };
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Provenance_AgentBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Provenance_AgentBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Provenance_AgentBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn on_behalf_of<'a>(&'a mut self, val: Reference) -> &'a mut Provenance_AgentBuilder {
        self.value["onBehalfOf"] = json!(val.value);
        return self;
    }

    pub fn role<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut Provenance_AgentBuilder {
        self.value["role"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Provenance_AgentBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
