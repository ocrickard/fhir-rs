#![allow(unused_imports, non_camel_case_types)]

use crate::model::Duration::Duration;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Range::Range;
use serde_json::value::Value;

/// Specifies an event that may occur multiple times. Timing schedules are used to
/// record when things are planned, expected or requested to occur. The most common
/// usage is in dosage instructions for medications. They are also used when
/// planning care of various kinds, and may be used for reporting the schedule to
/// which past regular activities were carried out.

#[derive(Debug)]
pub struct Timing_Repeat<'a> {
    pub value: &'a Value,
}

impl Timing_Repeat<'_> {
    /// Extensions for timeOfDay
    pub fn _time_of_day(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_timeOfDay") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Extensions for frequency
    pub fn _frequency(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_frequency") {
            return Some(Element { value: val });
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

    /// Extensions for dayOfWeek
    pub fn _day_of_week(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_dayOfWeek") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for duration
    pub fn _duration(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_duration") {
            return Some(Element { value: val });
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

    /// If present, indicates that the duration is a range - so to perform the action
    /// between [duration] and [durationMax] time length.
    pub fn duration_max(&self) -> Option<f64> {
        if let Some(val) = self.value.get("durationMax") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Extensions for frequencyMax
    pub fn _frequency_max(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_frequencyMax") {
            return Some(Element { value: val });
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

    /// The number of times to repeat the action within the specified period. If
    /// frequencyMax is present, this element indicates the lower bound of the allowed
    /// range of the frequency.
    pub fn frequency(&self) -> Option<i64> {
        if let Some(val) = self.value.get("frequency") {
            return Some(val.as_i64().unwrap());
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

    /// The number of minutes from the event. If the event code does not indicate
    /// whether the minutes is before or after the event, then the offset is assumed to
    /// be after the event.
    pub fn offset(&self) -> Option<u64> {
        if let Some(val) = self.value.get("offset") {
            return Some(val.as_u64().unwrap());
        }
        return None;
    }

    /// Either a duration for the length of the timing schedule, a range of possible
    /// length, or outer bounds for start and/or end limits of the timing schedule.
    pub fn bounds_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("boundsRange") {
            return Some(Range { value: val });
        }
        return None;
    }

    /// Extensions for durationMax
    pub fn _duration_max(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_durationMax") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for countMax
    pub fn _count_max(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_countMax") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Either a duration for the length of the timing schedule, a range of possible
    /// length, or outer bounds for start and/or end limits of the timing schedule.
    pub fn bounds_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("boundsPeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Extensions for period
    pub fn _period(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_period") {
            return Some(Element { value: val });
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for count
    pub fn _count(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_count") {
            return Some(Element { value: val });
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

    /// How long this thing happens for when it happens. If durationMax is present, this
    /// element indicates the lower bound of the allowed range of the duration.
    pub fn duration(&self) -> Option<f64> {
        if let Some(val) = self.value.get("duration") {
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

    /// Extensions for when
    pub fn _when(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_when") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Extensions for durationUnit
    pub fn _duration_unit(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_durationUnit") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for periodMax
    pub fn _period_max(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_periodMax") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for offset
    pub fn _offset(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_offset") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for periodUnit
    pub fn _period_unit(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_periodUnit") {
            return Some(Element { value: val });
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

    /// Either a duration for the length of the timing schedule, a range of possible
    /// length, or outer bounds for start and/or end limits of the timing schedule.
    pub fn bounds_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("boundsDuration") {
            return Some(Duration { value: val });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._time_of_day() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.frequency_max() {}
        if let Some(_val) = self._frequency() {
            _val.validate();
        }
        if let Some(_val) = self.period() {}
        if let Some(_val) = self._day_of_week() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._duration() {
            _val.validate();
        }
        if let Some(_val) = self.duration_unit() {}
        if let Some(_val) = self.time_of_day() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.duration_max() {}
        if let Some(_val) = self._frequency_max() {
            _val.validate();
        }
        if let Some(_val) = self.count_max() {}
        if let Some(_val) = self.frequency() {}
        if let Some(_val) = self.count() {}
        if let Some(_val) = self.offset() {}
        if let Some(_val) = self.bounds_range() {
            _val.validate();
        }
        if let Some(_val) = self._duration_max() {
            _val.validate();
        }
        if let Some(_val) = self._count_max() {
            _val.validate();
        }
        if let Some(_val) = self.bounds_period() {
            _val.validate();
        }
        if let Some(_val) = self._period() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._count() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.duration() {}
        if let Some(_val) = self.period_max() {}
        if let Some(_val) = self._when() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._duration_unit() {
            _val.validate();
        }
        if let Some(_val) = self._period_max() {
            _val.validate();
        }
        if let Some(_val) = self._offset() {
            _val.validate();
        }
        if let Some(_val) = self._period_unit() {
            _val.validate();
        }
        if let Some(_val) = self.day_of_week() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.bounds_duration() {
            _val.validate();
        }
        if let Some(_val) = self.period_unit() {}
        return true;
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
}
