#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The subscription resource is used to define a push-based subscription from a
/// server to another system. Once a subscription is registered with the server, the
/// server checks every resource that is created or updated, and if the resource
/// matches the given criteria, it sends a message on the defined "channel" so that
/// another system can take an appropriate action.

#[derive(Debug)]
pub struct Subscription_Channel<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Subscription_Channel<'_> {
    pub fn new(value: &Value) -> Subscription_Channel {
        Subscription_Channel {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for endpoint
    pub fn _endpoint(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_endpoint") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for header
    pub fn _header(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_header") {
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

    /// Extensions for payload
    pub fn _payload(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_payload") {
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

    /// The url that describes the actual end-point to send messages to.
    pub fn endpoint(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("endpoint") {
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

    /// Additional headers / information to send as part of the notification.
    pub fn header(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("header") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
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

    /// The mime type to send the payload in - either application/fhir+xml, or
    /// application/fhir+json. If the payload is not present, then there is no payload
    /// in the notification, just a notification. The mime type "text/plain" may also be
    /// used for Email and SMS subscriptions.
    pub fn payload(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("payload") {
            return Some(string);
        }
        return None;
    }

    /// The type of channel to send notifications on.
    pub fn fhir_type(&self) -> Option<Subscription_ChannelType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(Subscription_ChannelType::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._endpoint() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._header() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._payload() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.endpoint() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.header() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.payload() {}
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Subscription_ChannelBuilder {
    pub(crate) value: Value,
}

impl Subscription_ChannelBuilder {
    pub fn build(&self) -> Subscription_Channel {
        Subscription_Channel {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Subscription_Channel) -> Subscription_ChannelBuilder {
        Subscription_ChannelBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Subscription_ChannelBuilder {
        let mut __value: Value = json!({});
        return Subscription_ChannelBuilder { value: __value };
    }

    pub fn _endpoint<'a>(&'a mut self, val: Element) -> &'a mut Subscription_ChannelBuilder {
        self.value["_endpoint"] = json!(val.value);
        return self;
    }

    pub fn _header<'a>(&'a mut self, val: Vec<Element>) -> &'a mut Subscription_ChannelBuilder {
        self.value["_header"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _payload<'a>(&'a mut self, val: Element) -> &'a mut Subscription_ChannelBuilder {
        self.value["_payload"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut Subscription_ChannelBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn endpoint<'a>(&'a mut self, val: &str) -> &'a mut Subscription_ChannelBuilder {
        self.value["endpoint"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Subscription_ChannelBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn header<'a>(&'a mut self, val: Vec<&str>) -> &'a mut Subscription_ChannelBuilder {
        self.value["header"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Subscription_ChannelBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Subscription_ChannelBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn payload<'a>(&'a mut self, val: &str) -> &'a mut Subscription_ChannelBuilder {
        self.value["payload"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: Subscription_ChannelType,
    ) -> &'a mut Subscription_ChannelBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum Subscription_ChannelType {
    RestHook,
    Websocket,
    Email,
    Sms,
    Message,
}

impl Subscription_ChannelType {
    pub fn from_string(string: &str) -> Option<Subscription_ChannelType> {
        match string {
            "rest-hook" => Some(Subscription_ChannelType::RestHook),
            "websocket" => Some(Subscription_ChannelType::Websocket),
            "email" => Some(Subscription_ChannelType::Email),
            "sms" => Some(Subscription_ChannelType::Sms),
            "message" => Some(Subscription_ChannelType::Message),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Subscription_ChannelType::RestHook => "rest-hook".to_string(),
            Subscription_ChannelType::Websocket => "websocket".to_string(),
            Subscription_ChannelType::Email => "email".to_string(),
            Subscription_ChannelType::Sms => "sms".to_string(),
            Subscription_ChannelType::Message => "message".to_string(),
        }
    }
}
