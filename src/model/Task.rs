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
use serde_json::value::Value;

/// A task to be performed.

#[derive(Debug)]
pub struct Task<'a> {
    pub value: &'a Value,
}

impl Task<'_> {
    /// Indicates the "level" of actionability associated with the Task, i.e. i+R[9]Cs
    /// this a proposed task, a planned task, an actionable task, etc.
    pub fn intent(&self) -> Option<TaskIntent> {
        if let Some(Value::String(val)) = self.value.get("intent") {
            return Some(TaskIntent::from_string(&val).unwrap());
        }
        return None;
    }

    /// Task that this particular task is part of.
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

    /// Insurance plans, coverage extensions, pre-authorizations and/or pre-
    /// determinations that may be relevant to the Task.
    pub fn insurance(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("insurance") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
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
                    .map(|e| CodeableConcept { value: e })
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
                    .map(|e| Task_Output { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// The creator of the task.
    pub fn requester(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("requester") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Principal physical location where the this task is performed.
    pub fn location(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("location") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Extensions for authoredOn
    pub fn _authored_on(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_authoredOn") {
            return Some(Element { value: val });
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The business identifier for this task.
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

    /// An explanation as to why this task is held, failed, was refused, etc.
    pub fn status_reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("statusReason") {
            return Some(CodeableConcept { value: val });
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

    /// Extensions for intent
    pub fn _intent(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_intent") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for instantiatesUri
    pub fn _instantiates_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_instantiatesUri") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A description or code indicating why this task needs to be performed.
    pub fn reason_code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("reasonCode") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// If the Task.focus is a request resource and the task is seeking fulfillment
    /// (i.e. is asking for the request to be actioned), this element identifies any
    /// limitations on what parts of the referenced request should be actioned.
    pub fn restriction(&self) -> Option<Task_Restriction> {
        if let Some(val) = self.value.get("restriction") {
            return Some(Task_Restriction { value: val });
        }
        return None;
    }

    /// Additional information that may be needed in the execution of the task.
    pub fn input(&self) -> Option<Vec<Task_Input>> {
        if let Some(Value::Array(val)) = self.value.get("input") {
            return Some(
                val.into_iter()
                    .map(|e| Task_Input { value: e })
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

    /// The date and time this task was created.
    pub fn authored_on(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("authoredOn") {
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

    /// A resource reference indicating why this task needs to be performed.
    pub fn reason_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("reasonReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Free-text information captured about the task as it progresses.
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

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element { value: val });
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

    /// A name or code (or both) briefly describing what the task involves.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept { value: val });
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

    /// Contains business-specific nuances of the business state.
    pub fn business_status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("businessStatus") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The healthcare event  (e.g. a patient and healthcare provider interaction)
    /// during which this task was created.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference { value: val });
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

    /// The entity who benefits from the performance of the service specified in the
    /// task (e.g., the patient).
    pub fn fhir_for(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("for") {
            return Some(Reference { value: val });
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
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for priority
    pub fn _priority(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_priority") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Individual organization or Device currently responsible for task execution.
    pub fn owner(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("owner") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Extensions for lastModified
    pub fn _last_modified(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lastModified") {
            return Some(Element { value: val });
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

    /// The URL pointing to a *FHIR*-defined protocol, guideline, orderset or other
    /// definition that is adhered to in whole or in part by this Task.
    pub fn instantiates_canonical(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("instantiatesCanonical") {
            return Some(string);
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

    /// An identifier that links together multiple tasks and other requests that were
    /// created in the same context.
    pub fn group_identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("groupIdentifier") {
            return Some(Identifier { value: val });
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

    /// The request being actioned or the resource being manipulated by this task.
    pub fn focus(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("focus") {
            return Some(Reference { value: val });
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

    /// The current status of the task.
    pub fn status(&self) -> Option<TaskStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(TaskStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// Identifies the time action was first taken against the task (start) and/or the
    /// time final action was taken against the task prior to marking it as completed
    /// (end).
    pub fn execution_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("executionPeriod") {
            return Some(Period { value: val });
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
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.intent() {}
        if let Some(_val) = self.part_of() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.insurance() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.performer_type() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.output() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.requester() {
            _val.validate();
        }
        if let Some(_val) = self.location() {
            _val.validate();
        }
        if let Some(_val) = self._authored_on() {
            _val.validate();
        }
        if let Some(_val) = self.instantiates_uri() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.status_reason() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self._intent() {
            _val.validate();
        }
        if let Some(_val) = self._instantiates_uri() {
            _val.validate();
        }
        if let Some(_val) = self.reason_code() {
            _val.validate();
        }
        if let Some(_val) = self.restriction() {
            _val.validate();
        }
        if let Some(_val) = self.input() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.authored_on() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.reason_reference() {
            _val.validate();
        }
        if let Some(_val) = self.note() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.last_modified() {}
        if let Some(_val) = self.code() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.business_status() {
            _val.validate();
        }
        if let Some(_val) = self.encounter() {
            _val.validate();
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.fhir_for() {
            _val.validate();
        }
        if let Some(_val) = self.based_on() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._priority() {
            _val.validate();
        }
        if let Some(_val) = self.owner() {
            _val.validate();
        }
        if let Some(_val) = self._last_modified() {
            _val.validate();
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.instantiates_canonical() {}
        if let Some(_val) = self.priority() {}
        if let Some(_val) = self.group_identifier() {
            _val.validate();
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.focus() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.execution_period() {
            _val.validate();
        }
        if let Some(_val) = self.relevant_history() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
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
}
