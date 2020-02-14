#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::MedicinalProductAuthorization_JurisdictionalAuthorization::MedicinalProductAuthorization_JurisdictionalAuthorization;
use crate::model::MedicinalProductAuthorization_Procedure::MedicinalProductAuthorization_Procedure;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// The regulatory authorization of a medicinal product.

#[derive(Debug)]
pub struct MedicinalProductAuthorization<'a> {
    pub value: &'a Value,
}

impl MedicinalProductAuthorization<'_> {
    /// Extensions for statusDate
    pub fn _status_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_statusDate") {
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

    /// Extensions for internationalBirthDate
    pub fn _international_birth_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_internationalBirthDate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Marketing Authorization Holder.
    pub fn holder(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("holder") {
            return Some(Reference { value: val });
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

    /// The regulatory procedure for granting or amending a marketing authorization.
    pub fn procedure(&self) -> Option<MedicinalProductAuthorization_Procedure> {
        if let Some(val) = self.value.get("procedure") {
            return Some(MedicinalProductAuthorization_Procedure { value: val });
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

    /// Business identifier for the marketing authorization, as assigned by a regulator.
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

    /// Jurisdiction within a country.
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

    /// The date when the first authorization was granted by a Medicines Regulatory
    /// Agency.
    pub fn date_of_first_authorization(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("dateOfFirstAuthorization") {
            return Some(string);
        }
        return None;
    }

    /// Authorization in areas within a country.
    pub fn jurisdictional_authorization(
        &self,
    ) -> Option<Vec<MedicinalProductAuthorization_JurisdictionalAuthorization>> {
        if let Some(Value::Array(val)) = self.value.get("jurisdictionalAuthorization") {
            return Some(
                val.into_iter()
                    .map(|e| MedicinalProductAuthorization_JurisdictionalAuthorization { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The country in which the marketing authorization has been granted.
    pub fn country(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("country") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for dateOfFirstAuthorization
    pub fn _date_of_first_authorization(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_dateOfFirstAuthorization") {
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

    /// The medicinal product that is being authorized.
    pub fn subject(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subject") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Extensions for restoreDate
    pub fn _restore_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_restoreDate") {
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

    /// Medicines Regulatory Agency.
    pub fn regulator(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("regulator") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Date of first marketing authorization for a company's new medicinal product in
    /// any country in the World.
    pub fn international_birth_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("internationalBirthDate") {
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

    /// A period of time after authorization before generic product applicatiosn can be
    /// submitted.
    pub fn data_exclusivity_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("dataExclusivityPeriod") {
            return Some(Period { value: val });
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

    /// The date at which the given status has become applicable.
    pub fn status_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("statusDate") {
            return Some(string);
        }
        return None;
    }

    /// The date when a suspended the marketing or the marketing authorization of the
    /// product is anticipated to be restored.
    pub fn restore_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("restoreDate") {
            return Some(string);
        }
        return None;
    }

    /// The legal framework against which this authorization is granted.
    pub fn legal_basis(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("legalBasis") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The beginning of the time period in which the marketing authorization is in the
    /// specific status shall be specified A complete date consisting of day, month and
    /// year shall be specified using the ISO 8601 date format.
    pub fn validity_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("validityPeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// The status of the marketing authorization.
    pub fn status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("status") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._status_date() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._international_birth_date() {
            _val.validate();
        }
        if let Some(_val) = self.holder() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.procedure() {
            _val.validate();
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.text() {
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
        if let Some(_val) = self.jurisdiction() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.date_of_first_authorization() {}
        if let Some(_val) = self.jurisdictional_authorization() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.country() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._date_of_first_authorization() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.subject() {
            _val.validate();
        }
        if let Some(_val) = self._restore_date() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.regulator() {
            _val.validate();
        }
        if let Some(_val) = self.international_birth_date() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.data_exclusivity_period() {
            _val.validate();
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.status_date() {}
        if let Some(_val) = self.restore_date() {}
        if let Some(_val) = self.legal_basis() {
            _val.validate();
        }
        if let Some(_val) = self.validity_period() {
            _val.validate();
        }
        if let Some(_val) = self.status() {
            _val.validate();
        }
        return true;
    }
}
