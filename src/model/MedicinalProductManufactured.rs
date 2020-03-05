#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::ProdCharacteristic::ProdCharacteristic;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The manufactured item as contained in the packaged medicinal product.

#[derive(Debug)]
pub struct MedicinalProductManufactured<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicinalProductManufactured<'_> {
    pub fn new(value: &Value) -> MedicinalProductManufactured {
        MedicinalProductManufactured {
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

    /// Ingredient.
    pub fn ingredient(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("ingredient") {
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// Dose form as manufactured and before any transformation into the pharmaceutical
    /// product.
    pub fn manufactured_dose_form(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["manufacturedDoseForm"]),
        }
    }

    /// Manufacturer of the item (Note that this should be named "manufacturer" but it
    /// currently causes technical issues).
    pub fn manufacturer(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("manufacturer") {
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

    /// Other codeable characteristics.
    pub fn other_characteristics(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("otherCharacteristics") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Dimensions, color etc.
    pub fn physical_characteristics(&self) -> Option<ProdCharacteristic> {
        if let Some(val) = self.value.get("physicalCharacteristics") {
            return Some(ProdCharacteristic {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The quantity or "count number" of the manufactured item.
    pub fn quantity(&self) -> Quantity {
        Quantity {
            value: Cow::Borrowed(&self.value["quantity"]),
        }
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

    /// The “real world” units in which the quantity of the manufactured item is
    /// described.
    pub fn unit_of_presentation(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("unitOfPresentation") {
            return Some(CodeableConcept {
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
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.ingredient() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.language() {}
        if !self.manufactured_dose_form().validate() {
            return false;
        }
        if let Some(_val) = self.manufacturer() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
        if let Some(_val) = self.other_characteristics() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.physical_characteristics() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.quantity().validate() {
            return false;
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.unit_of_presentation() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicinalProductManufacturedBuilder {
    pub(crate) value: Value,
}

impl MedicinalProductManufacturedBuilder {
    pub fn build(&self) -> MedicinalProductManufactured {
        MedicinalProductManufactured {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: MedicinalProductManufactured) -> MedicinalProductManufacturedBuilder {
        MedicinalProductManufacturedBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        manufactured_dose_form: CodeableConcept,
        quantity: Quantity,
    ) -> MedicinalProductManufacturedBuilder {
        let mut __value: Value = json!({});
        __value["manufacturedDoseForm"] = json!(manufactured_dose_form.value);
        __value["quantity"] = json!(quantity.value);
        return MedicinalProductManufacturedBuilder { value: __value };
    }

    pub fn _implicit_rules<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicinalProductManufacturedBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicinalProductManufacturedBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut MedicinalProductManufacturedBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductManufacturedBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicinalProductManufacturedBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn implicit_rules<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicinalProductManufacturedBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn ingredient<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut MedicinalProductManufacturedBuilder {
        self.value["ingredient"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut MedicinalProductManufacturedBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn manufacturer<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut MedicinalProductManufacturedBuilder {
        self.value["manufacturer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut MedicinalProductManufacturedBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductManufacturedBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn other_characteristics<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut MedicinalProductManufacturedBuilder {
        self.value["otherCharacteristics"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn physical_characteristics<'a>(
        &'a mut self,
        val: ProdCharacteristic,
    ) -> &'a mut MedicinalProductManufacturedBuilder {
        self.value["physicalCharacteristics"] = json!(val.value);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut MedicinalProductManufacturedBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn unit_of_presentation<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProductManufacturedBuilder {
        self.value["unitOfPresentation"] = json!(val.value);
        return self;
    }
}
