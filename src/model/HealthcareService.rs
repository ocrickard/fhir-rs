#![allow(unused_imports, non_camel_case_types)]

use crate::model::Attachment::Attachment;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::HealthcareService_AvailableTime::HealthcareService_AvailableTime;
use crate::model::HealthcareService_Eligibility::HealthcareService_Eligibility;
use crate::model::HealthcareService_NotAvailable::HealthcareService_NotAvailable;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// The details of a healthcare service available at a location.

#[derive(Debug)]
pub struct HealthcareService<'a> {
    pub value: &'a Value,
}

impl HealthcareService<'_> {
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

    /// Identifies the broad category of service being performed or delivered.
    pub fn category(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("category") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The location(s) where this healthcare service may be provided.
    pub fn location(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("location") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for comment
    pub fn _comment(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_comment") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Collection of characteristics (attributes).
    pub fn characteristic(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("characteristic") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
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

    /// A description of site availability exceptions, e.g. public holiday availability.
    /// Succinctly describing all possible exceptions to normal site availability as
    /// details in the available Times and not available Times.
    pub fn availability_exceptions(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("availabilityExceptions") {
            return Some(string);
        }
        return None;
    }

    /// If there is a photo/symbol associated with this HealthcareService, it may be
    /// included here to facilitate quick identification of the service in a list.
    pub fn photo(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("photo") {
            return Some(Attachment { value: val });
        }
        return None;
    }

    /// Some services are specifically made available in multiple languages, this
    /// property permits a directory to declare the languages this is offered in.
    /// Typically this is only provided where a service operates in communities with
    /// mixed languages used.
    pub fn communication(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("communication") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The specific type of service that may be delivered or performed.
    pub fn fhir_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("type") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
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

    /// Extensions for extraDetails
    pub fn _extra_details(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_extraDetails") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// External identifiers for this item.
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

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The code(s) that detail the conditions under which the healthcare service is
    /// available/offered.
    pub fn service_provision_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("serviceProvisionCode") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
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

    /// Extensions for active
    pub fn _active(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_active") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Ways that the service accepts referrals, if this is not provided then it is
    /// implied that no referral is required.
    pub fn referral_method(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("referralMethod") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Programs that this service is applicable to.
    pub fn program(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("program") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The HealthcareService is not available during this period of time due to the
    /// provided reason.
    pub fn not_available(&self) -> Option<Vec<HealthcareService_NotAvailable>> {
        if let Some(Value::Array(val)) = self.value.get("notAvailable") {
            return Some(
                val.into_iter()
                    .map(|e| HealthcareService_NotAvailable { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Technical endpoints providing access to services operated for the specific
    /// healthcare services defined at this resource.
    pub fn endpoint(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("endpoint") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Any additional description of the service and/or any specific issues not covered
    /// by the other attributes, which can be displayed as further detail under the
    /// serviceName.
    pub fn comment(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("comment") {
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

    /// Indicates whether or not a prospective consumer will require an appointment for
    /// a particular service at a site to be provided by the Organization. Indicates if
    /// an appointment is required for access to this service.
    pub fn appointment_required(&self) -> Option<bool> {
        if let Some(val) = self.value.get("appointmentRequired") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Extensions for appointmentRequired
    pub fn _appointment_required(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_appointmentRequired") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// This flag is used to mark the record to not be used. This is not used when a
    /// center is closed for maintenance, or for holidays, the notAvailable period is to
    /// be used for this.
    pub fn active(&self) -> Option<bool> {
        if let Some(val) = self.value.get("active") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// List of contacts related to this specific healthcare service.
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The location(s) that this service is available to (not where the service is
    /// provided).
    pub fn coverage_area(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("coverageArea") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Further description of the service as it would be presented to a consumer while
    /// searching.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Does this service have specific eligibility requirements that need to be met in
    /// order to use the service?
    pub fn eligibility(&self) -> Option<Vec<HealthcareService_Eligibility>> {
        if let Some(Value::Array(val)) = self.value.get("eligibility") {
            return Some(
                val.into_iter()
                    .map(|e| HealthcareService_Eligibility { value: e })
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
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

    /// The organization that provides this healthcare service.
    pub fn provided_by(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("providedBy") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Extra details about the service that can't be placed in the other fields.
    pub fn extra_details(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("extraDetails") {
            return Some(string);
        }
        return None;
    }

    /// Collection of specialties handled by the service site. This is more of a medical
    /// term.
    pub fn specialty(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("specialty") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A collection of times that the Service Site is available.
    pub fn available_time(&self) -> Option<Vec<HealthcareService_AvailableTime>> {
        if let Some(Value::Array(val)) = self.value.get("availableTime") {
            return Some(
                val.into_iter()
                    .map(|e| HealthcareService_AvailableTime { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for availabilityExceptions
    pub fn _availability_exceptions(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_availabilityExceptions") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.category() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.location() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._comment() {
            _val.validate();
        }
        if let Some(_val) = self.characteristic() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._name() {
            _val.validate();
        }
        if let Some(_val) = self.availability_exceptions() {}
        if let Some(_val) = self.photo() {
            _val.validate();
        }
        if let Some(_val) = self.communication() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.fhir_type() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self._extra_details() {
            _val.validate();
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.service_provision_code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._active() {
            _val.validate();
        }
        if let Some(_val) = self.referral_method() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.program() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.not_available() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.endpoint() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.comment() {}
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.appointment_required() {}
        if let Some(_val) = self._appointment_required() {
            _val.validate();
        }
        if let Some(_val) = self.active() {}
        if let Some(_val) = self.telecom() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.coverage_area() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.eligibility() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.provided_by() {
            _val.validate();
        }
        if let Some(_val) = self.extra_details() {}
        if let Some(_val) = self.specialty() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.available_time() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._availability_exceptions() {
            _val.validate();
        }
        return true;
    }
}
