#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Nucleic acids are defined by three distinct elements: the base, sugar and
/// linkage. Individual substance/moiety IDs will be created for each of these
/// elements. The nucleotide sequence will be always entered in the 5’-3’ direction.

#[derive(Debug)]
pub struct SubstanceNucleicAcid_Linkage<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceNucleicAcid_Linkage<'_> {
    pub fn new(value: &Value) -> SubstanceNucleicAcid_Linkage {
        SubstanceNucleicAcid_Linkage {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for connectivity
    pub fn _connectivity(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_connectivity") {
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

    /// Extensions for residueSite
    pub fn _residue_site(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_residueSite") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The entity that links the sugar residues together should also be captured for
    /// nearly all naturally occurring nucleic acid the linkage is a phosphate group.
    /// For many synthetic oligonucleotides phosphorothioate linkages are often seen.
    /// Linkage connectivity is assumed to be 3’-5’. If the linkage is either 3’-3’ or
    /// 5’-5’ this should be specified.
    pub fn connectivity(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("connectivity") {
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Each linkage will be registered as a fragment and have an ID.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier {
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

    /// Each linkage will be registered as a fragment and have at least one name. A
    /// single name shall be assigned to each linkage.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Residues shall be captured as described in 5.3.6.8.3.
    pub fn residue_site(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("residueSite") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._connectivity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._residue_site() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.connectivity() {}
        if let Some(_val) = self.extension() {
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
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.residue_site() {}
        return true;
    }
}

#[derive(Debug)]
pub struct SubstanceNucleicAcid_LinkageBuilder {
    pub(crate) value: Value,
}

impl SubstanceNucleicAcid_LinkageBuilder {
    pub fn build(&self) -> SubstanceNucleicAcid_Linkage {
        SubstanceNucleicAcid_Linkage {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SubstanceNucleicAcid_Linkage) -> SubstanceNucleicAcid_LinkageBuilder {
        SubstanceNucleicAcid_LinkageBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstanceNucleicAcid_LinkageBuilder {
        let mut __value: Value = json!({});
        return SubstanceNucleicAcid_LinkageBuilder { value: __value };
    }

    pub fn _connectivity<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceNucleicAcid_LinkageBuilder {
        self.value["_connectivity"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut SubstanceNucleicAcid_LinkageBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _residue_site<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceNucleicAcid_LinkageBuilder {
        self.value["_residueSite"] = json!(val.value);
        return self;
    }

    pub fn connectivity<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubstanceNucleicAcid_LinkageBuilder {
        self.value["connectivity"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceNucleicAcid_LinkageBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstanceNucleicAcid_LinkageBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Identifier,
    ) -> &'a mut SubstanceNucleicAcid_LinkageBuilder {
        self.value["identifier"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceNucleicAcid_LinkageBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut SubstanceNucleicAcid_LinkageBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn residue_site<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubstanceNucleicAcid_LinkageBuilder {
        self.value["residueSite"] = json!(val);
        return self;
    }
}
