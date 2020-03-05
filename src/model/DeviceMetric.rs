#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::DeviceMetric_Calibration::DeviceMetric_Calibration;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::Timing::Timing;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Describes a measurement, calculation or setting capability of a medical device.

#[derive(Debug)]
pub struct DeviceMetric<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl DeviceMetric<'_> {
    pub fn new(value: &Value) -> DeviceMetric {
        DeviceMetric {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for category
    pub fn _category(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_category") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for color
    pub fn _color(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_color") {
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

    /// Extensions for operationalStatus
    pub fn _operational_status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_operationalStatus") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Describes the calibrations that have been performed or that are required to be
    /// performed.
    pub fn calibration(&self) -> Option<Vec<DeviceMetric_Calibration>> {
        if let Some(Value::Array(val)) = self.value.get("calibration") {
            return Some(
                val.into_iter()
                    .map(|e| DeviceMetric_Calibration {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates the category of the observation generation process. A DeviceMetric can
    /// be for example a setting, measurement, or calculation.
    pub fn category(&self) -> Option<DeviceMetricCategory> {
        if let Some(Value::String(val)) = self.value.get("category") {
            return Some(DeviceMetricCategory::from_string(&val).unwrap());
        }
        return None;
    }

    /// Describes the color representation for the metric. This is often used to aid
    /// clinicians to track and identify parameter types by color. In practice, consider
    /// a Patient Monitor that has ECG/HR and Pleth for example; the parameters are
    /// displayed in different characteristic colors, such as HR-blue, BP-green, and PR
    /// and SpO2- magenta.
    pub fn color(&self) -> Option<DeviceMetricColor> {
        if let Some(Value::String(val)) = self.value.get("color") {
            return Some(DeviceMetricColor::from_string(&val).unwrap());
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Unique instance identifiers assigned to a device by the device or gateway
    /// software, manufacturers, other organizations or owners. For example: handle ID.
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

    /// Describes the measurement repetition time. This is not necessarily the same as
    /// the update period. The measurement repetition time can range from milliseconds
    /// up to hours. An example for a measurement repetition time in the range of
    /// milliseconds is the sampling rate of an ECG. An example for a measurement
    /// repetition time in the range of hours is a NIBP that is triggered automatically
    /// every hour. The update period may be different than the measurement repetition
    /// time, if the device does not update the published observed value with the same
    /// frequency as it was measured.
    pub fn measurement_period(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("measurementPeriod") {
            return Some(Timing {
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

    /// Indicates current operational state of the device. For example: On, Off,
    /// Standby, etc.
    pub fn operational_status(&self) -> Option<DeviceMetricOperationalStatus> {
        if let Some(Value::String(val)) = self.value.get("operationalStatus") {
            return Some(DeviceMetricOperationalStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// Describes the link to the  Device that this DeviceMetric belongs to and that
    /// provide information about the location of this DeviceMetric in the containment
    /// structure of the parent Device. An example would be a Device that represents a
    /// Channel. This reference can be used by a client application to distinguish
    /// DeviceMetrics that have the same type, but should be interpreted based on their
    /// containment location.
    pub fn parent(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("parent") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Describes the link to the  Device that this DeviceMetric belongs to and that
    /// contains administrative device information such as manufacturer, serial number,
    /// etc.
    pub fn source(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("source") {
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

    /// Describes the type of the metric. For example: Heart Rate, PEEP Setting, etc.
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["type"]),
        }
    }

    /// Describes the unit that an observed value determined for this metric will have.
    /// For example: Percent, Seconds, etc.
    pub fn unit(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("unit") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._category() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._color() {
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
        if let Some(_val) = self._operational_status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.calibration() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.category() {}
        if let Some(_val) = self.color() {}
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
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.measurement_period() {
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
        if let Some(_val) = self.operational_status() {}
        if let Some(_val) = self.parent() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.source() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.fhir_type().validate() {
            return false;
        }
        if let Some(_val) = self.unit() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct DeviceMetricBuilder {
    pub(crate) value: Value,
}

impl DeviceMetricBuilder {
    pub fn build(&self) -> DeviceMetric {
        DeviceMetric {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: DeviceMetric) -> DeviceMetricBuilder {
        DeviceMetricBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(fhir_type: CodeableConcept) -> DeviceMetricBuilder {
        let mut __value: Value = json!({});
        __value["type"] = json!(fhir_type.value);
        return DeviceMetricBuilder { value: __value };
    }

    pub fn _category<'a>(&'a mut self, val: Element) -> &'a mut DeviceMetricBuilder {
        self.value["_category"] = json!(val.value);
        return self;
    }

    pub fn _color<'a>(&'a mut self, val: Element) -> &'a mut DeviceMetricBuilder {
        self.value["_color"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut DeviceMetricBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut DeviceMetricBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _operational_status<'a>(&'a mut self, val: Element) -> &'a mut DeviceMetricBuilder {
        self.value["_operationalStatus"] = json!(val.value);
        return self;
    }

    pub fn calibration<'a>(
        &'a mut self,
        val: Vec<DeviceMetric_Calibration>,
    ) -> &'a mut DeviceMetricBuilder {
        self.value["calibration"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn category<'a>(&'a mut self, val: DeviceMetricCategory) -> &'a mut DeviceMetricBuilder {
        self.value["category"] = json!(val.to_string());
        return self;
    }

    pub fn color<'a>(&'a mut self, val: DeviceMetricColor) -> &'a mut DeviceMetricBuilder {
        self.value["color"] = json!(val.to_string());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut DeviceMetricBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut DeviceMetricBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut DeviceMetricBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut DeviceMetricBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut DeviceMetricBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut DeviceMetricBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn measurement_period<'a>(&'a mut self, val: Timing) -> &'a mut DeviceMetricBuilder {
        self.value["measurementPeriod"] = json!(val.value);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut DeviceMetricBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DeviceMetricBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn operational_status<'a>(
        &'a mut self,
        val: DeviceMetricOperationalStatus,
    ) -> &'a mut DeviceMetricBuilder {
        self.value["operationalStatus"] = json!(val.to_string());
        return self;
    }

    pub fn parent<'a>(&'a mut self, val: Reference) -> &'a mut DeviceMetricBuilder {
        self.value["parent"] = json!(val.value);
        return self;
    }

    pub fn source<'a>(&'a mut self, val: Reference) -> &'a mut DeviceMetricBuilder {
        self.value["source"] = json!(val.value);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut DeviceMetricBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn unit<'a>(&'a mut self, val: CodeableConcept) -> &'a mut DeviceMetricBuilder {
        self.value["unit"] = json!(val.value);
        return self;
    }
}

#[derive(Debug)]
pub enum DeviceMetricCategory {
    Measurement,
    Setting,
    Calculation,
    Unspecified,
}

impl DeviceMetricCategory {
    pub fn from_string(string: &str) -> Option<DeviceMetricCategory> {
        match string {
            "measurement" => Some(DeviceMetricCategory::Measurement),
            "setting" => Some(DeviceMetricCategory::Setting),
            "calculation" => Some(DeviceMetricCategory::Calculation),
            "unspecified" => Some(DeviceMetricCategory::Unspecified),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            DeviceMetricCategory::Measurement => "measurement".to_string(),
            DeviceMetricCategory::Setting => "setting".to_string(),
            DeviceMetricCategory::Calculation => "calculation".to_string(),
            DeviceMetricCategory::Unspecified => "unspecified".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum DeviceMetricColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl DeviceMetricColor {
    pub fn from_string(string: &str) -> Option<DeviceMetricColor> {
        match string {
            "black" => Some(DeviceMetricColor::Black),
            "red" => Some(DeviceMetricColor::Red),
            "green" => Some(DeviceMetricColor::Green),
            "yellow" => Some(DeviceMetricColor::Yellow),
            "blue" => Some(DeviceMetricColor::Blue),
            "magenta" => Some(DeviceMetricColor::Magenta),
            "cyan" => Some(DeviceMetricColor::Cyan),
            "white" => Some(DeviceMetricColor::White),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            DeviceMetricColor::Black => "black".to_string(),
            DeviceMetricColor::Red => "red".to_string(),
            DeviceMetricColor::Green => "green".to_string(),
            DeviceMetricColor::Yellow => "yellow".to_string(),
            DeviceMetricColor::Blue => "blue".to_string(),
            DeviceMetricColor::Magenta => "magenta".to_string(),
            DeviceMetricColor::Cyan => "cyan".to_string(),
            DeviceMetricColor::White => "white".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum DeviceMetricOperationalStatus {
    On,
    Off,
    Standby,
    EnteredInError,
}

impl DeviceMetricOperationalStatus {
    pub fn from_string(string: &str) -> Option<DeviceMetricOperationalStatus> {
        match string {
            "on" => Some(DeviceMetricOperationalStatus::On),
            "off" => Some(DeviceMetricOperationalStatus::Off),
            "standby" => Some(DeviceMetricOperationalStatus::Standby),
            "entered-in-error" => Some(DeviceMetricOperationalStatus::EnteredInError),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            DeviceMetricOperationalStatus::On => "on".to_string(),
            DeviceMetricOperationalStatus::Off => "off".to_string(),
            DeviceMetricOperationalStatus::Standby => "standby".to_string(),
            DeviceMetricOperationalStatus::EnteredInError => "entered-in-error".to_string(),
        }
    }
}
