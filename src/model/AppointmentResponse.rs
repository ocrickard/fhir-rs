#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A reply to an appointment request for a patient and/or practitioner(s), such as
/// a confirmation or rejection.

#[derive(Debug)]
pub struct AppointmentResponse<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl AppointmentResponse<'_> {
    pub fn new(value: &Value) -> AppointmentResponse {
        AppointmentResponse {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for comment
    pub fn _comment(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_comment") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for end
    pub fn _end(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_end") {
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

    /// Extensions for participantStatus
    pub fn _participant_status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_participantStatus") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for start
    pub fn _start(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_start") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A Person, Location, HealthcareService, or Device that is participating in the
    /// appointment.
    pub fn actor(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("actor") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Appointment that this response is replying to.
    pub fn appointment(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["appointment"]),
        }
    }

    /// Additional comments about the appointment.
    pub fn comment(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("comment") {
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
                    .map(|e| ResourceList {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// This may be either the same as the appointment request to confirm the details of
    /// the appointment, or alternately a new time to request a re-negotiation of the
    /// end time.
    pub fn end(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("end") {
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

    /// This records identifiers associated with this appointment response concern that
    /// are defined by business processes and/ or used to refer to it when a direct URL
    /// reference to the resource itself is not appropriate.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier {
                        value: Cow::Borrowed(e),
                    })
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

    /// Participation status of the participant. When the status is declined or
    /// tentative if the start/end times are different to the appointment, then these
    /// times should be interpreted as a requested time change. When the status is
    /// accepted, the times can either be the time of the appointment (as a confirmation
    /// of the time) or can be empty.
    pub fn participant_status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("participantStatus") {
            return Some(string);
        }
        return None;
    }

    /// Role of participant in the appointment.
    pub fn participant_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("participantType") {
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

    /// Date/Time that the appointment is to take place, or requested new start time.
    pub fn start(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("start") {
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
            return Some(Narrative {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._comment() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._end() {
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
        if let Some(_val) = self._participant_status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._start() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.actor() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.appointment().validate() {
            return false;
        }
        if let Some(_val) = self.comment() {}
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.end() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
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
        if let Some(_val) = self.participant_status() {}
        if let Some(_val) = self.participant_type() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.start() {}
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct AppointmentResponseBuilder {
    pub(crate) value: Value,
}

impl AppointmentResponseBuilder {
    pub fn build(&self) -> AppointmentResponse {
        AppointmentResponse {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: AppointmentResponse) -> AppointmentResponseBuilder {
        AppointmentResponseBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(appointment: Reference) -> AppointmentResponseBuilder {
        let mut __value: Value = json!({});
        __value["appointment"] = json!(appointment.value);
        return AppointmentResponseBuilder { value: __value };
    }

    pub fn _comment<'a>(&'a mut self, val: Element) -> &'a mut AppointmentResponseBuilder {
        self.value["_comment"] = json!(val.value);
        return self;
    }

    pub fn _end<'a>(&'a mut self, val: Element) -> &'a mut AppointmentResponseBuilder {
        self.value["_end"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut AppointmentResponseBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut AppointmentResponseBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _participant_status<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut AppointmentResponseBuilder {
        self.value["_participantStatus"] = json!(val.value);
        return self;
    }

    pub fn _start<'a>(&'a mut self, val: Element) -> &'a mut AppointmentResponseBuilder {
        self.value["_start"] = json!(val.value);
        return self;
    }

    pub fn actor<'a>(&'a mut self, val: Reference) -> &'a mut AppointmentResponseBuilder {
        self.value["actor"] = json!(val.value);
        return self;
    }

    pub fn comment<'a>(&'a mut self, val: &str) -> &'a mut AppointmentResponseBuilder {
        self.value["comment"] = json!(val);
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut AppointmentResponseBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn end<'a>(&'a mut self, val: &str) -> &'a mut AppointmentResponseBuilder {
        self.value["end"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut AppointmentResponseBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut AppointmentResponseBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut AppointmentResponseBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut AppointmentResponseBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut AppointmentResponseBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut AppointmentResponseBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut AppointmentResponseBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn participant_status<'a>(&'a mut self, val: &str) -> &'a mut AppointmentResponseBuilder {
        self.value["participantStatus"] = json!(val);
        return self;
    }

    pub fn participant_type<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut AppointmentResponseBuilder {
        self.value["participantType"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn start<'a>(&'a mut self, val: &str) -> &'a mut AppointmentResponseBuilder {
        self.value["start"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut AppointmentResponseBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }
}
