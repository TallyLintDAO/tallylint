use ic_cdk::query;
use serde::{Serialize, Deserialize};
use candid::{CandidType,Principal};
use crate::BACKUP_DATA;
#[derive(Debug, Clone, CandidType, Serialize, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[derive(Debug, Clone, CandidType, Serialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
    pub streaming_strategy: Option<StreamingStrategy>,
}

#[derive(Debug, Clone, CandidType, Serialize)]
pub enum StreamingStrategy {
    Callback {
        callback: Principal,
        token: Vec<u8>,
    },
}

#[query]
fn http_request(request: HttpRequest) -> HttpResponse {
    if request.url == "/download_backup" {
        let backup_data = BACKUP_DATA.with(|backup| backup.borrow().clone());
        HttpResponse {
            status_code: 200,
            headers: vec![
                ("Content-Type".to_string(), "application/octet-stream".to_string()),
            ],
            body: backup_data,
            streaming_strategy: None,
        }
    } else {
        HttpResponse {
            status_code: 404,
            headers: vec![],
            body: b"Not Found".to_vec(),
            streaming_strategy: None,
        }
    }
}