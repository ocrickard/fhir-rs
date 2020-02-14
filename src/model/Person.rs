#![allow(unused_imports, non_camel_case_types)]

use crate::model::Address::Address;
use crate::model::Attachment::Attachment;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::HumanName::HumanName;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Person_Link::Person_Link;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// Demographics and administrative information about a person independent of a
/// specific health-related context.

#[derive(Debug)]
pub struct Person<'a> {
    pub value: &'a Value,
}

impl Person<'_> {
    /// The birth date for the person.
    pub fn birth_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("birthDate") {
            return Some(string);
        }
        return None;
    }

    /// An image that can be displayed as a thumbnail of the person to enhance the
    /// identification of the individual.
    pub fn photo(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("photo") {
            return Some(Attachment { value: val });
        }
        return None;
    }

    /// Link to a resource that concerns the same actual person.
    pub fn link(&self) -> Option<Vec<Person_Link>> {
        if let Some(Value::Array(val)) = self.value.get("link") {
            return Some(
                val.into_iter()
                    .map(|e| Person_Link { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Identifier for a person within a particular scope.
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

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
        }
        return None;
    }

    /// A name associated with the person.
    pub fn name(&self) -> Option<Vec<HumanName>> {
        if let Some(Value::Array(val)) = self.value.get("name") {
            return Some(
                val.into_iter()
                    .map(|e| HumanName { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Administrative Gender.
    pub fn gender(&self) -> Option<PersonGender> {
        if let Some(Value::String(val)) = self.value.get("gender") {
            return Some(PersonGender::from_string(&val).unwrap());
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

    /// One or more addresses for the person.
    pub fn address(&self) -> Option<Vec<Address>> {
        if let Some(Value::Array(val)) = self.value.get("address") {
            return Some(
                val.into_iter()
                    .map(|e| Address { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The organization that is the custodian of the person record.
    pub fn managing_organization(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("managingOrganization") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Extensions for active
    pub fn _active(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_active") {
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

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for birthDate
    pub fn _birth_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_birthDate") {
            return Some(Element { value: val });
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for gender
    pub fn _gender(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_gender") {
            return Some(Element { value: val });
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

    /// Whether this person's record is in active use.
    pub fn active(&self) -> Option<bool> {
        if let Some(val) = self.value.get("active") {
            return Some(val.as_bool().unwrap());
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

    /// A contact detail for the person, e.g. a telephone number or an email address.
    pub fn telecom(&self) -> Option<Vec<ContactPoint>> {
        if let Some(Value::Array(val)) = self.value.get("telecom") {
            return Some(
                val.into_iter()
                    .map(|e| ContactPoint { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.birth_date() {}
        if let Some(_val) = self.photo() {
            _val.validate();
        }
        if let Some(_val) = self.link() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.name() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.gender() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.address() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.managing_organization() {
            _val.validate();
        }
        if let Some(_val) = self._active() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self._birth_date() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self._gender() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.active() {}
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.telecom() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

#[derive(Debug)]
pub enum PersonGender {
    Male,
    Female,
    Other,
    Unknown,
}

impl PersonGender {
    pub fn from_string(string: &str) -> Option<PersonGender> {
        match string {
            "male" => Some(PersonGender::Male),
            "female" => Some(PersonGender::Female),
            "other" => Some(PersonGender::Other),
            "unknown" => Some(PersonGender::Unknown),
            _ => None,
        }
    }
}
