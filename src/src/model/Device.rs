#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Device_DeviceName::Device_DeviceName;
use crate::model::Device_Property::Device_Property;
use crate::model::Device_Specialization::Device_Specialization;
use crate::model::Device_UdiCarrier::Device_UdiCarrier;
use crate::model::Device_Version::Device_Version;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A type of a manufactured item that is used in the provision of healthcare
/// without being substantially changed through that activity. The device may be a
/// medical or non-medical device.

#[derive(Debug)]
pub struct Device<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Device<'_> {
    pub fn new(value: &Value) -> Device {
        Device {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for distinctIdentifier
    pub fn _distinct_identifier(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_distinctIdentifier") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for expirationDate
    pub fn _expiration_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_expirationDate") {
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

    /// Extensions for lotNumber
    pub fn _lot_number(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lotNumber") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for manufactureDate
    pub fn _manufacture_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_manufactureDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for manufacturer
    pub fn _manufacturer(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_manufacturer") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for modelNumber
    pub fn _model_number(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_modelNumber") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for partNumber
    pub fn _part_number(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_partNumber") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for serialNumber
    pub fn _serial_number(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_serialNumber") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for url
    pub fn _url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_url") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Contact details for an organization or a particular human that is responsible
    /// for the device.
    pub fn contact(&self) -> Option<Vec<ContactPoint>> {
        if let Some(Value::Array(val)) = self.value.get("contact") {
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

    /// The reference to the definition for the device.
    pub fn definition(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("definition") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// This represents the manufacturer's name of the device as provided by the device,
    /// from a UDI label, or by a person describing the Device.  This typically would be
    /// used when a person provides the name(s) or when the device represents one of the
    /// names available from DeviceDefinition.
    pub fn device_name(&self) -> Option<Vec<Device_DeviceName>> {
        if let Some(Value::Array(val)) = self.value.get("deviceName") {
            return Some(
                val.into_iter()
                    .map(|e| Device_DeviceName {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The distinct identification string as required by regulation for a human cell,
    /// tissue, or cellular and tissue-based product.
    pub fn distinct_identifier(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("distinctIdentifier") {
            return Some(string);
        }
        return None;
    }

    /// The date and time beyond which this device is no longer valid or should not be
    /// used (if applicable).
    pub fn expiration_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("expirationDate") {
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Unique instance identifiers assigned to a device by manufacturers other
    /// organizations or owners.
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

    /// The place where the device can be found.
    pub fn location(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("location") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Lot number assigned by the manufacturer.
    pub fn lot_number(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("lotNumber") {
            return Some(string);
        }
        return None;
    }

    /// The date and time when the device was manufactured.
    pub fn manufacture_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("manufactureDate") {
            return Some(string);
        }
        return None;
    }

    /// A name of the manufacturer.
    pub fn manufacturer(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("manufacturer") {
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

    /// The model number for the device.
    pub fn model_number(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("modelNumber") {
            return Some(string);
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

    /// Descriptive information, usage information or implantation information that is
    /// not captured in an existing element.
    pub fn note(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("note") {
            return Some(
                val.into_iter()
                    .map(|e| Annotation {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An organization that is responsible for the provision and ongoing maintenance of
    /// the device.
    pub fn owner(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("owner") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The parent device.
    pub fn parent(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("parent") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The part number of the device.
    pub fn part_number(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("partNumber") {
            return Some(string);
        }
        return None;
    }

    /// Patient information, If the device is affixed to a person.
    pub fn patient(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("patient") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The actual configuration settings of a device as it actually operates, e.g.,
    /// regulation status, time properties.
    pub fn property(&self) -> Option<Vec<Device_Property>> {
        if let Some(Value::Array(val)) = self.value.get("property") {
            return Some(
                val.into_iter()
                    .map(|e| Device_Property {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Provides additional safety characteristics about a medical device.  For example
    /// devices containing latex.
    pub fn safety(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("safety") {
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

    /// The serial number assigned by the organization when the device was manufactured.
    pub fn serial_number(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("serialNumber") {
            return Some(string);
        }
        return None;
    }

    /// The capabilities supported on a  device, the standards to which the device
    /// conforms for a particular purpose, and used for the communication.
    pub fn specialization(&self) -> Option<Vec<Device_Specialization>> {
        if let Some(Value::Array(val)) = self.value.get("specialization") {
            return Some(
                val.into_iter()
                    .map(|e| Device_Specialization {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Status of the Device availability.
    pub fn status(&self) -> Option<DeviceStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(DeviceStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// Reason for the dtatus of the Device availability.
    pub fn status_reason(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("statusReason") {
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

    /// The kind or type of device.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Unique device identifier (UDI) assigned to device label or package.  Note that
    /// the Device may include multiple udiCarriers as it either may include just the
    /// udiCarrier for the jurisdiction it is sold, or for multiple jurisdictions it
    /// could have been sold.
    pub fn udi_carrier(&self) -> Option<Vec<Device_UdiCarrier>> {
        if let Some(Value::Array(val)) = self.value.get("udiCarrier") {
            return Some(
                val.into_iter()
                    .map(|e| Device_UdiCarrier {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A network address on which the device may be contacted directly.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    /// The actual design of the device or software version running on the device.
    pub fn version(&self) -> Option<Vec<Device_Version>> {
        if let Some(Value::Array(val)) = self.value.get("version") {
            return Some(
                val.into_iter()
                    .map(|e| Device_Version {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._distinct_identifier() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._expiration_date() {
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
        if let Some(_val) = self._lot_number() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._manufacture_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._manufacturer() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._model_number() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._part_number() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._serial_number() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._url() {
            if !_val.validate() {
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
        if let Some(_val) = self.definition() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.device_name() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.distinct_identifier() {}
        if let Some(_val) = self.expiration_date() {}
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
        if let Some(_val) = self.location() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.lot_number() {}
        if let Some(_val) = self.manufacture_date() {}
        if let Some(_val) = self.manufacturer() {}
        if let Some(_val) = self.meta() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.model_number() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.note() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.owner() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.parent() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.part_number() {}
        if let Some(_val) = self.patient() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.property() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.safety() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.serial_number() {}
        if let Some(_val) = self.specialization() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.status_reason() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.udi_carrier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.version() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct DeviceBuilder {
    pub(crate) value: Value,
}

impl DeviceBuilder {
    pub fn build(&self) -> Device {
        Device {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Device) -> DeviceBuilder {
        DeviceBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> DeviceBuilder {
        let mut __value: Value = json!({});
        return DeviceBuilder { value: __value };
    }

    pub fn _distinct_identifier<'a>(&'a mut self, val: Element) -> &'a mut DeviceBuilder {
        self.value["_distinctIdentifier"] = json!(val.value);
        return self;
    }

    pub fn _expiration_date<'a>(&'a mut self, val: Element) -> &'a mut DeviceBuilder {
        self.value["_expirationDate"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut DeviceBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut DeviceBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _lot_number<'a>(&'a mut self, val: Element) -> &'a mut DeviceBuilder {
        self.value["_lotNumber"] = json!(val.value);
        return self;
    }

    pub fn _manufacture_date<'a>(&'a mut self, val: Element) -> &'a mut DeviceBuilder {
        self.value["_manufactureDate"] = json!(val.value);
        return self;
    }

    pub fn _manufacturer<'a>(&'a mut self, val: Element) -> &'a mut DeviceBuilder {
        self.value["_manufacturer"] = json!(val.value);
        return self;
    }

    pub fn _model_number<'a>(&'a mut self, val: Element) -> &'a mut DeviceBuilder {
        self.value["_modelNumber"] = json!(val.value);
        return self;
    }

    pub fn _part_number<'a>(&'a mut self, val: Element) -> &'a mut DeviceBuilder {
        self.value["_partNumber"] = json!(val.value);
        return self;
    }

    pub fn _serial_number<'a>(&'a mut self, val: Element) -> &'a mut DeviceBuilder {
        self.value["_serialNumber"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut DeviceBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _url<'a>(&'a mut self, val: Element) -> &'a mut DeviceBuilder {
        self.value["_url"] = json!(val.value);
        return self;
    }

    pub fn contact<'a>(&'a mut self, val: Vec<ContactPoint>) -> &'a mut DeviceBuilder {
        self.value["contact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut DeviceBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn definition<'a>(&'a mut self, val: Reference) -> &'a mut DeviceBuilder {
        self.value["definition"] = json!(val.value);
        return self;
    }

    pub fn device_name<'a>(&'a mut self, val: Vec<Device_DeviceName>) -> &'a mut DeviceBuilder {
        self.value["deviceName"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn distinct_identifier<'a>(&'a mut self, val: &str) -> &'a mut DeviceBuilder {
        self.value["distinctIdentifier"] = json!(val);
        return self;
    }

    pub fn expiration_date<'a>(&'a mut self, val: &str) -> &'a mut DeviceBuilder {
        self.value["expirationDate"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut DeviceBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut DeviceBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut DeviceBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut DeviceBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut DeviceBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn location<'a>(&'a mut self, val: Reference) -> &'a mut DeviceBuilder {
        self.value["location"] = json!(val.value);
        return self;
    }

    pub fn lot_number<'a>(&'a mut self, val: &str) -> &'a mut DeviceBuilder {
        self.value["lotNumber"] = json!(val);
        return self;
    }

    pub fn manufacture_date<'a>(&'a mut self, val: &str) -> &'a mut DeviceBuilder {
        self.value["manufactureDate"] = json!(val);
        return self;
    }

    pub fn manufacturer<'a>(&'a mut self, val: &str) -> &'a mut DeviceBuilder {
        self.value["manufacturer"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut DeviceBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn model_number<'a>(&'a mut self, val: &str) -> &'a mut DeviceBuilder {
        self.value["modelNumber"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut DeviceBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut DeviceBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn owner<'a>(&'a mut self, val: Reference) -> &'a mut DeviceBuilder {
        self.value["owner"] = json!(val.value);
        return self;
    }

    pub fn parent<'a>(&'a mut self, val: Reference) -> &'a mut DeviceBuilder {
        self.value["parent"] = json!(val.value);
        return self;
    }

    pub fn part_number<'a>(&'a mut self, val: &str) -> &'a mut DeviceBuilder {
        self.value["partNumber"] = json!(val);
        return self;
    }

    pub fn patient<'a>(&'a mut self, val: Reference) -> &'a mut DeviceBuilder {
        self.value["patient"] = json!(val.value);
        return self;
    }

    pub fn property<'a>(&'a mut self, val: Vec<Device_Property>) -> &'a mut DeviceBuilder {
        self.value["property"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn safety<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut DeviceBuilder {
        self.value["safety"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn serial_number<'a>(&'a mut self, val: &str) -> &'a mut DeviceBuilder {
        self.value["serialNumber"] = json!(val);
        return self;
    }

    pub fn specialization<'a>(
        &'a mut self,
        val: Vec<Device_Specialization>,
    ) -> &'a mut DeviceBuilder {
        self.value["specialization"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(&'a mut self, val: DeviceStatus) -> &'a mut DeviceBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn status_reason<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut DeviceBuilder {
        self.value["statusReason"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut DeviceBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut DeviceBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }

    pub fn udi_carrier<'a>(&'a mut self, val: Vec<Device_UdiCarrier>) -> &'a mut DeviceBuilder {
        self.value["udiCarrier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn url<'a>(&'a mut self, val: &str) -> &'a mut DeviceBuilder {
        self.value["url"] = json!(val);
        return self;
    }

    pub fn version<'a>(&'a mut self, val: Vec<Device_Version>) -> &'a mut DeviceBuilder {
        self.value["version"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

#[derive(Debug)]
pub enum DeviceStatus {
    Active,
    Inactive,
    EnteredInError,
    Unknown,
}

impl DeviceStatus {
    pub fn from_string(string: &str) -> Option<DeviceStatus> {
        match string {
            "active" => Some(DeviceStatus::Active),
            "inactive" => Some(DeviceStatus::Inactive),
            "entered-in-error" => Some(DeviceStatus::EnteredInError),
            "unknown" => Some(DeviceStatus::Unknown),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            DeviceStatus::Active => "active".to_string(),
            DeviceStatus::Inactive => "inactive".to_string(),
            DeviceStatus::EnteredInError => "entered-in-error".to_string(),
            DeviceStatus::Unknown => "unknown".to_string(),
        }
    }
}
