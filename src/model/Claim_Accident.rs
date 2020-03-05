#![allow(unused_imports, non_camel_case_types)]

use crate::model::Address::Address;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A provider issued list of professional services and products which have been
/// provided, or are to be provided, to a patient which is sent to an insurer for
/// reimbursement.

#[derive(Debug)]
pub struct Claim_Accident<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Claim_Accident<'_> {
    pub fn new(value: &Value) -> Claim_Accident {
        Claim_Accident {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for date
    pub fn _date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_date") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Date of an accident event  related to the products and services contained in the
    /// claim.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element. To make the use of extensions safe and manageable,
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// The physical location of the accident event.
    pub fn location_address(&self) -> Option<Address> {
        if let Some(val) = self.value.get("locationAddress") {
            return Some(Address {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The physical location of the accident event.
    pub fn location_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("locationReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element and that modifies the understanding of the element in
    /// which it is contained and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer can define an extension, there is a set of requirements that SHALL
    /// be met as part of the definition of the extension. Applications processing a
    /// resource are required to check for modifier extensions.    Modifier extensions
    /// SHALL NOT change the meaning of any elements on Resource or DomainResource
    /// (including cannot change the meaning of modifierExtension itself).
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

    /// The type or context of the accident event for the purposes of selection of
    /// potential insurance coverages and determination of coordination between
    /// insurers.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.location_address() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.location_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Claim_AccidentBuilder {
    pub(crate) value: Value,
}

impl Claim_AccidentBuilder {
    pub fn build(&self) -> Claim_Accident {
        Claim_Accident {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Claim_Accident) -> Claim_AccidentBuilder {
        Claim_AccidentBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Claim_AccidentBuilder {
        let mut __value: Value = json!({});
        return Claim_AccidentBuilder { value: __value };
    }

    pub fn _date<'a>(&'a mut self, val: Element) -> &'a mut Claim_AccidentBuilder {
        self.value["_date"] = json!(val.value);
        return self;
    }

    pub fn date<'a>(&'a mut self, val: &str) -> &'a mut Claim_AccidentBuilder {
        self.value["date"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Claim_AccidentBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Claim_AccidentBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn location_address<'a>(&'a mut self, val: Address) -> &'a mut Claim_AccidentBuilder {
        self.value["locationAddress"] = json!(val.value);
        return self;
    }

    pub fn location_reference<'a>(&'a mut self, val: Reference) -> &'a mut Claim_AccidentBuilder {
        self.value["locationReference"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Claim_AccidentBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Claim_AccidentBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
