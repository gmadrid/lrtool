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
    pub id: String,
    #[serde(rename = "type")]
    typ: String,
    payload: RetrieveCatalogPayload,
}

#[derive(Debug, Deserialize)]
pub struct RetrieveCatalogPayload {
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct RetrieveAssetsResponse {
    pub base: String,
    pub resources: Vec<AssetResponse>,
}

#[derive(Debug, Deserialize)]
pub struct AssetResponse {
    pub id: String,
    pub subtype: String,
    pub links: Links,
}

#[derive(Debug, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_: Link,
}

#[derive(Debug, Deserialize)]
pub struct Link {
    pub href: String,
}