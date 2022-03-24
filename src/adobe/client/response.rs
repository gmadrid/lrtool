use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EntitlementResponse {
    base: String,
    id: String,
    #[serde(rename = "type")]
    typ: String,
    email: String,
    entitlement: EntitlementDetail,
}

#[derive(Debug, Deserialize)]
pub struct EntitlementDetail {
    status: String,
}

#[derive(Debug, Deserialize)]
pub struct RetrieveCatalogResponse {
    base: String,
    id: String,
    #[serde(rename = "type")]
    typ: String,
    payload: RetrieveCatalogPayload,
}

#[derive(Debug, Deserialize)]
pub struct RetrieveCatalogPayload {
    name: String,
}
