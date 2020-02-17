#![allow(unused_imports, non_camel_case_types)]

use crate::model::BiologicallyDerivedProduct_Collection::BiologicallyDerivedProduct_Collection;
use crate::model::BiologicallyDerivedProduct_Manipulation::BiologicallyDerivedProduct_Manipulation;
use crate::model::BiologicallyDerivedProduct_Processing::BiologicallyDerivedProduct_Processing;
use crate::model::BiologicallyDerivedProduct_Storage::BiologicallyDerivedProduct_Storage;
use crate::model::CodeableConcept::CodeableConcept;
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

/// A material substance originating from a biological entity intended to be
/// transplanted or infused
/// into another (possibly the same) biological entity.

#[derive(Debug)]
pub struct BiologicallyDerivedProduct<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl BiologicallyDerivedProduct<'_> {
    pub fn new(value: &Value) -> BiologicallyDerivedProduct {
        BiologicallyDerivedProduct {
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

    /// Extensions for productCategory
    pub fn _product_category(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_productCategory") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for quantity
    pub fn _quantity(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_quantity") {
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

    /// How this product was collected.
    pub fn collection(&self) -> Option<BiologicallyDerivedProduct_Collection> {
        if let Some(val) = self.value.get("collection") {
            return Some(BiologicallyDerivedProduct_Collection {
                value: Cow::Borrowed(val),
            });
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

    /// This records identifiers associated with this biologically derived product
    /// instance that are defined by business processes and/or used to refer to it when
    /// a direct URL reference to the resource itself is not appropriate (e.g. in CDA
    /// documents, or in written / printed documentation).
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

    /// Any manipulation of product post-collection that is intended to alter the
    /// product.  For example a buffy-coat enrichment or CD8 reduction of Peripheral
    /// Blood Stem Cells to make it more suitable for infusion.
    pub fn manipulation(&self) -> Option<BiologicallyDerivedProduct_Manipulation> {
        if let Some(val) = self.value.get("manipulation") {
            return Some(BiologicallyDerivedProduct_Manipulation {
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

    /// Parent product (if any).
    pub fn parent(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("parent") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Any processing of the product during collection that does not change the
    /// fundamental nature of the product. For example adding anti-coagulants during the
    /// collection of Peripheral Blood Stem Cells.
    pub fn processing(&self) -> Option<Vec<BiologicallyDerivedProduct_Processing>> {
        if let Some(Value::Array(val)) = self.value.get("processing") {
            return Some(
                val.into_iter()
                    .map(|e| BiologicallyDerivedProduct_Processing {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Broad category of this product.
    pub fn product_category(&self) -> Option<BiologicallyDerivedProductProductCategory> {
        if let Some(Value::String(val)) = self.value.get("productCategory") {
            return Some(BiologicallyDerivedProductProductCategory::from_string(&val).unwrap());
        }
        return None;
    }

    /// A code that identifies the kind of this biologically derived product (SNOMED
    /// Ctcode).
    pub fn product_code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("productCode") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Number of discrete units within this product.
    pub fn quantity(&self) -> Option<i64> {
        if let Some(val) = self.value.get("quantity") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// Procedure request to obtain this biologically derived product.
    pub fn request(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("request") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Whether the product is currently available.
    pub fn status(&self) -> Option<BiologicallyDerivedProductStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(BiologicallyDerivedProductStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// Product storage.
    pub fn storage(&self) -> Option<Vec<BiologicallyDerivedProduct_Storage>> {
        if let Some(Value::Array(val)) = self.value.get("storage") {
            return Some(
                val.into_iter()
                    .map(|e| BiologicallyDerivedProduct_Storage {
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
        if let Some(_val) = self._product_category() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.collection() {
            if !_val.validate() {
                return false;
            }
        }
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
        if let Some(_val) = self.manipulation() {
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
        if let Some(_val) = self.parent() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.processing() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.product_category() {}
        if let Some(_val) = self.product_code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.quantity() {}
        if let Some(_val) = self.request() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.storage() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct BiologicallyDerivedProductBuilder {
    pub(crate) value: Value,
}

impl BiologicallyDerivedProductBuilder {
    pub fn build(&self) -> BiologicallyDerivedProduct {
        BiologicallyDerivedProduct {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: BiologicallyDerivedProduct) -> BiologicallyDerivedProductBuilder {
        BiologicallyDerivedProductBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> BiologicallyDerivedProductBuilder {
        let mut __value: Value = json!({});
        return BiologicallyDerivedProductBuilder { value: __value };
    }

    pub fn _implicit_rules<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _product_category<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["_productCategory"] = json!(val.value);
        return self;
    }

    pub fn _quantity<'a>(&'a mut self, val: Element) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["_quantity"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn collection<'a>(
        &'a mut self,
        val: BiologicallyDerivedProduct_Collection,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["collection"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn manipulation<'a>(
        &'a mut self,
        val: BiologicallyDerivedProduct_Manipulation,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["manipulation"] = json!(val.value);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn parent<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["parent"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn processing<'a>(
        &'a mut self,
        val: Vec<BiologicallyDerivedProduct_Processing>,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["processing"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn product_category<'a>(
        &'a mut self,
        val: BiologicallyDerivedProductProductCategory,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["productCategory"] = json!(val.to_string());
        return self;
    }

    pub fn product_code<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["productCode"] = json!(val.value);
        return self;
    }

    pub fn quantity<'a>(&'a mut self, val: i64) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["quantity"] = json!(val);
        return self;
    }

    pub fn request<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["request"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(
        &'a mut self,
        val: BiologicallyDerivedProductStatus,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn storage<'a>(
        &'a mut self,
        val: Vec<BiologicallyDerivedProduct_Storage>,
    ) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["storage"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut BiologicallyDerivedProductBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }
}

#[derive(Debug)]
pub enum BiologicallyDerivedProductProductCategory {
    Organ,
    Tissue,
    Fluid,
    Cells,
    BiologicalAgent,
}

impl BiologicallyDerivedProductProductCategory {
    pub fn from_string(string: &str) -> Option<BiologicallyDerivedProductProductCategory> {
        match string {
            "organ" => Some(BiologicallyDerivedProductProductCategory::Organ),
            "tissue" => Some(BiologicallyDerivedProductProductCategory::Tissue),
            "fluid" => Some(BiologicallyDerivedProductProductCategory::Fluid),
            "cells" => Some(BiologicallyDerivedProductProductCategory::Cells),
            "biologicalAgent" => Some(BiologicallyDerivedProductProductCategory::BiologicalAgent),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            BiologicallyDerivedProductProductCategory::Organ => "organ".to_string(),
            BiologicallyDerivedProductProductCategory::Tissue => "tissue".to_string(),
            BiologicallyDerivedProductProductCategory::Fluid => "fluid".to_string(),
            BiologicallyDerivedProductProductCategory::Cells => "cells".to_string(),
            BiologicallyDerivedProductProductCategory::BiologicalAgent => {
                "biologicalAgent".to_string()
            }
        }
    }
}

#[derive(Debug)]
pub enum BiologicallyDerivedProductStatus {
    Available,
    Unavailable,
}

impl BiologicallyDerivedProductStatus {
    pub fn from_string(string: &str) -> Option<BiologicallyDerivedProductStatus> {
        match string {
            "available" => Some(BiologicallyDerivedProductStatus::Available),
            "unavailable" => Some(BiologicallyDerivedProductStatus::Unavailable),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            BiologicallyDerivedProductStatus::Available => "available".to_string(),
            BiologicallyDerivedProductStatus::Unavailable => "unavailable".to_string(),
        }
    }
}
