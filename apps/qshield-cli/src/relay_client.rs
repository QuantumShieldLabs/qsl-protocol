use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct RegisterRequest {
    pub id: String,
    pub bundle: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct SendRequest {
    pub to: String,
    pub from: String,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pad_len: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct PollRequest {
    pub id: String,
    pub max: u32,
}

#[derive(Debug, Serialize)]
pub struct ConsumeRequest {
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct EstablishRecordRequest {
    pub peer_id: String,
    pub bundle_id: String,
    pub session_id_hex: String,
    pub dh_init: String,
    pub pq_init_ss: String,
}

#[derive(Debug, Deserialize)]
pub struct GenericOk {
    pub ok: bool,
}

#[derive(Debug, Deserialize)]
pub struct EstablishRecordResponse {
    pub ok: bool,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BundleResponse {
    pub ok: bool,
    pub bundle: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct PollResponse {
    pub ok: bool,
    pub msgs: Option<Vec<RelayMsg>>,
}

#[derive(Debug, Deserialize)]
pub struct RelayMsg {
    pub from: String,
    pub msg: String,
    #[serde(default)]
    pub pad_len: Option<u32>,
    #[serde(default)]
    pub bucket: Option<u32>,
}

pub fn post_json<T: serde::Serialize, R: for<'de> serde::Deserialize<'de>>(
    base: &str,
    path: &str,
    req: &T,
    token: &str,
) -> Result<R, String> {
    let url = format!("{}{}", base.trim_end_matches('/'), path);
    let resp = ureq::post(&url)
        .set("Content-Type", "application/json")
        .set("Authorization", &format!("Bearer {token}"))
        .send_json(req)
        .map_err(|e| format!("relay POST {path} failed: {e}"))?;
    resp.into_json::<R>().map_err(|e| format!("relay POST {path} parse: {e}"))
}

pub fn post_json_allow_status<T: serde::Serialize, R: for<'de> serde::Deserialize<'de>>(
    base: &str,
    path: &str,
    req: &T,
    token: &str,
) -> Result<(u16, R), String> {
    let url = format!("{}{}", base.trim_end_matches('/'), path);
    let resp = ureq::post(&url)
        .set("Content-Type", "application/json")
        .set("Authorization", &format!("Bearer {token}"))
        .send_json(req);
    match resp {
        Ok(resp) => {
            let status = resp.status();
            let parsed = resp
                .into_json::<R>()
                .map_err(|e| format!("relay POST {path} parse: {e}"))?;
            Ok((status, parsed))
        }
        Err(ureq::Error::Status(status, resp)) => {
            let parsed = resp
                .into_json::<R>()
                .map_err(|e| format!("relay POST {path} parse: {e}"))?;
            Ok((status, parsed))
        }
        Err(e) => Err(format!("relay POST {path} failed: {e}")),
    }
}

pub fn get_json<R: for<'de> serde::Deserialize<'de>>(
    base: &str,
    path: &str,
    token: &str,
) -> Result<R, String> {
    let url = format!("{}{}", base.trim_end_matches('/'), path);
    let resp = ureq::get(&url)
        .set("Accept", "application/json")
        .set("Authorization", &format!("Bearer {token}"))
        .call()
        .map_err(|e| format!("relay GET {path} failed: {e}"))?;
    resp.into_json::<R>().map_err(|e| format!("relay GET {path} parse: {e}"))
}
