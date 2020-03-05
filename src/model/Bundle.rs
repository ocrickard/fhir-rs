#![allow(unused_imports, non_camel_case_types)]

use crate::model::Bundle_Entry::Bundle_Entry;
use crate::model::Bundle_Link::Bundle_Link;
use crate::model::Element::Element;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Signature::Signature;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A container for a collection of resources.

#[derive(Debug)]
pub struct Bundle<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Bundle<'_> {
    pub fn new(value: &Value) -> Bundle {
        Bundle {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// Extensions for timestamp
    pub fn _timestamp(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_timestamp") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for total
    pub fn _total(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_total") {
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

    /// An entry in a bundle resource - will either contain a resource or information
    /// about a resource (transactions and history only).
    pub fn entry(&self) -> Option<Vec<Bundle_Entry>> {
        if let Some(Value::Array(val)) = self.value.get("entry") {
            return Some(
                val.into_iter()
                    .map(|e| Bundle_Entry {
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

    /// A persistent identifier for the bundle that won't change as a bundle is copied
    /// from server to server.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier {
                value: Cow::Borrowed(val),
            });
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

    /// A series of links that provide context to this bundle.
    pub fn link(&self) -> Option<Vec<Bundle_Link>> {
        if let Some(Value::Array(val)) = self.value.get("link") {
            return Some(
                val.into_iter()
                    .map(|e| Bundle_Link {
                        value: Cow::Borrowed(e),
                    })
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
            return Some(Meta {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Digital Signature - base64 encoded. XML-DSig or a JWT.
    pub fn signature(&self) -> Option<Signature> {
        if let Some(val) = self.value.get("signature") {
            return Some(Signature {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The date/time that the bundle was assembled - i.e. when the resources were
    /// placed in the bundle.
    pub fn timestamp(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("timestamp") {
            return Some(string);
        }
        return None;
    }

    /// If a set of search matches, this is the total number of entries of type 'match'
    /// across all pages in the search.  It does not include search.mode = 'include' or
    /// 'outcome' entries and it does not provide a count of the number of entries in
    /// the Bundle.
    pub fn total(&self) -> Option<u64> {
        if let Some(val) = self.value.get("total") {
            return Some(val.as_u64().unwrap());
        }
        return None;
    }

    /// Indicates the purpose of this bundle - how it is intended to be used.
    pub fn fhir_type(&self) -> Option<BundleType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(BundleType::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
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
        if let Some(_val) = self._timestamp() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._total() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.entry() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.link() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.meta() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.signature() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.timestamp() {}
        if let Some(_val) = self.total() {}
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct BundleBuilder {
    pub(crate) value: Value,
}

impl BundleBuilder {
    pub fn build(&self) -> Bundle {
        Bundle {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Bundle) -> BundleBuilder {
        BundleBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> BundleBuilder {
        let mut __value: Value = json!({});
        return BundleBuilder { value: __value };
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut BundleBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut BundleBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _timestamp<'a>(&'a mut self, val: Element) -> &'a mut BundleBuilder {
        self.value["_timestamp"] = json!(val.value);
        return self;
    }

    pub fn _total<'a>(&'a mut self, val: Element) -> &'a mut BundleBuilder {
        self.value["_total"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut BundleBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn entry<'a>(&'a mut self, val: Vec<Bundle_Entry>) -> &'a mut BundleBuilder {
        self.value["entry"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut BundleBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Identifier) -> &'a mut BundleBuilder {
        self.value["identifier"] = json!(val.value);
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut BundleBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut BundleBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn link<'a>(&'a mut self, val: Vec<Bundle_Link>) -> &'a mut BundleBuilder {
        self.value["link"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut BundleBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn signature<'a>(&'a mut self, val: Signature) -> &'a mut BundleBuilder {
        self.value["signature"] = json!(val.value);
        return self;
    }

    pub fn timestamp<'a>(&'a mut self, val: &str) -> &'a mut BundleBuilder {
        self.value["timestamp"] = json!(val);
        return self;
    }

    pub fn total<'a>(&'a mut self, val: u64) -> &'a mut BundleBuilder {
        self.value["total"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: BundleType) -> &'a mut BundleBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum BundleType {
    Document,
    Message,
    Transaction,
    TransactionResponse,
    Batch,
    BatchResponse,
    History,
    Searchset,
    Collection,
}

impl BundleType {
    pub fn from_string(string: &str) -> Option<BundleType> {
        match string {
            "document" => Some(BundleType::Document),
            "message" => Some(BundleType::Message),
            "transaction" => Some(BundleType::Transaction),
            "transaction-response" => Some(BundleType::TransactionResponse),
            "batch" => Some(BundleType::Batch),
            "batch-response" => Some(BundleType::BatchResponse),
            "history" => Some(BundleType::History),
            "searchset" => Some(BundleType::Searchset),
            "collection" => Some(BundleType::Collection),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            BundleType::Document => "document".to_string(),
            BundleType::Message => "message".to_string(),
            BundleType::Transaction => "transaction".to_string(),
            BundleType::TransactionResponse => "transaction-response".to_string(),
            BundleType::Batch => "batch".to_string(),
            BundleType::BatchResponse => "batch-response".to_string(),
            BundleType::History => "history".to_string(),
            BundleType::Searchset => "searchset".to_string(),
            BundleType::Collection => "collection".to_string(),
        }
    }
}
