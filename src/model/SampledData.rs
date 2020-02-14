#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use serde_json::value::Value;

/// A series of measurements taken by a device, with upper and lower limits. There
/// may be more than one dimension in the data.

#[derive(Debug)]
pub struct SampledData<'a> {
    pub value: &'a Value,
}

impl SampledData<'_> {
    /// Extensions for factor
    pub fn _factor(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_factor") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The lower limit of detection of the measured points. This is needed if any of
    /// the data points have the value "L" (lower than detection limit).
    pub fn lower_limit(&self) -> Option<f64> {
        if let Some(val) = self.value.get("lowerLimit") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Extensions for data
    pub fn _data(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_data") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for upperLimit
    pub fn _upper_limit(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_upperLimit") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for dimensions
    pub fn _dimensions(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_dimensions") {
            return Some(Element { value: val });
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A series of data points which are decimal values separated by a single space
    /// (character u20). The special values "E" (error), "L" (below detection limit) and
    /// "U" (above detection limit) can also be used in place of a decimal value.
    pub fn data(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("data") {
            return Some(string);
        }
        return None;
    }

    /// A correction factor that is applied to the sampled data points before they are
    /// added to the origin.
    pub fn factor(&self) -> Option<f64> {
        if let Some(val) = self.value.get("factor") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The length of time between sampling times, measured in milliseconds.
    pub fn period(&self) -> Option<f64> {
        if let Some(val) = self.value.get("period") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The number of sample points at each time point. If this value is greater than
    /// one, then the dimensions will be interlaced - all the sample points for a point
    /// in time will be recorded at once.
    pub fn dimensions(&self) -> Option<i64> {
        if let Some(val) = self.value.get("dimensions") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// Extensions for lowerLimit
    pub fn _lower_limit(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lowerLimit") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The base quantity that a measured value of zero represents. In addition, this
    /// provides the units of the entire measurement series.
    pub fn origin(&self) -> Quantity {
        Quantity {
            value: &self.value["origin"],
        }
    }

    /// Extensions for period
    pub fn _period(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_period") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The upper limit of detection of the measured points. This is needed if any of
    /// the data points have the value "U" (higher than detection limit).
    pub fn upper_limit(&self) -> Option<f64> {
        if let Some(val) = self.value.get("upperLimit") {
            return Some(val.as_f64().unwrap());
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._factor() {
            _val.validate();
        }
        if let Some(_val) = self.lower_limit() {}
        if let Some(_val) = self._data() {
            _val.validate();
        }
        if let Some(_val) = self._upper_limit() {
            _val.validate();
        }
        if let Some(_val) = self._dimensions() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.data() {}
        if let Some(_val) = self.factor() {}
        if let Some(_val) = self.period() {}
        if let Some(_val) = self.dimensions() {}
        if let Some(_val) = self._lower_limit() {
            _val.validate();
        }
        let _ = self.origin().validate();
        if let Some(_val) = self._period() {
            _val.validate();
        }
        if let Some(_val) = self.upper_limit() {}
        if let Some(_val) = self.id() {}
        return true;
    }
}
