/*
 * Numaflow
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

/// BasicAuth : BasicAuth represents the basic authentication approach which contains a user name and a password.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BasicAuth {
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<k8s_openapi::api::core::v1::SecretKeySelector>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<k8s_openapi::api::core::v1::SecretKeySelector>,
}

impl BasicAuth {
    /// BasicAuth represents the basic authentication approach which contains a user name and a password.
    pub fn new() -> BasicAuth {
        BasicAuth {
            password: None,
            user: None,
        }
    }
}
