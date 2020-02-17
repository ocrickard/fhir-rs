#![allow(unused_imports, non_camel_case_types)]

use crate::model::AuditEvent_Detail::AuditEvent_Detail;
use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A record of an event made for purposes of maintaining a security log. Typical
/// uses include detection of intrusion attempts and monitoring for inappropriate
/// usage.

#[derive(Debug)]
pub struct AuditEvent_Entity<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl AuditEvent_Entity<'_> {
    pub fn new(value: &Value) -> AuditEvent_Entity {
        AuditEvent_Entity {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for query
    pub fn _query(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_query") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Text that describes the entity in more detail.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// Tagged value pairs for conveying additional information about the entity.
    pub fn detail(&self) -> Option<Vec<AuditEvent_Detail>> {
        if let Some(Value::Array(val)) = self.value.get("detail") {
            return Some(
                val.into_iter()
                    .map(|e| AuditEvent_Detail {
                        value: Cow::Borrowed(e),
                    })
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

    /// Identifier for the data life-cycle stage for the entity.
    pub fn lifecycle(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("lifecycle") {
            return Some(Coding {
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

    /// A name of the entity in the audit event.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// The query parameters for a query-type entities.
    pub fn query(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("query") {
            return Some(string);
        }
        return None;
    }

    /// Code representing the role the entity played in the event being audited.
    pub fn role(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("role") {
            return Some(Coding {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Security labels for the identified entity.
    pub fn security_label(&self) -> Option<Vec<Coding>> {
        if let Some(Value::Array(val)) = self.value.get("securityLabel") {
            return Some(
                val.into_iter()
                    .map(|e| Coding {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The type of the object that was involved in this audit event.
    pub fn fhir_type(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("type") {
            return Some(Coding {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifies a specific instance of the entity. The reference should be version
    /// specific.
    pub fn what(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("what") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._query() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.detail() {
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
        if let Some(_val) = self.lifecycle() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.query() {}
        if let Some(_val) = self.role() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.security_label() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.what() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct AuditEvent_EntityBuilder {
    pub(crate) value: Value,
}

impl AuditEvent_EntityBuilder {
    pub fn build(&self) -> AuditEvent_Entity {
        AuditEvent_Entity {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: AuditEvent_Entity) -> AuditEvent_EntityBuilder {
        AuditEvent_EntityBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> AuditEvent_EntityBuilder {
        let mut __value: Value = json!({});
        return AuditEvent_EntityBuilder { value: __value };
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut AuditEvent_EntityBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut AuditEvent_EntityBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _query<'a>(&'a mut self, val: Element) -> &'a mut AuditEvent_EntityBuilder {
        self.value["_query"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut AuditEvent_EntityBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn detail<'a>(
        &'a mut self,
        val: Vec<AuditEvent_Detail>,
    ) -> &'a mut AuditEvent_EntityBuilder {
        self.value["detail"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut AuditEvent_EntityBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut AuditEvent_EntityBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn lifecycle<'a>(&'a mut self, val: Coding) -> &'a mut AuditEvent_EntityBuilder {
        self.value["lifecycle"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut AuditEvent_EntityBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut AuditEvent_EntityBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn query<'a>(&'a mut self, val: &str) -> &'a mut AuditEvent_EntityBuilder {
        self.value["query"] = json!(val);
        return self;
    }

    pub fn role<'a>(&'a mut self, val: Coding) -> &'a mut AuditEvent_EntityBuilder {
        self.value["role"] = json!(val.value);
        return self;
    }

    pub fn security_label<'a>(&'a mut self, val: Vec<Coding>) -> &'a mut AuditEvent_EntityBuilder {
        self.value["securityLabel"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: Coding) -> &'a mut AuditEvent_EntityBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }

    pub fn what<'a>(&'a mut self, val: Reference) -> &'a mut AuditEvent_EntityBuilder {
        self.value["what"] = json!(val.value);
        return self;
    }
}
