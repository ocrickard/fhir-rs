#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An address expressed using postal conventions (as opposed to GPS or other
/// location definition formats).  This data type may be used to convey addresses
/// for use in delivering mail as well as for visiting locations which might not be
/// valid for mail delivery.  There are a variety of postal address formats defined
/// around the world.

#[derive(Debug)]
pub struct Address<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Address<'_> {
    pub fn new(value: &Value) -> Address {
        Address {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for city
    pub fn _city(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_city") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for country
    pub fn _country(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_country") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for district
    pub fn _district(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_district") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for line
    pub fn _line(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_line") {
            return Some(
                val.into_iter()
                    .map(|e| Element {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for postalCode
    pub fn _postal_code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_postalCode") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for state
    pub fn _state(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_state") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for text
    pub fn _text(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_text") {
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

    /// Extensions for use
    pub fn _use(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_use") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The name of the city, town, suburb, village or other community or delivery
    /// center.
    pub fn city(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("city") {
            return Some(string);
        }
        return None;
    }

    /// Country - a nation as commonly understood or generally accepted.
    pub fn country(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("country") {
            return Some(string);
        }
        return None;
    }

    /// The name of the administrative area (county).
    pub fn district(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("district") {
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

    /// This component contains the house number, apartment number, street name, street
    /// direction,  P.O. Box number, delivery hints, and similar address information.
    pub fn line(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("line") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Time period when address was/is in use.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A postal code designating a region defined by the postal service.
    pub fn postal_code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("postalCode") {
            return Some(string);
        }
        return None;
    }

    /// Sub-unit of a country with limited sovereignty in a federally organized country.
    /// A code may be used if codes are in common use (e.g. US 2 letter state codes).
    pub fn state(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("state") {
            return Some(string);
        }
        return None;
    }

    /// Specifies the entire address as it should be displayed e.g. on a postal label.
    /// This may be provided instead of or as well as the specific parts.
    pub fn text(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("text") {
            return Some(string);
        }
        return None;
    }

    /// Distinguishes between physical addresses (those you can visit) and mailing
    /// addresses (e.g. PO Boxes and care-of addresses). Most addresses are both.
    pub fn fhir_type(&self) -> Option<AddressType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(AddressType::from_string(&val).unwrap());
        }
        return None;
    }

    /// The purpose of this address.
    pub fn fhir_use(&self) -> Option<AddressUse> {
        if let Some(Value::String(val)) = self.value.get("use") {
            return Some(AddressUse::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._city() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._country() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._district() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._line() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._postal_code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._state() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._use() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.city() {}
        if let Some(_val) = self.country() {}
        if let Some(_val) = self.district() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.line() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.postal_code() {}
        if let Some(_val) = self.state() {}
        if let Some(_val) = self.text() {}
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.fhir_use() {}
        return true;
    }
}

#[derive(Debug)]
pub struct AddressBuilder {
    pub(crate) value: Value,
}

impl AddressBuilder {
    pub fn build(&self) -> Address {
        Address {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Address) -> AddressBuilder {
        AddressBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> AddressBuilder {
        let mut __value: Value = json!({});
        return AddressBuilder { value: __value };
    }

    pub fn _city<'a>(&'a mut self, val: Element) -> &'a mut AddressBuilder {
        self.value["_city"] = json!(val.value);
        return self;
    }

    pub fn _country<'a>(&'a mut self, val: Element) -> &'a mut AddressBuilder {
        self.value["_country"] = json!(val.value);
        return self;
    }

    pub fn _district<'a>(&'a mut self, val: Element) -> &'a mut AddressBuilder {
        self.value["_district"] = json!(val.value);
        return self;
    }

    pub fn _line<'a>(&'a mut self, val: Vec<Element>) -> &'a mut AddressBuilder {
        self.value["_line"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _postal_code<'a>(&'a mut self, val: Element) -> &'a mut AddressBuilder {
        self.value["_postalCode"] = json!(val.value);
        return self;
    }

    pub fn _state<'a>(&'a mut self, val: Element) -> &'a mut AddressBuilder {
        self.value["_state"] = json!(val.value);
        return self;
    }

    pub fn _text<'a>(&'a mut self, val: Element) -> &'a mut AddressBuilder {
        self.value["_text"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut AddressBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn _use<'a>(&'a mut self, val: Element) -> &'a mut AddressBuilder {
        self.value["_use"] = json!(val.value);
        return self;
    }

    pub fn city<'a>(&'a mut self, val: &str) -> &'a mut AddressBuilder {
        self.value["city"] = json!(val);
        return self;
    }

    pub fn country<'a>(&'a mut self, val: &str) -> &'a mut AddressBuilder {
        self.value["country"] = json!(val);
        return self;
    }

    pub fn district<'a>(&'a mut self, val: &str) -> &'a mut AddressBuilder {
        self.value["district"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut AddressBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut AddressBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn line<'a>(&'a mut self, val: Vec<&str>) -> &'a mut AddressBuilder {
        self.value["line"] = json!(val);
        return self;
    }

    pub fn period<'a>(&'a mut self, val: Period) -> &'a mut AddressBuilder {
        self.value["period"] = json!(val.value);
        return self;
    }

    pub fn postal_code<'a>(&'a mut self, val: &str) -> &'a mut AddressBuilder {
        self.value["postalCode"] = json!(val);
        return self;
    }

    pub fn state<'a>(&'a mut self, val: &str) -> &'a mut AddressBuilder {
        self.value["state"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: &str) -> &'a mut AddressBuilder {
        self.value["text"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: AddressType) -> &'a mut AddressBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
    }

    pub fn fhir_use<'a>(&'a mut self, val: AddressUse) -> &'a mut AddressBuilder {
        self.value["use"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum AddressType {
    Postal,
    Physical,
    Both,
}

impl AddressType {
    pub fn from_string(string: &str) -> Option<AddressType> {
        match string {
            "postal" => Some(AddressType::Postal),
            "physical" => Some(AddressType::Physical),
            "both" => Some(AddressType::Both),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            AddressType::Postal => "postal".to_string(),
            AddressType::Physical => "physical".to_string(),
            AddressType::Both => "both".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum AddressUse {
    Home,
    Work,
    Temp,
    Old,
    Billing,
}

impl AddressUse {
    pub fn from_string(string: &str) -> Option<AddressUse> {
        match string {
            "home" => Some(AddressUse::Home),
            "work" => Some(AddressUse::Work),
            "temp" => Some(AddressUse::Temp),
            "old" => Some(AddressUse::Old),
            "billing" => Some(AddressUse::Billing),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            AddressUse::Home => "home".to_string(),
            AddressUse::Work => "work".to_string(),
            AddressUse::Temp => "temp".to_string(),
            AddressUse::Old => "old".to_string(),
            AddressUse::Billing => "billing".to_string(),
        }
    }
}
