#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::PractitionerRole_AvailableTime::PractitionerRole_AvailableTime;
use crate::model::PractitionerRole_NotAvailable::PractitionerRole_NotAvailable;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A specific set of Roles/Locations/specialties/services that a practitioner may
/// perform at an organization for a period of time.

#[derive(Debug)]
pub struct PractitionerRole<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl PractitionerRole<'_> {
    pub fn new(value: &Value) -> PractitionerRole {
        PractitionerRole {
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

    /// Extensions for availabilityExceptions
    pub fn _availability_exceptions(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_availabilityExceptions") {
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

    /// Whether this practitioner role record is in active use.
    pub fn active(&self) -> Option<bool> {
        if let Some(val) = self.value.get("active") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// A description of site availability exceptions, e.g. public holiday availability.
    /// Succinctly describing all possible exceptions to normal site availability as
    /// details in the available Times and not available Times.
    pub fn availability_exceptions(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("availabilityExceptions") {
            return Some(string);
        }
        return None;
    }

    /// A collection of times the practitioner is available or performing this role at
    /// the location and/or healthcareservice.
    pub fn available_time(&self) -> Option<Vec<PractitionerRole_AvailableTime>> {
        if let Some(Value::Array(val)) = self.value.get("availableTime") {
            return Some(
                val.into_iter()
                    .map(|e| PractitionerRole_AvailableTime {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Roles which this practitioner is authorized to perform for the organization.
    pub fn code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("code") {
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

    /// Technical endpoints providing access to services operated for the practitioner
    /// with this role.
    pub fn endpoint(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("endpoint") {
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

    /// The list of healthcare services that this worker provides for this role's
    /// Organization/Location(s).
    pub fn healthcare_service(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("healthcareService") {
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

    /// Business Identifiers that are specific to a role/location.
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

    /// The location(s) at which this practitioner provides care.
    pub fn location(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("location") {
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

    /// The practitioner is not available or performing this role during this period of
    /// time due to the provided reason.
    pub fn not_available(&self) -> Option<Vec<PractitionerRole_NotAvailable>> {
        if let Some(Value::Array(val)) = self.value.get("notAvailable") {
            return Some(
                val.into_iter()
                    .map(|e| PractitionerRole_NotAvailable {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The organization where the Practitioner performs the roles associated.
    pub fn organization(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("organization") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The period during which the person is authorized to act as a practitioner in
    /// these role(s) for the organization.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Practitioner that is able to provide the defined services for the organization.
    pub fn practitioner(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("practitioner") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Specific specialty of the practitioner.
    pub fn specialty(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("specialty") {
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

    /// Contact details that are specific to the role/location/service.
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
        if let Some(_val) = self._availability_exceptions() {
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
        if let Some(_val) = self.availability_exceptions() {}
        if let Some(_val) = self.available_time() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.code() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.endpoint() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.healthcare_service() {
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
        if let Some(_val) = self.location() {
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
        if let Some(_val) = self.not_available() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.organization() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.practitioner() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.specialty() {
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
pub struct PractitionerRoleBuilder {
    pub(crate) value: Value,
}

impl PractitionerRoleBuilder {
    pub fn build(&self) -> PractitionerRole {
        PractitionerRole {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: PractitionerRole) -> PractitionerRoleBuilder {
        PractitionerRoleBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> PractitionerRoleBuilder {
        let mut __value: Value = json!({});
        return PractitionerRoleBuilder { value: __value };
    }

    pub fn _active<'a>(&'a mut self, val: Element) -> &'a mut PractitionerRoleBuilder {
        self.value["_active"] = json!(val.value);
        return self;
    }

    pub fn _availability_exceptions<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut PractitionerRoleBuilder {
        self.value["_availabilityExceptions"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut PractitionerRoleBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut PractitionerRoleBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn active<'a>(&'a mut self, val: bool) -> &'a mut PractitionerRoleBuilder {
        self.value["active"] = json!(val);
        return self;
    }

    pub fn availability_exceptions<'a>(&'a mut self, val: &str) -> &'a mut PractitionerRoleBuilder {
        self.value["availabilityExceptions"] = json!(val);
        return self;
    }

    pub fn available_time<'a>(
        &'a mut self,
        val: Vec<PractitionerRole_AvailableTime>,
    ) -> &'a mut PractitionerRoleBuilder {
        self.value["availableTime"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn code<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut PractitionerRoleBuilder {
        self.value["code"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut PractitionerRoleBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn endpoint<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut PractitionerRoleBuilder {
        self.value["endpoint"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut PractitionerRoleBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn healthcare_service<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut PractitionerRoleBuilder {
        self.value["healthcareService"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut PractitionerRoleBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut PractitionerRoleBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut PractitionerRoleBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut PractitionerRoleBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn location<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut PractitionerRoleBuilder {
        self.value["location"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut PractitionerRoleBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut PractitionerRoleBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn not_available<'a>(
        &'a mut self,
        val: Vec<PractitionerRole_NotAvailable>,
    ) -> &'a mut PractitionerRoleBuilder {
        self.value["notAvailable"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn organization<'a>(&'a mut self, val: Reference) -> &'a mut PractitionerRoleBuilder {
        self.value["organization"] = json!(val.value);
        return self;
    }

    pub fn period<'a>(&'a mut self, val: Period) -> &'a mut PractitionerRoleBuilder {
        self.value["period"] = json!(val.value);
        return self;
    }

    pub fn practitioner<'a>(&'a mut self, val: Reference) -> &'a mut PractitionerRoleBuilder {
        self.value["practitioner"] = json!(val.value);
        return self;
    }

    pub fn specialty<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut PractitionerRoleBuilder {
        self.value["specialty"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn telecom<'a>(&'a mut self, val: Vec<ContactPoint>) -> &'a mut PractitionerRoleBuilder {
        self.value["telecom"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut PractitionerRoleBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }
}
