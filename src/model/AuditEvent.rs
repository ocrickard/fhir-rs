#![allow(unused_imports, non_camel_case_types)]

use crate::model::AuditEvent_Agent::AuditEvent_Agent;
use crate::model::AuditEvent_Entity::AuditEvent_Entity;
use crate::model::AuditEvent_Source::AuditEvent_Source;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A record of an event made for purposes of maintaining a security log. Typical
/// uses include detection of intrusion attempts and monitoring for inappropriate
/// usage.

#[derive(Debug)]
pub struct AuditEvent<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl AuditEvent<'_> {
    pub fn new(value: &Value) -> AuditEvent {
        AuditEvent {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for action
    pub fn _action(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_action") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for outcome
    pub fn _outcome(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_outcome") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for outcomeDesc
    pub fn _outcome_desc(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_outcomeDesc") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for recorded
    pub fn _recorded(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_recorded") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicator for type of action performed during the event that generated the
    /// audit.
    pub fn action(&self) -> Option<AuditEventAction> {
        if let Some(Value::String(val)) = self.value.get("action") {
            return Some(AuditEventAction::from_string(&val).unwrap());
        }
        return None;
    }

    /// An actor taking an active role in the event or activity that is logged.
    pub fn agent(&self) -> Vec<AuditEvent_Agent> {
        self.value
            .get("agent")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| AuditEvent_Agent {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
    }

    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, and nor can they
    /// have their own independent transaction scope.
    pub fn contained(&self) -> Option<Vec<ResourceList>> {
        if let Some(Value::Array(val)) = self.value.get("contained") {
            return Some(
                val.into_iter()
                    .map(|e| ResourceList {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Specific instances of data or objects that have been accessed.
    pub fn entity(&self) -> Option<Vec<AuditEvent_Entity>> {
        if let Some(Value::Array(val)) = self.value.get("entity") {
            return Some(
                val.into_iter()
                    .map(|e| AuditEvent_Entity {
                        value: Cow::Borrowed(e),
                    })
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
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

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta {
                value: Cow::Borrowed(val),
            });
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates whether the event succeeded or failed.
    pub fn outcome(&self) -> Option<AuditEventOutcome> {
        if let Some(Value::String(val)) = self.value.get("outcome") {
            return Some(AuditEventOutcome::from_string(&val).unwrap());
        }
        return None;
    }

    /// A free text description of the outcome of the event.
    pub fn outcome_desc(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("outcomeDesc") {
            return Some(string);
        }
        return None;
    }

    /// The period during which the activity occurred.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The purposeOfUse (reason) that was used during the event being recorded.
    pub fn purpose_of_event(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("purposeOfEvent") {
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

    /// The time when the event was recorded.
    pub fn recorded(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("recorded") {
            return Some(string);
        }
        return None;
    }

    /// The system that is reporting the event.
    pub fn source(&self) -> AuditEvent_Source {
        AuditEvent_Source {
            value: Cow::Borrowed(&self.value["source"]),
        }
    }

    /// Identifier for the category of event.
    pub fn subtype(&self) -> Option<Vec<Coding>> {
        if let Some(Value::Array(val)) = self.value.get("subtype") {
            return Some(
                val.into_iter()
                    .map(|e| Coding {
                        value: Cow::Borrowed(e),
                    })
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
            return Some(Narrative {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifier for a family of the event.  For example, a menu item, program, rule,
    /// policy, function code, application name or URL. It identifies the performed
    /// function.
    pub fn fhir_type(&self) -> Coding {
        Coding {
            value: Cow::Borrowed(&self.value["type"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._action() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._outcome() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._outcome_desc() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._recorded() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.action() {}
        if !self
            .agent()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.entity() {
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
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.meta() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.outcome() {}
        if let Some(_val) = self.outcome_desc() {}
        if let Some(_val) = self.period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.purpose_of_event() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.recorded() {}
        if !self.source().validate() {
            return false;
        }
        if let Some(_val) = self.subtype() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.fhir_type().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct AuditEventBuilder {
    pub(crate) value: Value,
}

impl AuditEventBuilder {
    pub fn build(&self) -> AuditEvent {
        AuditEvent {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: AuditEvent) -> AuditEventBuilder {
        AuditEventBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        agent: Vec<AuditEvent_Agent>,
        source: AuditEvent_Source,
        fhir_type: Coding,
    ) -> AuditEventBuilder {
        let mut __value: Value = json!({});
        __value["agent"] = json!(agent.into_iter().map(|e| e.value).collect::<Vec<_>>());
        __value["source"] = json!(source.value);
        __value["type"] = json!(fhir_type.value);
        return AuditEventBuilder { value: __value };
    }

    pub fn _action<'a>(&'a mut self, val: Element) -> &'a mut AuditEventBuilder {
        self.value["_action"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut AuditEventBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut AuditEventBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _outcome<'a>(&'a mut self, val: Element) -> &'a mut AuditEventBuilder {
        self.value["_outcome"] = json!(val.value);
        return self;
    }

    pub fn _outcome_desc<'a>(&'a mut self, val: Element) -> &'a mut AuditEventBuilder {
        self.value["_outcomeDesc"] = json!(val.value);
        return self;
    }

    pub fn _recorded<'a>(&'a mut self, val: Element) -> &'a mut AuditEventBuilder {
        self.value["_recorded"] = json!(val.value);
        return self;
    }

    pub fn action<'a>(&'a mut self, val: AuditEventAction) -> &'a mut AuditEventBuilder {
        self.value["action"] = json!(val.to_string());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut AuditEventBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn entity<'a>(&'a mut self, val: Vec<AuditEvent_Entity>) -> &'a mut AuditEventBuilder {
        self.value["entity"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut AuditEventBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut AuditEventBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut AuditEventBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut AuditEventBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut AuditEventBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut AuditEventBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn outcome<'a>(&'a mut self, val: AuditEventOutcome) -> &'a mut AuditEventBuilder {
        self.value["outcome"] = json!(val.to_string());
        return self;
    }

    pub fn outcome_desc<'a>(&'a mut self, val: &str) -> &'a mut AuditEventBuilder {
        self.value["outcomeDesc"] = json!(val);
        return self;
    }

    pub fn period<'a>(&'a mut self, val: Period) -> &'a mut AuditEventBuilder {
        self.value["period"] = json!(val.value);
        return self;
    }

    pub fn purpose_of_event<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut AuditEventBuilder {
        self.value["purposeOfEvent"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn recorded<'a>(&'a mut self, val: &str) -> &'a mut AuditEventBuilder {
        self.value["recorded"] = json!(val);
        return self;
    }

    pub fn subtype<'a>(&'a mut self, val: Vec<Coding>) -> &'a mut AuditEventBuilder {
        self.value["subtype"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut AuditEventBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }
}

#[derive(Debug)]
pub enum AuditEventAction {
    C,
    R,
    U,
    D,
    E,
}

impl AuditEventAction {
    pub fn from_string(string: &str) -> Option<AuditEventAction> {
        match string {
            "C" => Some(AuditEventAction::C),
            "R" => Some(AuditEventAction::R),
            "U" => Some(AuditEventAction::U),
            "D" => Some(AuditEventAction::D),
            "E" => Some(AuditEventAction::E),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            AuditEventAction::C => "C".to_string(),
            AuditEventAction::R => "R".to_string(),
            AuditEventAction::U => "U".to_string(),
            AuditEventAction::D => "D".to_string(),
            AuditEventAction::E => "E".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum AuditEventOutcome {
    Zero,
    Four,
    Eight,
    Twelve,
}

impl AuditEventOutcome {
    pub fn from_string(string: &str) -> Option<AuditEventOutcome> {
        match string {
            "0" => Some(AuditEventOutcome::Zero),
            "4" => Some(AuditEventOutcome::Four),
            "8" => Some(AuditEventOutcome::Eight),
            "12" => Some(AuditEventOutcome::Twelve),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            AuditEventOutcome::Zero => "0".to_string(),
            AuditEventOutcome::Four => "4".to_string(),
            AuditEventOutcome::Eight => "8".to_string(),
            AuditEventOutcome::Twelve => "12".to_string(),
        }
    }
}
