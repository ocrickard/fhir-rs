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
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The regulatory authorization of a medicinal product.

#[derive(Debug)]
pub struct MedicinalProductAuthorization<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicinalProductAuthorization<'_> {
    pub fn new(value: &Value) -> MedicinalProductAuthorization {
        MedicinalProductAuthorization {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for dateOfFirstAuthorization
    pub fn _date_of_first_authorization(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_dateOfFirstAuthorization") {
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

    /// Extensions for internationalBirthDate
    pub fn _international_birth_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_internationalBirthDate") {
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

    /// Extensions for restoreDate
    pub fn _restore_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_restoreDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for statusDate
    pub fn _status_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_statusDate") {
            return Some(Element {
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

    /// The country in which the marketing authorization has been granted.
    pub fn country(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("country") {
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

    /// A period of time after authorization before generic product applicatiosn can be
    /// submitted.
    pub fn data_exclusivity_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("dataExclusivityPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
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

    /// Marketing Authorization Holder.
    pub fn holder(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("holder") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// Business identifier for the marketing authorization, as assigned by a regulator.
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

    /// Date of first marketing authorization for a company's new medicinal product in
    /// any country in the World.
    pub fn international_birth_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("internationalBirthDate") {
            return Some(string);
        }
        return None;
    }

    /// Jurisdiction within a country.
    pub fn jurisdiction(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("jurisdiction") {
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

    /// Authorization in areas within a country.
    pub fn jurisdictional_authorization(
        &self,
    ) -> Option<Vec<MedicinalProductAuthorization_JurisdictionalAuthorization>> {
        if let Some(Value::Array(val)) = self.value.get("jurisdictionalAuthorization") {
            return Some(
                val.into_iter()
                    .map(
                        |e| MedicinalProductAuthorization_JurisdictionalAuthorization {
                            value: Cow::Borrowed(e),
                        },
                    )
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

    /// The legal framework against which this authorization is granted.
    pub fn legal_basis(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("legalBasis") {
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

    /// The regulatory procedure for granting or amending a marketing authorization.
    pub fn procedure(&self) -> Option<MedicinalProductAuthorization_Procedure> {
        if let Some(val) = self.value.get("procedure") {
            return Some(MedicinalProductAuthorization_Procedure {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Medicines Regulatory Agency.
    pub fn regulator(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("regulator") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// The status of the marketing authorization.
    pub fn status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("status") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// The medicinal product that is being authorized.
    pub fn subject(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subject") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// The beginning of the time period in which the marketing authorization is in the
    /// specific status shall be specified A complete date consisting of day, month and
    /// year shall be specified using the ISO 8601 date format.
    pub fn validity_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("validityPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._date_of_first_authorization() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._international_birth_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._restore_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.country() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.data_exclusivity_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.date_of_first_authorization() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.holder() {
            if !_val.validate() {
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
        if let Some(_val) = self.international_birth_date() {}
        if let Some(_val) = self.jurisdiction() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.jurisdictional_authorization() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.legal_basis() {
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
        if let Some(_val) = self.procedure() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.regulator() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.restore_date() {}
        if let Some(_val) = self.status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status_date() {}
        if let Some(_val) = self.subject() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.validity_period() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicinalProductAuthorizationBuilder {
    pub(crate) value: Value,
}

impl MedicinalProductAuthorizationBuilder {
    pub fn build(&self) -> MedicinalProductAuthorization {
        MedicinalProductAuthorization {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: MedicinalProductAuthorization) -> MedicinalProductAuthorizationBuilder {
        MedicinalProductAuthorizationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MedicinalProductAuthorizationBuilder {
        let mut __value: Value = json!({});
        return MedicinalProductAuthorizationBuilder { value: __value };
    }

    pub fn _date_of_first_authorization<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["_dateOfFirstAuthorization"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _international_birth_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["_internationalBirthDate"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _restore_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["_restoreDate"] = json!(val.value);
        return self;
    }

    pub fn _status_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["_statusDate"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn country<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["country"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn data_exclusivity_period<'a>(
        &'a mut self,
        val: Period,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["dataExclusivityPeriod"] = json!(val.value);
        return self;
    }

    pub fn date_of_first_authorization<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["dateOfFirstAuthorization"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn holder<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["holder"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn international_birth_date<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["internationalBirthDate"] = json!(val);
        return self;
    }

    pub fn jurisdiction<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["jurisdiction"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn jurisdictional_authorization<'a>(
        &'a mut self,
        val: Vec<MedicinalProductAuthorization_JurisdictionalAuthorization>,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["jurisdictionalAuthorization"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn legal_basis<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["legalBasis"] = json!(val.value);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn procedure<'a>(
        &'a mut self,
        val: MedicinalProductAuthorization_Procedure,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["procedure"] = json!(val.value);
        return self;
    }

    pub fn regulator<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["regulator"] = json!(val.value);
        return self;
    }

    pub fn restore_date<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["restoreDate"] = json!(val);
        return self;
    }

    pub fn status<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["status"] = json!(val.value);
        return self;
    }

    pub fn status_date<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["statusDate"] = json!(val);
        return self;
    }

    pub fn subject<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["subject"] = json!(val.value);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn validity_period<'a>(
        &'a mut self,
        val: Period,
    ) -> &'a mut MedicinalProductAuthorizationBuilder {
        self.value["validityPeriod"] = json!(val.value);
        return self;
    }
}
