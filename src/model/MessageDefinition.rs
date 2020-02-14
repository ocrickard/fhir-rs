#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Coding::Coding;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::MessageDefinition_AllowedResponse::MessageDefinition_AllowedResponse;
use crate::model::MessageDefinition_Focus::MessageDefinition_Focus;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::ResourceList::ResourceList;
use crate::model::UsageContext::UsageContext;
use serde_json::value::Value;

/// Defines the characteristics of a message that can be shared between systems,
/// including the type of event that initiates the message, the content to be
/// transmitted and what response(s), if any, are permitted.

#[derive(Debug)]
pub struct MessageDefinition<'a> {
    pub value: &'a Value,
}

impl MessageDefinition<'_> {
    /// A copyright statement relating to the message definition and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the message definition.
    pub fn copyright(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("copyright") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for category
    pub fn _category(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_category") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for url
    pub fn _url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_url") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for date
    pub fn _date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_date") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Identifies the resource (or resources) that are being addressed by the event.
    /// For example, the Encounter for an admit message or two Account records for a
    /// merge.
    pub fn focus(&self) -> Option<Vec<MessageDefinition_Focus>> {
        if let Some(Value::Array(val)) = self.value.get("focus") {
            return Some(
                val.into_iter()
                    .map(|e| MessageDefinition_Focus { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for copyright
    pub fn _copyright(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_copyright") {
            return Some(Element { value: val });
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

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The business identifier that is used to reference the MessageDefinition and *is*
    /// expected to be consistent from server to server.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    /// Explanation of why this message definition is needed and why it has been
    /// designed as it has.
    pub fn purpose(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("purpose") {
            return Some(string);
        }
        return None;
    }

    /// Event code or link to the EventDefinition.
    pub fn event_uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("eventUri") {
            return Some(string);
        }
        return None;
    }

    /// The date  (and optionally time) when the message definition was published. The
    /// date must change when the business version changes and it must change if the
    /// status code changes. In addition, it should change when the substantive content
    /// of the message definition changes.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
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

    /// A free text natural language description of the message definition from a
    /// consumer's perspective.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
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

    /// The MessageDefinition that is the basis for the contents of this resource.
    pub fn base(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("base") {
            return Some(string);
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

    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate message definition
    /// instances.
    pub fn use_context(&self) -> Option<Vec<UsageContext>> {
        if let Some(Value::Array(val)) = self.value.get("useContext") {
            return Some(
                val.into_iter()
                    .map(|e| UsageContext { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Declare at a message definition level whether a response is required or only
    /// upon error or success, or never.
    pub fn response_required(&self) -> Option<MessageDefinitionResponseRequired> {
        if let Some(Value::String(val)) = self.value.get("responseRequired") {
            return Some(MessageDefinitionResponseRequired::from_string(&val).unwrap());
        }
        return None;
    }

    /// A natural language name identifying the message definition. This name should be
    /// usable as an identifier for the module by machine processing applications such
    /// as code generation.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
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

    /// Extensions for publisher
    pub fn _publisher(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_publisher") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for purpose
    pub fn _purpose(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_purpose") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Event code or link to the EventDefinition.
    pub fn event_coding(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("eventCoding") {
            return Some(Coding { value: val });
        }
        return None;
    }

    /// Extensions for version
    pub fn _version(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_version") {
            return Some(Element { value: val });
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

    /// A Boolean value to indicate that this message definition is authored for testing
    /// purposes (or education/evaluation/marketing) and is not intended to be used for
    /// genuine usage.
    pub fn experimental(&self) -> Option<bool> {
        if let Some(val) = self.value.get("experimental") {
            return Some(val.as_bool().unwrap());
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

    /// The name of the organization or individual that published the message
    /// definition.
    pub fn publisher(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("publisher") {
            return Some(string);
        }
        return None;
    }

    /// The status of this message definition. Enables tracking the life-cycle of the
    /// content.
    pub fn status(&self) -> Option<MessageDefinitionStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(MessageDefinitionStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// A legal or geographic region in which the message definition is intended to be
    /// used.
    pub fn jurisdiction(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("jurisdiction") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A formal identifier that is used to identify this message definition when it is
    /// represented in other formats, or referenced in a specification, model, design or
    /// an instance.
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// A MessageDefinition that is superseded by this definition.
    pub fn replaces(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("replaces") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Identifies a protocol or workflow that this MessageDefinition represents a step
    /// in.
    pub fn parent(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("parent") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A short, descriptive, user-friendly title for the message definition.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub fn contact(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("contact") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for experimental
    pub fn _experimental(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_experimental") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The impact of the content of the message.
    pub fn category(&self) -> Option<MessageDefinitionCategory> {
        if let Some(Value::String(val)) = self.value.get("category") {
            return Some(MessageDefinitionCategory::from_string(&val).unwrap());
        }
        return None;
    }

    /// Extensions for responseRequired
    pub fn _response_required(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_responseRequired") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Indicates what types of messages may be sent as an application-level response to
    /// this message.
    pub fn allowed_response(&self) -> Option<Vec<MessageDefinition_AllowedResponse>> {
        if let Some(Value::Array(val)) = self.value.get("allowedResponse") {
            return Some(
                val.into_iter()
                    .map(|e| MessageDefinition_AllowedResponse { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Canonical reference to a GraphDefinition. If a URL is provided, it is the
    /// canonical reference to a [[[GraphDefinition]]] that it controls what resources
    /// are to be added to the bundle when building the document. The GraphDefinition
    /// can also specify profiles that apply to the various resources.
    pub fn graph(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("graph") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The identifier that is used to identify this version of the message definition
    /// when it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the message definition author and is not expected to
    /// be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
    /// managed version is not available. There is also no expectation that versions can
    /// be placed in a lexicographical sequence.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
            return Some(string);
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

    /// Extensions for eventUri
    pub fn _event_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_eventUri") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.copyright() {}
        if let Some(_val) = self._category() {
            _val.validate();
        }
        if let Some(_val) = self._url() {
            _val.validate();
        }
        if let Some(_val) = self._date() {
            _val.validate();
        }
        if let Some(_val) = self.focus() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._copyright() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._name() {
            _val.validate();
        }
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.purpose() {}
        if let Some(_val) = self.event_uri() {}
        if let Some(_val) = self.date() {}
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.description() {}
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.base() {}
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.use_context() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._title() {
            _val.validate();
        }
        if let Some(_val) = self.response_required() {}
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self._publisher() {
            _val.validate();
        }
        if let Some(_val) = self._purpose() {
            _val.validate();
        }
        if let Some(_val) = self.event_coding() {
            _val.validate();
        }
        if let Some(_val) = self._version() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.experimental() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.publisher() {}
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.jurisdiction() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.replaces() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.parent() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.contact() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._experimental() {
            _val.validate();
        }
        if let Some(_val) = self.category() {}
        if let Some(_val) = self._response_required() {
            _val.validate();
        }
        if let Some(_val) = self.allowed_response() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.graph() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.version() {}
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self._event_uri() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum MessageDefinitionResponseRequired {
    Always,
    OnError,
    Never,
    OnSuccess,
}

impl MessageDefinitionResponseRequired {
    pub fn from_string(string: &str) -> Option<MessageDefinitionResponseRequired> {
        match string {
            "always" => Some(MessageDefinitionResponseRequired::Always),
            "on-error" => Some(MessageDefinitionResponseRequired::OnError),
            "never" => Some(MessageDefinitionResponseRequired::Never),
            "on-success" => Some(MessageDefinitionResponseRequired::OnSuccess),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum MessageDefinitionStatus {
    Draft,
    Active,
    Retired,
    Unknown,
}

impl MessageDefinitionStatus {
    pub fn from_string(string: &str) -> Option<MessageDefinitionStatus> {
        match string {
            "draft" => Some(MessageDefinitionStatus::Draft),
            "active" => Some(MessageDefinitionStatus::Active),
            "retired" => Some(MessageDefinitionStatus::Retired),
            "unknown" => Some(MessageDefinitionStatus::Unknown),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum MessageDefinitionCategory {
    Consequence,
    Currency,
    Notification,
}

impl MessageDefinitionCategory {
    pub fn from_string(string: &str) -> Option<MessageDefinitionCategory> {
        match string {
            "consequence" => Some(MessageDefinitionCategory::Consequence),
            "currency" => Some(MessageDefinitionCategory::Currency),
            "notification" => Some(MessageDefinitionCategory::Notification),
            _ => None,
        }
    }
}
