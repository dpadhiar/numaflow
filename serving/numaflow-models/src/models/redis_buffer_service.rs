/*
 * Numaflow
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisBufferService {
    #[serde(rename = "external", skip_serializing_if = "Option::is_none")]
    pub external: Option<Box<crate::models::RedisConfig>>,
    #[serde(rename = "native", skip_serializing_if = "Option::is_none")]
    pub native: Option<Box<crate::models::NativeRedis>>,
}

impl RedisBufferService {
    pub fn new() -> RedisBufferService {
        RedisBufferService {
            external: None,
            native: None,
        }
    }
}
