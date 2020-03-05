#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::Task_Input::Task_Input;
use crate::model::Task_Output::Task_Output;
use crate::model::Task_Restriction::Task_Restriction;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A task to be performed.

#[derive(Debug)]
pub struct Task<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Task<'_> {
    pub fn new(value: &Value) -> Task {
        Task {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for authoredOn
    pub fn _authored_on(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_authoredOn") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
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

    /// Extensions for instantiatesUri
    pub fn _instantiates_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_instantiatesUri") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for intent
    pub fn _intent(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_intent") {
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

    /// Extensions for lastModified
    pub fn _last_modified(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lastModified") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for priority
    pub fn _priority(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_priority") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The date and time this task was created.
    pub fn authored_on(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("authoredOn") {
            return Some(string);
        }
        return None;
    }

    /// BasedOn refers to a higher-level authorization that triggered the creation of
    /// the task.  It references a "request" resource such as a ServiceRequest,
    /// MedicationRequest, ServiceRequest, CarePlan, etc. which is distinct from the
    /// "request" resource the task is seeking to fulfill.  This latter resource is
    /// referenced by FocusOn.  For example, based on a ServiceRequest (= BasedOn), a
    /// task is created to fulfill a procedureRequest ( = FocusOn ) to collect a
    /// specimen from a patient.
    pub fn based_on(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("basedOn") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Contains business-specific nuances of the business state.
    pub fn business_status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("businessStatus") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A name or code (or both) briefly describing what the task involves.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// A free-text description of what is to be performed.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// The healthcare event  (e.g. a patient and healthcare provider interaction)
    /// during which this task was created.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifies the time action was first taken against the task (start) and/or the
    /// time final action was taken against the task prior to marking it as completed
    /// (end).
    pub fn execution_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("executionPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
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

    /// The request being actioned or the resource being manipulated by this task.
    pub fn focus(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("focus") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The entity who benefits from the performance of the service specified in the
    /// task (e.g., the patient).
    pub fn fhir_for(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("for") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An identifier that links together multiple tasks and other requests that were
    /// created in the same context.
    pub fn group_identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("groupIdentifier") {
            return Some(Identifier {
                value: Cow::Borrowed(val),
            });
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

    /// The business identifier for this task.
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

    /// Additional information that may be needed in the execution of the task.
    pub fn input(&self) -> Option<Vec<Task_Input>> {
        if let Some(Value::Array(val)) = self.value.get("input") {
            return Some(
                val.into_iter()
                    .map(|e| Task_Input {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The URL pointing to a *FHIR*-defined protocol, guideline, orderset or other
    /// definition that is adhered to in whole or in part by this Task.
    pub fn instantiates_canonical(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("instantiatesCanonical") {
            return Some(string);
        }
        return None;
    }

    /// The URL pointing to an *externally* maintained  protocol, guideline, orderset or
    /// other definition that is adhered to in whole or in part by this Task.
    pub fn instantiates_uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("instantiatesUri") {
            return Some(string);
        }
        return None;
    }

    /// Insurance plans, coverage extensions, pre-authorizations and/or pre-
    /// determinations that may be relevant to the Task.
    pub fn insurance(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("insurance") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates the "level" of actionability associated with the Task, i.e. i+R[9]Cs
    /// this a proposed task, a planned task, an actionable task, etc.
    pub fn intent(&self) -> Option<TaskIntent> {
        if let Some(Value::String(val)) = self.value.get("intent") {
            return Some(TaskIntent::from_string(&val).unwrap());
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

    /// The date and time of last modification to this task.
    pub fn last_modified(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("lastModified") {
            return Some(string);
        }
        return None;
    }

    /// Principal physical location where the this task is performed.
    pub fn location(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("location") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// Free-text information captured about the task as it progresses.
    pub fn note(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("note") {
            return Some(
                val.into_iter()
                    .map(|e| Annotation {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Outputs produced by the Task.
    pub fn output(&self) -> Option<Vec<Task_Output>> {
        if let Some(Value::Array(val)) = self.value.get("output") {
            return Some(
                val.into_iter()
                    .map(|e| Task_Output {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Individual organization or Device currently responsible for task execution.
    pub fn owner(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("owner") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Task that this particular task is part of.
    pub fn part_of(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("partOf") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The kind of participant that should perform the task.
    pub fn performer_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("performerType") {
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

    /// Indicates how quickly the Task should be addressed with respect to other
    /// requests.
    pub fn priority(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("priority") {
            return Some(string);
        }
        return None;
    }

    /// A description or code indicating why this task needs to be performed.
    pub fn reason_code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("reasonCode") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A resource reference indicating why this task needs to be performed.
    pub fn reason_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("reasonReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Links to Provenance records for past versions of this Task that identify key
    /// state transitions or updates that are likely to be relevant to a user looking at
    /// the current version of the task.
    pub fn relevant_history(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("relevantHistory") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The creator of the task.
    pub fn requester(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("requester") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// If the Task.focus is a request resource and the task is seeking fulfillment
    /// (i.e. is asking for the request to be actioned), this element identifies any
    /// limitations on what parts of the referenced request should be actioned.
    pub fn restriction(&self) -> Option<Task_Restriction> {
        if let Some(val) = self.value.get("restriction") {
            return Some(Task_Restriction {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The current status of the task.
    pub fn status(&self) -> Option<TaskStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(TaskStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// An explanation as to why this task is held, failed, was refused, etc.
    pub fn status_reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("statusReason") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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
        if let Some(_val) = self._authored_on() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._instantiates_uri() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._intent() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._last_modified() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._priority() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.authored_on() {}
        if let Some(_val) = self.based_on() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.business_status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.encounter() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.execution_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.focus() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_for() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.group_identifier() {
            if !_val.validate() {
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
        if let Some(_val) = self.input() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.instantiates_canonical() {}
        if let Some(_val) = self.instantiates_uri() {}
        if let Some(_val) = self.insurance() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.intent() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.last_modified() {}
        if let Some(_val) = self.location() {
            if !_val.validate() {
                return false;
            }
        }
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
        if let Some(_val) = self.note() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.output() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.owner() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.part_of() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.performer_type() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.priority() {}
        if let Some(_val) = self.reason_code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.reason_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.relevant_history() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.requester() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.restriction() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.status_reason() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct TaskBuilder {
    pub(crate) value: Value,
}

impl TaskBuilder {
    pub fn build(&self) -> Task {
        Task {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Task) -> TaskBuilder {
        TaskBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> TaskBuilder {
        let mut __value: Value = json!({});
        return TaskBuilder { value: __value };
    }

    pub fn _authored_on<'a>(&'a mut self, val: Element) -> &'a mut TaskBuilder {
        self.value["_authoredOn"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut TaskBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut TaskBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _instantiates_uri<'a>(&'a mut self, val: Element) -> &'a mut TaskBuilder {
        self.value["_instantiatesUri"] = json!(val.value);
        return self;
    }

    pub fn _intent<'a>(&'a mut self, val: Element) -> &'a mut TaskBuilder {
        self.value["_intent"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut TaskBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _last_modified<'a>(&'a mut self, val: Element) -> &'a mut TaskBuilder {
        self.value["_lastModified"] = json!(val.value);
        return self;
    }

    pub fn _priority<'a>(&'a mut self, val: Element) -> &'a mut TaskBuilder {
        self.value["_priority"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut TaskBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn authored_on<'a>(&'a mut self, val: &str) -> &'a mut TaskBuilder {
        self.value["authoredOn"] = json!(val);
        return self;
    }

    pub fn based_on<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut TaskBuilder {
        self.value["basedOn"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn business_status<'a>(&'a mut self, val: CodeableConcept) -> &'a mut TaskBuilder {
        self.value["businessStatus"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(&'a mut self, val: CodeableConcept) -> &'a mut TaskBuilder {
        self.value["code"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut TaskBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut TaskBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn encounter<'a>(&'a mut self, val: Reference) -> &'a mut TaskBuilder {
        self.value["encounter"] = json!(val.value);
        return self;
    }

    pub fn execution_period<'a>(&'a mut self, val: Period) -> &'a mut TaskBuilder {
        self.value["executionPeriod"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut TaskBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn focus<'a>(&'a mut self, val: Reference) -> &'a mut TaskBuilder {
        self.value["focus"] = json!(val.value);
        return self;
    }

    pub fn fhir_for<'a>(&'a mut self, val: Reference) -> &'a mut TaskBuilder {
        self.value["for"] = json!(val.value);
        return self;
    }

    pub fn group_identifier<'a>(&'a mut self, val: Identifier) -> &'a mut TaskBuilder {
        self.value["groupIdentifier"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut TaskBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut TaskBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut TaskBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn input<'a>(&'a mut self, val: Vec<Task_Input>) -> &'a mut TaskBuilder {
        self.value["input"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn instantiates_canonical<'a>(&'a mut self, val: &str) -> &'a mut TaskBuilder {
        self.value["instantiatesCanonical"] = json!(val);
        return self;
    }

    pub fn instantiates_uri<'a>(&'a mut self, val: &str) -> &'a mut TaskBuilder {
        self.value["instantiatesUri"] = json!(val);
        return self;
    }

    pub fn insurance<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut TaskBuilder {
        self.value["insurance"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn intent<'a>(&'a mut self, val: TaskIntent) -> &'a mut TaskBuilder {
        self.value["intent"] = json!(val.to_string());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut TaskBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn last_modified<'a>(&'a mut self, val: &str) -> &'a mut TaskBuilder {
        self.value["lastModified"] = json!(val);
        return self;
    }

    pub fn location<'a>(&'a mut self, val: Reference) -> &'a mut TaskBuilder {
        self.value["location"] = json!(val.value);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut TaskBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut TaskBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut TaskBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn output<'a>(&'a mut self, val: Vec<Task_Output>) -> &'a mut TaskBuilder {
        self.value["output"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn owner<'a>(&'a mut self, val: Reference) -> &'a mut TaskBuilder {
        self.value["owner"] = json!(val.value);
        return self;
    }

    pub fn part_of<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut TaskBuilder {
        self.value["partOf"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn performer_type<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut TaskBuilder {
        self.value["performerType"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn priority<'a>(&'a mut self, val: &str) -> &'a mut TaskBuilder {
        self.value["priority"] = json!(val);
        return self;
    }

    pub fn reason_code<'a>(&'a mut self, val: CodeableConcept) -> &'a mut TaskBuilder {
        self.value["reasonCode"] = json!(val.value);
        return self;
    }

    pub fn reason_reference<'a>(&'a mut self, val: Reference) -> &'a mut TaskBuilder {
        self.value["reasonReference"] = json!(val.value);
        return self;
    }

    pub fn relevant_history<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut TaskBuilder {
        self.value["relevantHistory"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn requester<'a>(&'a mut self, val: Reference) -> &'a mut TaskBuilder {
        self.value["requester"] = json!(val.value);
        return self;
    }

    pub fn restriction<'a>(&'a mut self, val: Task_Restriction) -> &'a mut TaskBuilder {
        self.value["restriction"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: TaskStatus) -> &'a mut TaskBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn status_reason<'a>(&'a mut self, val: CodeableConcept) -> &'a mut TaskBuilder {
        self.value["statusReason"] = json!(val.value);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut TaskBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }
}

#[derive(Debug)]
pub enum TaskIntent {
    Unknown,
    Proposal,
    Plan,
    Order,
    OriginalOrder,
    ReflexOrder,
    FillerOrder,
    InstanceOrder,
    Option,
}

impl TaskIntent {
    pub fn from_string(string: &str) -> Option<TaskIntent> {
        match string {
            "unknown" => Some(TaskIntent::Unknown),
            "proposal" => Some(TaskIntent::Proposal),
            "plan" => Some(TaskIntent::Plan),
            "order" => Some(TaskIntent::Order),
            "original-order" => Some(TaskIntent::OriginalOrder),
            "reflex-order" => Some(TaskIntent::ReflexOrder),
            "filler-order" => Some(TaskIntent::FillerOrder),
            "instance-order" => Some(TaskIntent::InstanceOrder),
            "option" => Some(TaskIntent::Option),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TaskIntent::Unknown => "unknown".to_string(),
            TaskIntent::Proposal => "proposal".to_string(),
            TaskIntent::Plan => "plan".to_string(),
            TaskIntent::Order => "order".to_string(),
            TaskIntent::OriginalOrder => "original-order".to_string(),
            TaskIntent::ReflexOrder => "reflex-order".to_string(),
            TaskIntent::FillerOrder => "filler-order".to_string(),
            TaskIntent::InstanceOrder => "instance-order".to_string(),
            TaskIntent::Option => "option".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum TaskStatus {
    Draft,
    Requested,
    Received,
    Accepted,
    Rejected,
    Ready,
    Cancelled,
    InProgress,
    OnHold,
    Failed,
    Completed,
    EnteredInError,
}

impl TaskStatus {
    pub fn from_string(string: &str) -> Option<TaskStatus> {
        match string {
            "draft" => Some(TaskStatus::Draft),
            "requested" => Some(TaskStatus::Requested),
            "received" => Some(TaskStatus::Received),
            "accepted" => Some(TaskStatus::Accepted),
            "rejected" => Some(TaskStatus::Rejected),
            "ready" => Some(TaskStatus::Ready),
            "cancelled" => Some(TaskStatus::Cancelled),
            "in-progress" => Some(TaskStatus::InProgress),
            "on-hold" => Some(TaskStatus::OnHold),
            "failed" => Some(TaskStatus::Failed),
            "completed" => Some(TaskStatus::Completed),
            "entered-in-error" => Some(TaskStatus::EnteredInError),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TaskStatus::Draft => "draft".to_string(),
            TaskStatus::Requested => "requested".to_string(),
            TaskStatus::Received => "received".to_string(),
            TaskStatus::Accepted => "accepted".to_string(),
            TaskStatus::Rejected => "rejected".to_string(),
            TaskStatus::Ready => "ready".to_string(),
            TaskStatus::Cancelled => "cancelled".to_string(),
            TaskStatus::InProgress => "in-progress".to_string(),
            TaskStatus::OnHold => "on-hold".to_string(),
            TaskStatus::Failed => "failed".to_string(),
            TaskStatus::Completed => "completed".to_string(),
            TaskStatus::EnteredInError => "entered-in-error".to_string(),
        }
    }
}
