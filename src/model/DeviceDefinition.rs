#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactPoint::ContactPoint;
use crate::model::DeviceDefinition_Capability::DeviceDefinition_Capability;
use crate::model::DeviceDefinition_DeviceName::DeviceDefinition_DeviceName;
use crate::model::DeviceDefinition_Material::DeviceDefinition_Material;
use crate::model::DeviceDefinition_Property::DeviceDefinition_Property;
use crate::model::DeviceDefinition_Specialization::DeviceDefinition_Specialization;
use crate::model::DeviceDefinition_UdiDeviceIdentifier::DeviceDefinition_UdiDeviceIdentifier;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::ProdCharacteristic::ProdCharacteristic;
use crate::model::ProductShelfLife::ProductShelfLife;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.

#[derive(Debug)]
pub struct DeviceDefinition<'a> {
    pub value: &'a Value,
}

impl DeviceDefinition<'_> {
    /// Contact details for an organization or a particular human that is responsible
    /// for the device.
    pub fn contact(&self) -> Option<Vec<ContactPoint>> {
        if let Some(Value::Array(val)) = self.value.get("contact") {
            return Some(
                val.into_iter()
                    .map(|e| ContactPoint { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Device capabilities.
    pub fn capability(&self) -> Option<Vec<DeviceDefinition_Capability>> {
        if let Some(Value::Array(val)) = self.value.get("capability") {
            return Some(
                val.into_iter()
                    .map(|e| DeviceDefinition_Capability { value: e })
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

    /// A name of the manufacturer.
    pub fn manufacturer_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("manufacturerReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The parent device it can be part of.
    pub fn parent_device(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("parentDevice") {
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

    /// A name of the manufacturer.
    pub fn manufacturer_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("manufacturerString") {
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The available versions of the device, e.g., software versions.
    pub fn version(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("version") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
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

    /// Unique device identifier (UDI) assigned to device label or package.  Note that
    /// the Device may include multiple udiCarriers as it either may include just the
    /// udiCarrier for the jurisdiction it is sold, or for multiple jurisdictions it
    /// could have been sold.
    pub fn udi_device_identifier(&self) -> Option<Vec<DeviceDefinition_UdiDeviceIdentifier>> {
        if let Some(Value::Array(val)) = self.value.get("udiDeviceIdentifier") {
            return Some(
                val.into_iter()
                    .map(|e| DeviceDefinition_UdiDeviceIdentifier { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A substance used to create the material(s) of which the device is made.
    pub fn material(&self) -> Option<Vec<DeviceDefinition_Material>> {
        if let Some(Value::Array(val)) = self.value.get("material") {
            return Some(
                val.into_iter()
                    .map(|e| DeviceDefinition_Material { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Language code for the human-readable text strings produced by the device (all
    /// supported).
    pub fn language_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("languageCode") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
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
                    .map(|e| ResourceList { value: e })
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

    /// Shelf Life and storage information.
    pub fn shelf_life_storage(&self) -> Option<Vec<ProductShelfLife>> {
        if let Some(Value::Array(val)) = self.value.get("shelfLifeStorage") {
            return Some(
                val.into_iter()
                    .map(|e| ProductShelfLife { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for manufacturerString
    pub fn _manufacturer_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_manufacturerString") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// What kind of device or device system this is.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept { value: val });
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

    /// An organization that is responsible for the provision and ongoing maintenance of
    /// the device.
    pub fn owner(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("owner") {
            return Some(Reference { value: val });
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

    /// Unique instance identifiers assigned to a device by the software, manufacturers,
    /// other organizations or owners. For example: handle ID.
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

    /// The model number for the device.
    pub fn model_number(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("modelNumber") {
            return Some(string);
        }
        return None;
    }

    /// The capabilities supported on a  device, the standards to which the device
    /// conforms for a particular purpose, and used for the communication.
    pub fn specialization(&self) -> Option<Vec<DeviceDefinition_Specialization>> {
        if let Some(Value::Array(val)) = self.value.get("specialization") {
            return Some(
                val.into_iter()
                    .map(|e| DeviceDefinition_Specialization { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Safety characteristics of the device.
    pub fn safety(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("safety") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The actual configuration settings of a device as it actually operates, e.g.,
    /// regulation status, time properties.
    pub fn property(&self) -> Option<Vec<DeviceDefinition_Property>> {
        if let Some(Value::Array(val)) = self.value.get("property") {
            return Some(
                val.into_iter()
                    .map(|e| DeviceDefinition_Property { value: e })
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

    /// Access to on-line information about the device.
    pub fn online_information(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("onlineInformation") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for onlineInformation
    pub fn _online_information(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_onlineInformation") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for modelNumber
    pub fn _model_number(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_modelNumber") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for version
    pub fn _version(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_version") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A name given to the device to identify it.
    pub fn device_name(&self) -> Option<Vec<DeviceDefinition_DeviceName>> {
        if let Some(Value::Array(val)) = self.value.get("deviceName") {
            return Some(
                val.into_iter()
                    .map(|e| DeviceDefinition_DeviceName { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The quantity of the device present in the packaging (e.g. the number of devices
    /// present in a pack, or the number of devices in the same package of the medicinal
    /// product).
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// Extensions for url
    pub fn _url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_url") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Dimensions, color etc.
    pub fn physical_characteristics(&self) -> Option<ProdCharacteristic> {
        if let Some(val) = self.value.get("physicalCharacteristics") {
            return Some(ProdCharacteristic { value: val });
        }
        return None;
    }

    /// Descriptive information, usage information or implantation information that is
    /// not captured in an existing element.
    pub fn note(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("note") {
            return Some(
                val.into_iter()
                    .map(|e| Annotation { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.contact() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.capability() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.manufacturer_reference() {
            _val.validate();
        }
        if let Some(_val) = self.parent_device() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.manufacturer_string() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.version() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.udi_device_identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.material() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.language_code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.shelf_life_storage() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._manufacturer_string() {
            _val.validate();
        }
        if let Some(_val) = self.fhir_type() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.owner() {
            _val.validate();
        }
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.model_number() {}
        if let Some(_val) = self.specialization() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.safety() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.property() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.online_information() {}
        if let Some(_val) = self._online_information() {
            _val.validate();
        }
        if let Some(_val) = self._model_number() {
            _val.validate();
        }
        if let Some(_val) = self._version() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.device_name() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.quantity() {
            _val.validate();
        }
        if let Some(_val) = self._url() {
            _val.validate();
        }
        if let Some(_val) = self.physical_characteristics() {
            _val.validate();
        }
        if let Some(_val) = self.note() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
