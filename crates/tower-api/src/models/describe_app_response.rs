/*
 * Tower API
 *
 * REST API to interact with Tower Services.
 *
 * The version of the OpenAPI document: development
 * Contact: hello@tower.dev
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DescribeAppResponse {
    /// A URL to the JSON Schema for this object.
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub dollar_schema: Option<String>,
    #[serde(rename = "app")]
    pub app: Box<models::App>,
    #[serde(rename = "runs")]
    pub runs: Vec<models::Run>,
}

impl DescribeAppResponse {
    pub fn new(app: models::App, runs: Vec<models::Run>) -> DescribeAppResponse {
        DescribeAppResponse {
            dollar_schema: None,
            app: Box::new(app),
            runs,
        }
    }
}

