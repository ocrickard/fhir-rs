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
use crate::model::Patient_Communication::Patient_Communication;
use crate::model::Patient_Contact::Patient_Contact;
use crate::model::Patient_Link::Patient_Link;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Demographics and other administrative information about an individual or animal
/// receiving care or other health-related services.

#[derive(Debug)]
pub struct Patient<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Patient<'_> {
    pub fn new(value: &Value) -> Patient {
        Patient {
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

    /// Extensions for deceasedBoolean
    pub fn _deceased_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_deceasedBoolean") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for deceasedDateTime
    pub fn _deceased_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_deceasedDateTime") {
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

    /// Extensions for multipleBirthBoolean
    pub fn _multiple_birth_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_multipleBirthBoolean") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for multipleBirthInteger
    pub fn _multiple_birth_integer(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_multipleBirthInteger") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Whether this patient record is in active use.   Many systems use this property
    /// to mark as non-current patients, such as those that have not been seen for a
    /// period of time based on an organization's business rules.    It is often used to
    /// filter patient lists to exclude inactive patients    Deceased patients may also
    /// be marked as inactive for the same reasons, but may be active for some time
    /// after death.
    pub fn active(&self) -> Option<bool> {
        if let Some(val) = self.value.get("active") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// An address for the individual.
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

    /// The date of birth for the individual.
    pub fn birth_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("birthDate") {
            return Some(string);
        }
        return None;
    }

    /// A language which may be used to communicate with the patient about his or her
    /// health.
    pub fn communication(&self) -> Option<Vec<Patient_Communication>> {
        if let Some(Value::Array(val)) = self.value.get("communication") {
            return Some(
                val.into_iter()
                    .map(|e| Patient_Communication {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A contact party (e.g. guardian, partner, friend) for the patient.
    pub fn contact(&self) -> Option<Vec<Patient_Contact>> {
        if let Some(Value::Array(val)) = self.value.get("contact") {
            return Some(
                val.into_iter()
                    .map(|e| Patient_Contact {
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

    /// Indicates if the individual is deceased or not.
    pub fn deceased_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("deceasedBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Indicates if the individual is deceased or not.
    pub fn deceased_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("deceasedDateTime") {
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

    /// Administrative Gender - the gender that the patient is considered to have for
    /// administration and record keeping purposes.
    pub fn gender(&self) -> Option<PatientGender> {
        if let Some(Value::String(val)) = self.value.get("gender") {
            return Some(PatientGender::from_string(&val).unwrap());
        }
        return None;
    }

    /// Patient's nominated care provider.
    pub fn general_practitioner(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("generalPractitioner") {
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// An identifier for this patient.
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

    /// Link to another patient resource that concerns the same actual patient.
    pub fn link(&self) -> Option<Vec<Patient_Link>> {
        if let Some(Value::Array(val)) = self.value.get("link") {
            return Some(
                val.into_iter()
                    .map(|e| Patient_Link {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Organization that is the custodian of the patient record.
    pub fn managing_organization(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("managingOrganization") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// This field contains a patient's most recent marital (civil) status.
    pub fn marital_status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("maritalStatus") {
            return Some(CodeableConcept {
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

    /// Indicates whether the patient is part of a multiple (boolean) or indicates the
    /// actual birth order (integer).
    pub fn multiple_birth_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("multipleBirthBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Indicates whether the patient is part of a multiple (boolean) or indicates the
    /// actual birth order (integer).
    pub fn multiple_birth_integer(&self) -> Option<f64> {
        if let Some(val) = self.value.get("multipleBirthInteger") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// A name associated with the individual.
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

    /// Image of the patient.
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

    /// A contact detail (e.g. a telephone number or an email address) by which the
    /// individual may be contacted.
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
        if let Some(_val) = self._deceased_boolean() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._deceased_date_time() {
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
        if let Some(_val) = self._multiple_birth_boolean() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._multiple_birth_integer() {
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
        if let Some(_val) = self.contact() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.deceased_boolean() {}
        if let Some(_val) = self.deceased_date_time() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.gender() {}
        if let Some(_val) = self.general_practitioner() {
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
        if let Some(_val) = self.link() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.managing_organization() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.marital_status() {
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
        if let Some(_val) = self.multiple_birth_boolean() {}
        if let Some(_val) = self.multiple_birth_integer() {}
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
pub struct PatientBuilder {
    pub(crate) value: Value,
}

impl PatientBuilder {
    pub fn build(&self) -> Patient {
        Patient {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Patient) -> PatientBuilder {
        PatientBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> PatientBuilder {
        let mut __value: Value = json!({});
        return PatientBuilder { value: __value };
    }

    pub fn _active<'a>(&'a mut self, val: Element) -> &'a mut PatientBuilder {
        self.value["_active"] = json!(val.value);
        return self;
    }

    pub fn _birth_date<'a>(&'a mut self, val: Element) -> &'a mut PatientBuilder {
        self.value["_birthDate"] = json!(val.value);
        return self;
    }

    pub fn _deceased_boolean<'a>(&'a mut self, val: Element) -> &'a mut PatientBuilder {
        self.value["_deceasedBoolean"] = json!(val.value);
        return self;
    }

    pub fn _deceased_date_time<'a>(&'a mut self, val: Element) -> &'a mut PatientBuilder {
        self.value["_deceasedDateTime"] = json!(val.value);
        return self;
    }

    pub fn _gender<'a>(&'a mut self, val: Element) -> &'a mut PatientBuilder {
        self.value["_gender"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut PatientBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut PatientBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _multiple_birth_boolean<'a>(&'a mut self, val: Element) -> &'a mut PatientBuilder {
        self.value["_multipleBirthBoolean"] = json!(val.value);
        return self;
    }

    pub fn _multiple_birth_integer<'a>(&'a mut self, val: Element) -> &'a mut PatientBuilder {
        self.value["_multipleBirthInteger"] = json!(val.value);
        return self;
    }

    pub fn active<'a>(&'a mut self, val: bool) -> &'a mut PatientBuilder {
        self.value["active"] = json!(val);
        return self;
    }

    pub fn address<'a>(&'a mut self, val: Vec<Address>) -> &'a mut PatientBuilder {
        self.value["address"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn birth_date<'a>(&'a mut self, val: &str) -> &'a mut PatientBuilder {
        self.value["birthDate"] = json!(val);
        return self;
    }

    pub fn communication<'a>(
        &'a mut self,
        val: Vec<Patient_Communication>,
    ) -> &'a mut PatientBuilder {
        self.value["communication"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contact<'a>(&'a mut self, val: Vec<Patient_Contact>) -> &'a mut PatientBuilder {
        self.value["contact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut PatientBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn deceased_boolean<'a>(&'a mut self, val: bool) -> &'a mut PatientBuilder {
        self.value["deceasedBoolean"] = json!(val);
        return self;
    }

    pub fn deceased_date_time<'a>(&'a mut self, val: &str) -> &'a mut PatientBuilder {
        self.value["deceasedDateTime"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut PatientBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn gender<'a>(&'a mut self, val: PatientGender) -> &'a mut PatientBuilder {
        self.value["gender"] = json!(val.to_string());
        return self;
    }

    pub fn general_practitioner<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut PatientBuilder {
        self.value["generalPractitioner"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut PatientBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut PatientBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut PatientBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut PatientBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn link<'a>(&'a mut self, val: Vec<Patient_Link>) -> &'a mut PatientBuilder {
        self.value["link"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn managing_organization<'a>(&'a mut self, val: Reference) -> &'a mut PatientBuilder {
        self.value["managingOrganization"] = json!(val.value);
        return self;
    }

    pub fn marital_status<'a>(&'a mut self, val: CodeableConcept) -> &'a mut PatientBuilder {
        self.value["maritalStatus"] = json!(val.value);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut PatientBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut PatientBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn multiple_birth_boolean<'a>(&'a mut self, val: bool) -> &'a mut PatientBuilder {
        self.value["multipleBirthBoolean"] = json!(val);
        return self;
    }

    pub fn multiple_birth_integer<'a>(&'a mut self, val: f64) -> &'a mut PatientBuilder {
        self.value["multipleBirthInteger"] = json!(val);
        return self;
    }

    pub fn name<'a>(&'a mut self, val: Vec<HumanName>) -> &'a mut PatientBuilder {
        self.value["name"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn photo<'a>(&'a mut self, val: Vec<Attachment>) -> &'a mut PatientBuilder {
        self.value["photo"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn telecom<'a>(&'a mut self, val: Vec<ContactPoint>) -> &'a mut PatientBuilder {
        self.value["telecom"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut PatientBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }
}

#[derive(Debug)]
pub enum PatientGender {
    Male,
    Female,
    Other,
    Unknown,
}

impl PatientGender {
    pub fn from_string(string: &str) -> Option<PatientGender> {
        match string {
            "male" => Some(PatientGender::Male),
            "female" => Some(PatientGender::Female),
            "other" => Some(PatientGender::Other),
            "unknown" => Some(PatientGender::Unknown),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            PatientGender::Male => "male".to_string(),
            PatientGender::Female => "female".to_string(),
            PatientGender::Other => "other".to_string(),
            PatientGender::Unknown => "unknown".to_string(),
        }
    }
}
