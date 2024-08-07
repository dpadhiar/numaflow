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
pub struct GetRedisServiceSpecReq {
    #[serde(rename = "Labels")]
    pub labels: ::std::collections::HashMap<String, String>,
    #[serde(rename = "RedisContainerPort")]
    pub redis_container_port: i32,
    #[serde(rename = "SentinelContainerPort")]
    pub sentinel_container_port: i32,
}

impl GetRedisServiceSpecReq {
    pub fn new(
        labels: ::std::collections::HashMap<String, String>,
        redis_container_port: i32,
        sentinel_container_port: i32,
    ) -> GetRedisServiceSpecReq {
        GetRedisServiceSpecReq {
            labels,
            redis_container_port,
            sentinel_container_port,
        }
    }
}
