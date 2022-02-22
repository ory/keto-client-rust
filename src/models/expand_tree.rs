/*
 * Ory Oathkeeper API
 *
 * Documentation for all of Ory Oathkeeper's APIs. 
 *
 * The version of the OpenAPI document: v0.8.0-alpha.1
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// ExpandTree : ExpandTree ExpandTree ExpandTree expand tree



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpandTree {
    /// children
    #[serde(rename = "children", skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<crate::models::ExpandTree>>,
    /// subject id
    #[serde(rename = "subject_id", skip_serializing_if = "Option::is_none")]
    pub subject_id: Option<String>,
    #[serde(rename = "subject_set", skip_serializing_if = "Option::is_none")]
    pub subject_set: Option<Box<crate::models::SubjectSet>>,
    /// type
    #[serde(rename = "type")]
    pub _type: Type,
}

impl ExpandTree {
    /// ExpandTree ExpandTree ExpandTree expand tree
    pub fn new(_type: Type) -> ExpandTree {
        ExpandTree {
            children: None,
            subject_id: None,
            subject_set: None,
            _type,
        }
    }
}

/// type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "[[[union exclusion intersection leaf]]]")]
    UnionExclusionIntersectionLeaf,
}

