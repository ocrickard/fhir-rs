#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::NamingSystem_UniqueId::NamingSystem_UniqueId;
use crate::model::Narrative::Narrative;
use crate::model::ResourceList::ResourceList;
use crate::model::UsageContext::UsageContext;
use serde_json::value::Value;

/// A curated namespace that issues unique symbols within that namespace for the
/// identification of concepts, people, devices, etc.  Represents a "System" used
/// within the Identifier and Coding data types.

#[derive(Debug)]
pub struct NamingSystem<'a> {
    pub value: &'a Value,
}

impl NamingSystem<'_> {
    /// Provides guidance on the use of the namespace, including the handling of
    /// formatting characters, use of upper vs. lower case, etc.
    pub fn usage(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("usage") {
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

    /// The name of the organization or individual that published the naming system.
    pub fn publisher(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("publisher") {
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

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for kind
    pub fn _kind(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_kind") {
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

    /// Extensions for publisher
    pub fn _publisher(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_publisher") {
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

    /// Extensions for responsible
    pub fn _responsible(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_responsible") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A legal or geographic region in which the naming system is intended to be used.
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// Indicates the purpose for the naming system - what kinds of things does it make
    /// unique?
    pub fn kind(&self) -> Option<NamingSystemKind> {
        if let Some(Value::String(val)) = self.value.get("kind") {
            return Some(NamingSystemKind::from_string(&val).unwrap());
        }
        return None;
    }

    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate naming system
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

    /// A natural language name identifying the naming system. This name should be
    /// usable as an identifier for the module by machine processing applications such
    /// as code generation.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
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

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
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

    /// The status of this naming system. Enables tracking the life-cycle of the
    /// content.
    pub fn status(&self) -> Option<NamingSystemStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(NamingSystemStatus::from_string(&val).unwrap());
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

    /// The name of the organization that is responsible for issuing identifiers or
    /// codes for this namespace and ensuring their non-collision.
    pub fn responsible(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("responsible") {
            return Some(string);
        }
        return None;
    }

    /// A free text natural language description of the naming system from a consumer's
    /// perspective. Details about what the namespace identifies including scope,
    /// granularity, version labeling, etc.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
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

    /// Categorizes a naming system for easier search by grouping related naming
    /// systems.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for usage
    pub fn _usage(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_usage") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Indicates how the system may be identified when referenced in electronic
    /// exchange.
    pub fn unique_id(&self) -> Vec<NamingSystem_UniqueId> {
        self.value
            .get("uniqueId")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| NamingSystem_UniqueId { value: e })
            .collect::<Vec<_>>()
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The date  (and optionally time) when the naming system was published. The date
    /// must change when the business version changes and it must change if the status
    /// code changes. In addition, it should change when the substantive content of the
    /// naming system changes.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.usage() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.publisher() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self._kind() {
            _val.validate();
        }
        if let Some(_val) = self._date() {
            _val.validate();
        }
        if let Some(_val) = self.contact() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._publisher() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self._responsible() {
            _val.validate();
        }
        if let Some(_val) = self.jurisdiction() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.kind() {}
        if let Some(_val) = self.use_context() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self._name() {
            _val.validate();
        }
        if let Some(_val) = self.responsible() {}
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.fhir_type() {
            _val.validate();
        }
        if let Some(_val) = self._usage() {
            _val.validate();
        }
        let _ = self.unique_id().into_iter().for_each(|e| {
            e.validate();
        });
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

#[derive(Debug)]
pub enum NamingSystemKind {
    Codesystem,
    Identifier,
    Root,
}

impl NamingSystemKind {
    pub fn from_string(string: &str) -> Option<NamingSystemKind> {
        match string {
            "codesystem" => Some(NamingSystemKind::Codesystem),
            "identifier" => Some(NamingSystemKind::Identifier),
            "root" => Some(NamingSystemKind::Root),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum NamingSystemStatus {
    Draft,
    Active,
    Retired,
    Unknown,
}

impl NamingSystemStatus {
    pub fn from_string(string: &str) -> Option<NamingSystemStatus> {
        match string {
            "draft" => Some(NamingSystemStatus::Draft),
            "active" => Some(NamingSystemStatus::Active),
            "retired" => Some(NamingSystemStatus::Retired),
            "unknown" => Some(NamingSystemStatus::Unknown),
            _ => None,
        }
    }
}
