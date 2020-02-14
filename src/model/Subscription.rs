#![allow(unused_imports, non_camel_case_types)]

use crate::model::ContactPoint::ContactPoint;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::ResourceList::ResourceList;
use crate::model::Subscription_Channel::Subscription_Channel;
use serde_json::value::Value;

/// The subscription resource is used to define a push-based subscription from a
/// server to another system. Once a subscription is registered with the server, the
/// server checks every resource that is created or updated, and if the resource
/// matches the given criteria, it sends a message on the defined "channel" so that
/// another system can take an appropriate action.

#[derive(Debug)]
pub struct Subscription<'a> {
    pub value: &'a Value,
}

impl Subscription<'_> {
    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The status of the subscription, which marks the server state for managing the
    /// subscription.
    pub fn status(&self) -> Option<SubscriptionStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(SubscriptionStatus::from_string(&val).unwrap());
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

    /// A record of the last error that occurred when the server processed a
    /// notification.
    pub fn error(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("error") {
            return Some(string);
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

    /// The rules that the server should use to determine when to generate notifications
    /// for this subscription.
    pub fn criteria(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("criteria") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for end
    pub fn _end(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_end") {
            return Some(Element { value: val });
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

    /// Extensions for reason
    pub fn _reason(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_reason") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Details where to send notifications when resources are received that meet the
    /// criteria.
    pub fn channel(&self) -> Subscription_Channel {
        Subscription_Channel {
            value: &self.value["channel"],
        }
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

    /// Extensions for criteria
    pub fn _criteria(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_criteria") {
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

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
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

    /// Contact details for a human to contact about the subscription. The primary use
    /// of this for system administrator troubleshooting.
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

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A description of why this subscription is defined.
    pub fn reason(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("reason") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for error
    pub fn _error(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_error") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The time for the server to turn the subscription off.
    pub fn end(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("end") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.error() {}
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.criteria() {}
        if let Some(_val) = self._end() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._reason() {
            _val.validate();
        }
        let _ = self.channel().validate();
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self._criteria() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.contact() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.reason() {}
        if let Some(_val) = self._error() {
            _val.validate();
        }
        if let Some(_val) = self.end() {}
        return true;
    }
}

#[derive(Debug)]
pub enum SubscriptionStatus {
    Requested,
    Active,
    Error,
    Off,
}

impl SubscriptionStatus {
    pub fn from_string(string: &str) -> Option<SubscriptionStatus> {
        match string {
            "requested" => Some(SubscriptionStatus::Requested),
            "active" => Some(SubscriptionStatus::Active),
            "error" => Some(SubscriptionStatus::Error),
            "off" => Some(SubscriptionStatus::Off),
            _ => None,
        }
    }
}
