#![allow(unused_imports, non_camel_case_types)]

use crate::model::Address::Address;
use crate::model::Attachment::Attachment;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::HumanName::HumanName;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Practitioner_Qualification::Practitioner_Qualification;
use crate::model::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A person who is directly or indirectly involved in the provisioning of
/// healthcare.

#[derive(Debug)]
pub struct Practitioner<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Practitioner<'_> {
    pub fn new(value: &Value) -> Practitioner {
        Practitioner {
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

    /// Extensions for birthDate
    pub fn _birth_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_birthDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for gender
    pub fn _gender(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_gender") {
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

    /// Whether this practitioner's record is in active use.
    pub fn active(&self) -> Option<bool> {
        if let Some(val) = self.value.get("active") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Address(es) of the practitioner that are not role specific (typically home
    /// address).   Work addresses are not typically entered in this property as they
    /// are usually role dependent.
    pub fn address(&self) -> Option<Vec<Address>> {
        if let Some(Value::Array(val)) = self.value.get("address") {
            return Some(
                val.into_iter()
                    .map(|e| Address {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The date of birth for the practitioner.
    pub fn birth_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("birthDate") {
            return Some(string);
        }
        return None;
    }

    /// A language the practitioner can use in patient communication.
    pub fn communication(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("communication") {
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

    /// Administrative Gender - the gender that the person is considered to have for
    /// administration and record keeping purposes.
    pub fn gender(&self) -> Option<PractitionerGender> {
        if let Some(Value::String(val)) = self.value.get("gender") {
            return Some(PractitionerGender::from_string(&val).unwrap());
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

    /// An identifier that applies to this person in this role.
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

    /// The name(s) associated with the practitioner.
    pub fn name(&self) -> Option<Vec<HumanName>> {
        if let Some(Value::Array(val)) = self.value.get("name") {
            return Some(
                val.into_iter()
                    .map(|e| HumanName {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Image of the person.
    pub fn photo(&self) -> Option<Vec<Attachment>> {
        if let Some(Value::Array(val)) = self.value.get("photo") {
            return Some(
                val.into_iter()
                    .map(|e| Attachment {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The official certifications, training, and licenses that authorize or otherwise
    /// pertain to the provision of care by the practitioner.  For example, a medical
    /// license issued by a medical board authorizing the practitioner to practice
    /// medicine within a certian locality.
    pub fn qualification(&self) -> Option<Vec<Practitioner_Qualification>> {
        if let Some(Value::Array(val)) = self.value.get("qualification") {
            return Some(
                val.into_iter()
                    .map(|e| Practitioner_Qualification {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A contact detail for the practitioner, e.g. a telephone number or an email
    /// address.
    pub fn telecom(&self) -> Option<Vec<ContactPoint>> {
        if let Some(Value::Array(val)) = self.value.get("telecom") {
            return Some(
                val.into_iter()
                    .map(|e| ContactPoint {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._active() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._birth_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._gender() {
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
        if let Some(_val) = self.active() {}
        if let Some(_val) = self.address() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.birth_date() {}
        if let Some(_val) = self.communication() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
        if let Some(_val) = self.gender() {}
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
        if let Some(_val) = self.name() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.photo() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.qualification() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.telecom() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
pub struct PractitionerBuilder {
    pub(crate) value: Value,
}

impl PractitionerBuilder {
    pub fn build(&self) -> Practitioner {
        Practitioner {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Practitioner) -> PractitionerBuilder {
        PractitionerBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> PractitionerBuilder {
        let mut __value: Value = json!({});
        return PractitionerBuilder { value: __value };
    }

    pub fn _active<'a>(&'a mut self, val: Element) -> &'a mut PractitionerBuilder {
        self.value["_active"] = json!(val.value);
        return self;
    }

    pub fn _birth_date<'a>(&'a mut self, val: Element) -> &'a mut PractitionerBuilder {
        self.value["_birthDate"] = json!(val.value);
        return self;
    }

    pub fn _gender<'a>(&'a mut self, val: Element) -> &'a mut PractitionerBuilder {
        self.value["_gender"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut PractitionerBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut PractitionerBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn active<'a>(&'a mut self, val: bool) -> &'a mut PractitionerBuilder {
        self.value["active"] = json!(val);
        return self;
    }

    pub fn address<'a>(&'a mut self, val: Vec<Address>) -> &'a mut PractitionerBuilder {
        self.value["address"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn birth_date<'a>(&'a mut self, val: &str) -> &'a mut PractitionerBuilder {
        self.value["birthDate"] = json!(val);
        return self;
    }

    pub fn communication<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut PractitionerBuilder {
        self.value["communication"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut PractitionerBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut PractitionerBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn gender<'a>(&'a mut self, val: PractitionerGender) -> &'a mut PractitionerBuilder {
        self.value["gender"] = json!(val.to_string());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut PractitionerBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut PractitionerBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut PractitionerBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut PractitionerBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut PractitionerBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut PractitionerBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: Vec<HumanName>) -> &'a mut PractitionerBuilder {
        self.value["name"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn photo<'a>(&'a mut self, val: Vec<Attachment>) -> &'a mut PractitionerBuilder {
        self.value["photo"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn qualification<'a>(
        &'a mut self,
        val: Vec<Practitioner_Qualification>,
    ) -> &'a mut PractitionerBuilder {
        self.value["qualification"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn telecom<'a>(&'a mut self, val: Vec<ContactPoint>) -> &'a mut PractitionerBuilder {
        self.value["telecom"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut PractitionerBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }
}

#[derive(Debug)]
pub enum PractitionerGender {
    Male,
    Female,
    Other,
    Unknown,
}

impl PractitionerGender {
    pub fn from_string(string: &str) -> Option<PractitionerGender> {
        match string {
            "male" => Some(PractitionerGender::Male),
            "female" => Some(PractitionerGender::Female),
            "other" => Some(PractitionerGender::Other),
            "unknown" => Some(PractitionerGender::Unknown),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            PractitionerGender::Male => "male".to_string(),
            PractitionerGender::Female => "female".to_string(),
            PractitionerGender::Other => "other".to_string(),
            PractitionerGender::Unknown => "unknown".to_string(),
        }
    }
}
