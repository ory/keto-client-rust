/*
 * Ory Oathkeeper API
 *
 * Documentation for all of Ory Oathkeeper's APIs. 
 *
 * The version of the OpenAPI document: v0.8.0-alpha.1
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// PostCheckInternalServerErrorBody : PostCheckInternalServerErrorBody PostCheckInternalServerErrorBody PostCheckInternalServerErrorBody PostCheckInternalServerErrorBody PostCheckInternalServerErrorBody PostCheckInternalServerErrorBody post check internal server error body



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostCheckInternalServerErrorBody {
    /// code
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    /// details
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<serde_json::Value>>,
    /// message
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// reason
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// request
    #[serde(rename = "request", skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
    /// status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl PostCheckInternalServerErrorBody {
    /// PostCheckInternalServerErrorBody PostCheckInternalServerErrorBody PostCheckInternalServerErrorBody PostCheckInternalServerErrorBody PostCheckInternalServerErrorBody PostCheckInternalServerErrorBody post check internal server error body
    pub fn new() -> PostCheckInternalServerErrorBody {
        PostCheckInternalServerErrorBody {
            code: None,
            details: None,
            message: None,
            reason: None,
            request: None,
            status: None,
        }
    }
}


