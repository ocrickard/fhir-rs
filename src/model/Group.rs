#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Group_Characteristic::Group_Characteristic;
use crate::model::Group_Member::Group_Member;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Represents a defined collection of entities that may be discussed or acted upon
/// collectively but which are not expected to act collectively, and are not
/// formally or legally recognized; i.e. a collection of entities that isn't an
/// Organization.

#[derive(Debug)]
pub struct Group<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Group<'_> {
    pub fn new(value: &Value) -> Group {
        Group {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for active
    pub fn _active(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_active") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for actual
    pub fn _actual(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_actual") {
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

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for quantity
    pub fn _quantity(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_quantity") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates whether the record for the group is available for use or is merely
    /// being retained for historical purposes.
    pub fn active(&self) -> Option<bool> {
        if let Some(val) = self.value.get("active") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// If true, indicates that the resource refers to a specific group of real
    /// individuals.  If false, the group defines a set of intended individuals.
    pub fn actual(&self) -> Option<bool> {
        if let Some(val) = self.value.get("actual") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Identifies traits whose presence r absence is shared by members of the group.
    pub fn characteristic(&self) -> Option<Vec<Group_Characteristic>> {
        if let Some(Value::Array(val)) = self.value.get("characteristic") {
            return Some(
                val.into_iter()
                    .map(|e| Group_Characteristic {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Provides a specific type of resource the group includes; e.g. "cow", "syringe",
    /// etc.
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

    /// A unique business identifier for this group.
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

    /// Entity responsible for defining and maintaining Group characteristics and/or
    /// registered members.
    pub fn managing_entity(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("managingEntity") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifies the resource instances that are members of the group.
    pub fn member(&self) -> Option<Vec<Group_Member>> {
        if let Some(Value::Array(val)) = self.value.get("member") {
            return Some(
                val.into_iter()
                    .map(|e| Group_Member {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// A label assigned to the group for human identification and communication.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// A count of the number of resource instances that are part of the group.
    pub fn quantity(&self) -> Option<u64> {
        if let Some(val) = self.value.get("quantity") {
            return Some(val.as_u64().unwrap());
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

    /// Identifies the broad classification of the kind of resources the group includes.
    pub fn fhir_type(&self) -> Option<GroupType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(GroupType::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._active() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._actual() {
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
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.active() {}
        if let Some(_val) = self.actual() {}
        if let Some(_val) = self.characteristic() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
        if let Some(_val) = self.managing_entity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.member() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.quantity() {}
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct GroupBuilder {
    pub(crate) value: Value,
}

impl GroupBuilder {
    pub fn build(&self) -> Group {
        Group {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Group) -> GroupBuilder {
        GroupBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> GroupBuilder {
        let mut __value: Value = json!({});
        return GroupBuilder { value: __value };
    }

    pub fn _active<'a>(&'a mut self, val: Element) -> &'a mut GroupBuilder {
        self.value["_active"] = json!(val.value);
        return self;
    }

    pub fn _actual<'a>(&'a mut self, val: Element) -> &'a mut GroupBuilder {
        self.value["_actual"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut GroupBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut GroupBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut GroupBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _quantity<'a>(&'a mut self, val: Element) -> &'a mut GroupBuilder {
        self.value["_quantity"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut GroupBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn active<'a>(&'a mut self, val: bool) -> &'a mut GroupBuilder {
        self.value["active"] = json!(val);
        return self;
    }

    pub fn actual<'a>(&'a mut self, val: bool) -> &'a mut GroupBuilder {
        self.value["actual"] = json!(val);
        return self;
    }

    pub fn characteristic<'a>(
        &'a mut self,
        val: Vec<Group_Characteristic>,
    ) -> &'a mut GroupBuilder {
        self.value["characteristic"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn code<'a>(&'a mut self, val: CodeableConcept) -> &'a mut GroupBuilder {
        self.value["code"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut GroupBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut GroupBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut GroupBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut GroupBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut GroupBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut GroupBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn managing_entity<'a>(&'a mut self, val: Reference) -> &'a mut GroupBuilder {
        self.value["managingEntity"] = json!(val.value);
        return self;
    }

    pub fn member<'a>(&'a mut self, val: Vec<Group_Member>) -> &'a mut GroupBuilder {
        self.value["member"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut GroupBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut GroupBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut GroupBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn quantity<'a>(&'a mut self, val: u64) -> &'a mut GroupBuilder {
        self.value["quantity"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut GroupBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: GroupType) -> &'a mut GroupBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum GroupType {
    Person,
    Animal,
    Practitioner,
    Device,
    Medication,
    Substance,
}

impl GroupType {
    pub fn from_string(string: &str) -> Option<GroupType> {
        match string {
            "person" => Some(GroupType::Person),
            "animal" => Some(GroupType::Animal),
            "practitioner" => Some(GroupType::Practitioner),
            "device" => Some(GroupType::Device),
            "medication" => Some(GroupType::Medication),
            "substance" => Some(GroupType::Substance),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            GroupType::Person => "person".to_string(),
            GroupType::Animal => "animal".to_string(),
            GroupType::Practitioner => "practitioner".to_string(),
            GroupType::Device => "device".to_string(),
            GroupType::Medication => "medication".to_string(),
            GroupType::Substance => "substance".to_string(),
        }
    }
}
