#![allow(unused_imports, non_camel_case_types)]

use crate::model::Duration::Duration;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Range::Range;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Specifies an event that may occur multiple times. Timing schedules are used to
/// record when things are planned, expected or requested to occur. The most common
/// usage is in dosage instructions for medications. They are also used when
/// planning care of various kinds, and may be used for reporting the schedule to
/// which past regular activities were carried out.

#[derive(Debug)]
pub struct Timing_Repeat<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Timing_Repeat<'_> {
    pub fn new(value: &Value) -> Timing_Repeat {
        Timing_Repeat {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for count
    pub fn _count(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_count") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for countMax
    pub fn _count_max(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_countMax") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for dayOfWeek
    pub fn _day_of_week(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_dayOfWeek") {
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

    /// Extensions for duration
    pub fn _duration(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_duration") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for durationMax
    pub fn _duration_max(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_durationMax") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for durationUnit
    pub fn _duration_unit(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_durationUnit") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for frequency
    pub fn _frequency(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_frequency") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for frequencyMax
    pub fn _frequency_max(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_frequencyMax") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for offset
    pub fn _offset(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_offset") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for period
    pub fn _period(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_period") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for periodMax
    pub fn _period_max(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_periodMax") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for periodUnit
    pub fn _period_unit(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_periodUnit") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for timeOfDay
    pub fn _time_of_day(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_timeOfDay") {
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

    /// Extensions for when
    pub fn _when(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_when") {
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

    /// Either a duration for the length of the timing schedule, a range of possible
    /// length, or outer bounds for start and/or end limits of the timing schedule.
    pub fn bounds_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("boundsDuration") {
            return Some(Duration {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Either a duration for the length of the timing schedule, a range of possible
    /// length, or outer bounds for start and/or end limits of the timing schedule.
    pub fn bounds_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("boundsPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Either a duration for the length of the timing schedule, a range of possible
    /// length, or outer bounds for start and/or end limits of the timing schedule.
    pub fn bounds_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("boundsRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A total count of the desired number of repetitions across the duration of the
    /// entire timing specification. If countMax is present, this element indicates the
    /// lower bound of the allowed range of count values.
    pub fn count(&self) -> Option<i64> {
        if let Some(val) = self.value.get("count") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// If present, indicates that the count is a range - so to perform the action
    /// between [count] and [countMax] times.
    pub fn count_max(&self) -> Option<i64> {
        if let Some(val) = self.value.get("countMax") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// If one or more days of week is provided, then the action happens only on the
    /// specified day(s).
    pub fn day_of_week(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("dayOfWeek") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// How long this thing happens for when it happens. If durationMax is present, this
    /// element indicates the lower bound of the allowed range of the duration.
    pub fn duration(&self) -> Option<f64> {
        if let Some(val) = self.value.get("duration") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// If present, indicates that the duration is a range - so to perform the action
    /// between [duration] and [durationMax] time length.
    pub fn duration_max(&self) -> Option<f64> {
        if let Some(val) = self.value.get("durationMax") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The units of time for the duration, in UCUM units.
    pub fn duration_unit(&self) -> Option<Timing_RepeatDurationUnit> {
        if let Some(Value::String(val)) = self.value.get("durationUnit") {
            return Some(Timing_RepeatDurationUnit::from_string(&val).unwrap());
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

    /// The number of times to repeat the action within the specified period. If
    /// frequencyMax is present, this element indicates the lower bound of the allowed
    /// range of the frequency.
    pub fn frequency(&self) -> Option<i64> {
        if let Some(val) = self.value.get("frequency") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// If present, indicates that the frequency is a range - so to repeat between
    /// [frequency] and [frequencyMax] times within the period or period range.
    pub fn frequency_max(&self) -> Option<i64> {
        if let Some(val) = self.value.get("frequencyMax") {
            return Some(val.as_i64().unwrap());
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

    /// The number of minutes from the event. If the event code does not indicate
    /// whether the minutes is before or after the event, then the offset is assumed to
    /// be after the event.
    pub fn offset(&self) -> Option<u64> {
        if let Some(val) = self.value.get("offset") {
            return Some(val.as_u64().unwrap());
        }
        return None;
    }

    /// Indicates the duration of time over which repetitions are to occur; e.g. to
    /// express "3 times per day", 3 would be the frequency and "1 day" would be the
    /// period. If periodMax is present, this element indicates the lower bound of the
    /// allowed range of the period length.
    pub fn period(&self) -> Option<f64> {
        if let Some(val) = self.value.get("period") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// If present, indicates that the period is a range from [period] to [periodMax],
    /// allowing expressing concepts such as "do this once every 3-5 days.
    pub fn period_max(&self) -> Option<f64> {
        if let Some(val) = self.value.get("periodMax") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The units of time for the period in UCUM units.
    pub fn period_unit(&self) -> Option<Timing_RepeatPeriodUnit> {
        if let Some(Value::String(val)) = self.value.get("periodUnit") {
            return Some(Timing_RepeatPeriodUnit::from_string(&val).unwrap());
        }
        return None;
    }

    /// Specified time of day for action to take place.
    pub fn time_of_day(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("timeOfDay") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._count() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._count_max() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._day_of_week() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._duration() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._duration_max() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._duration_unit() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._frequency() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._frequency_max() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._offset() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._period_max() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._period_unit() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._time_of_day() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._when() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.bounds_duration() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.bounds_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.bounds_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.count() {}
        if let Some(_val) = self.count_max() {}
        if let Some(_val) = self.day_of_week() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.duration() {}
        if let Some(_val) = self.duration_max() {}
        if let Some(_val) = self.duration_unit() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.frequency() {}
        if let Some(_val) = self.frequency_max() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.offset() {}
        if let Some(_val) = self.period() {}
        if let Some(_val) = self.period_max() {}
        if let Some(_val) = self.period_unit() {}
        if let Some(_val) = self.time_of_day() {
            _val.into_iter().for_each(|_e| {});
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Timing_RepeatBuilder {
    pub(crate) value: Value,
}

impl Timing_RepeatBuilder {
    pub fn build(&self) -> Timing_Repeat {
        Timing_Repeat {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Timing_Repeat) -> Timing_RepeatBuilder {
        Timing_RepeatBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Timing_RepeatBuilder {
        let mut __value: Value = json!({});
        return Timing_RepeatBuilder { value: __value };
    }

    pub fn _count<'a>(&'a mut self, val: Element) -> &'a mut Timing_RepeatBuilder {
        self.value["_count"] = json!(val.value);
        return self;
    }

    pub fn _count_max<'a>(&'a mut self, val: Element) -> &'a mut Timing_RepeatBuilder {
        self.value["_countMax"] = json!(val.value);
        return self;
    }

    pub fn _day_of_week<'a>(&'a mut self, val: Vec<Element>) -> &'a mut Timing_RepeatBuilder {
        self.value["_dayOfWeek"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _duration<'a>(&'a mut self, val: Element) -> &'a mut Timing_RepeatBuilder {
        self.value["_duration"] = json!(val.value);
        return self;
    }

    pub fn _duration_max<'a>(&'a mut self, val: Element) -> &'a mut Timing_RepeatBuilder {
        self.value["_durationMax"] = json!(val.value);
        return self;
    }

    pub fn _duration_unit<'a>(&'a mut self, val: Element) -> &'a mut Timing_RepeatBuilder {
        self.value["_durationUnit"] = json!(val.value);
        return self;
    }

    pub fn _frequency<'a>(&'a mut self, val: Element) -> &'a mut Timing_RepeatBuilder {
        self.value["_frequency"] = json!(val.value);
        return self;
    }

    pub fn _frequency_max<'a>(&'a mut self, val: Element) -> &'a mut Timing_RepeatBuilder {
        self.value["_frequencyMax"] = json!(val.value);
        return self;
    }

    pub fn _offset<'a>(&'a mut self, val: Element) -> &'a mut Timing_RepeatBuilder {
        self.value["_offset"] = json!(val.value);
        return self;
    }

    pub fn _period<'a>(&'a mut self, val: Element) -> &'a mut Timing_RepeatBuilder {
        self.value["_period"] = json!(val.value);
        return self;
    }

    pub fn _period_max<'a>(&'a mut self, val: Element) -> &'a mut Timing_RepeatBuilder {
        self.value["_periodMax"] = json!(val.value);
        return self;
    }

    pub fn _period_unit<'a>(&'a mut self, val: Element) -> &'a mut Timing_RepeatBuilder {
        self.value["_periodUnit"] = json!(val.value);
        return self;
    }

    pub fn _time_of_day<'a>(&'a mut self, val: Vec<Element>) -> &'a mut Timing_RepeatBuilder {
        self.value["_timeOfDay"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _when<'a>(&'a mut self, val: Vec<Element>) -> &'a mut Timing_RepeatBuilder {
        self.value["_when"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn bounds_duration<'a>(&'a mut self, val: Duration) -> &'a mut Timing_RepeatBuilder {
        self.value["boundsDuration"] = json!(val.value);
        return self;
    }

    pub fn bounds_period<'a>(&'a mut self, val: Period) -> &'a mut Timing_RepeatBuilder {
        self.value["boundsPeriod"] = json!(val.value);
        return self;
    }

    pub fn bounds_range<'a>(&'a mut self, val: Range) -> &'a mut Timing_RepeatBuilder {
        self.value["boundsRange"] = json!(val.value);
        return self;
    }

    pub fn count<'a>(&'a mut self, val: i64) -> &'a mut Timing_RepeatBuilder {
        self.value["count"] = json!(val);
        return self;
    }

    pub fn count_max<'a>(&'a mut self, val: i64) -> &'a mut Timing_RepeatBuilder {
        self.value["countMax"] = json!(val);
        return self;
    }

    pub fn day_of_week<'a>(&'a mut self, val: Vec<&str>) -> &'a mut Timing_RepeatBuilder {
        self.value["dayOfWeek"] = json!(val);
        return self;
    }

    pub fn duration<'a>(&'a mut self, val: f64) -> &'a mut Timing_RepeatBuilder {
        self.value["duration"] = json!(val);
        return self;
    }

    pub fn duration_max<'a>(&'a mut self, val: f64) -> &'a mut Timing_RepeatBuilder {
        self.value["durationMax"] = json!(val);
        return self;
    }

    pub fn duration_unit<'a>(
        &'a mut self,
        val: Timing_RepeatDurationUnit,
    ) -> &'a mut Timing_RepeatBuilder {
        self.value["durationUnit"] = json!(val.to_string());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Timing_RepeatBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn frequency<'a>(&'a mut self, val: i64) -> &'a mut Timing_RepeatBuilder {
        self.value["frequency"] = json!(val);
        return self;
    }

    pub fn frequency_max<'a>(&'a mut self, val: i64) -> &'a mut Timing_RepeatBuilder {
        self.value["frequencyMax"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Timing_RepeatBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Timing_RepeatBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn offset<'a>(&'a mut self, val: u64) -> &'a mut Timing_RepeatBuilder {
        self.value["offset"] = json!(val);
        return self;
    }

    pub fn period<'a>(&'a mut self, val: f64) -> &'a mut Timing_RepeatBuilder {
        self.value["period"] = json!(val);
        return self;
    }

    pub fn period_max<'a>(&'a mut self, val: f64) -> &'a mut Timing_RepeatBuilder {
        self.value["periodMax"] = json!(val);
        return self;
    }

    pub fn period_unit<'a>(
        &'a mut self,
        val: Timing_RepeatPeriodUnit,
    ) -> &'a mut Timing_RepeatBuilder {
        self.value["periodUnit"] = json!(val.to_string());
        return self;
    }

    pub fn time_of_day<'a>(&'a mut self, val: Vec<&str>) -> &'a mut Timing_RepeatBuilder {
        self.value["timeOfDay"] = json!(val);
        return self;
    }
}

#[derive(Debug)]
pub enum Timing_RepeatDurationUnit {
    S,
    Min,
    H,
    D,
    Wk,
    Mo,
    A,
}

impl Timing_RepeatDurationUnit {
    pub fn from_string(string: &str) -> Option<Timing_RepeatDurationUnit> {
        match string {
            "s" => Some(Timing_RepeatDurationUnit::S),
            "min" => Some(Timing_RepeatDurationUnit::Min),
            "h" => Some(Timing_RepeatDurationUnit::H),
            "d" => Some(Timing_RepeatDurationUnit::D),
            "wk" => Some(Timing_RepeatDurationUnit::Wk),
            "mo" => Some(Timing_RepeatDurationUnit::Mo),
            "a" => Some(Timing_RepeatDurationUnit::A),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Timing_RepeatDurationUnit::S => "s".to_string(),
            Timing_RepeatDurationUnit::Min => "min".to_string(),
            Timing_RepeatDurationUnit::H => "h".to_string(),
            Timing_RepeatDurationUnit::D => "d".to_string(),
            Timing_RepeatDurationUnit::Wk => "wk".to_string(),
            Timing_RepeatDurationUnit::Mo => "mo".to_string(),
            Timing_RepeatDurationUnit::A => "a".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Timing_RepeatPeriodUnit {
    S,
    Min,
    H,
    D,
    Wk,
    Mo,
    A,
}

impl Timing_RepeatPeriodUnit {
    pub fn from_string(string: &str) -> Option<Timing_RepeatPeriodUnit> {
        match string {
            "s" => Some(Timing_RepeatPeriodUnit::S),
            "min" => Some(Timing_RepeatPeriodUnit::Min),
            "h" => Some(Timing_RepeatPeriodUnit::H),
            "d" => Some(Timing_RepeatPeriodUnit::D),
            "wk" => Some(Timing_RepeatPeriodUnit::Wk),
            "mo" => Some(Timing_RepeatPeriodUnit::Mo),
            "a" => Some(Timing_RepeatPeriodUnit::A),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Timing_RepeatPeriodUnit::S => "s".to_string(),
            Timing_RepeatPeriodUnit::Min => "min".to_string(),
            Timing_RepeatPeriodUnit::H => "h".to_string(),
            Timing_RepeatPeriodUnit::D => "d".to_string(),
            Timing_RepeatPeriodUnit::Wk => "wk".to_string(),
            Timing_RepeatPeriodUnit::Mo => "mo".to_string(),
            Timing_RepeatPeriodUnit::A => "a".to_string(),
        }
    }
}
