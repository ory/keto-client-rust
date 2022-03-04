/*
 * Ory Keto API
 *
 * Documentation for all of Ory Keto's REST APIs. gRPC is documented separately. 
 *
 * The version of the OpenAPI document: v0.8.0-alpha.2
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineResponse2001 {
    /// The version of Ory Keto.
    #[serde(rename = "version")]
    pub version: String,
}

impl InlineResponse2001 {
    pub fn new(version: String) -> InlineResponse2001 {
        InlineResponse2001 {
            version,
        }
    }
}


