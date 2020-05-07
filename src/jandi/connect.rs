use ureq::{json};
use serde::{Serialize, Deserialize};

const JANDI_CONNECT_URI: &str = "https://wh.jandi.com/connect-api";

#[derive(Serialize, Deserialize)]
struct Payload {
    color: String,
    body: String,
}

pub struct ErrorRes {
    pub status: u16,
    pub reason: String,
}

pub fn send(team_id: i32, token: &str, message: &str) -> Result<u16, ErrorRes> {
    let url = format!("{}/webhook/{}/{}", JANDI_CONNECT_URI, team_id, token);

    let color = "#AAAAAAA";
    let _payload = Payload {
        body: message.to_string(),
        color: color.to_string(),
    };

    let body = json!(&_payload);
    let res = ureq::post(&url[..])
        .set("Accept", "application/vnd.tosslab.jandi-v2+json")
        .send_json(body);

    let status = res.status();
    return match res.ok() {
        true => Ok(res.status()),
        false => {
            let res_body = res.into_json().expect("??");
            let reason = res_body.get("msg")
                .map(|v| {
                    return match v.as_str() { str => str };
                }).expect("Unknown error.")
                .unwrap_or("EMPTY");

            Err(ErrorRes {
                status,
                reason: reason.to_string(),
            })
        }
    };
}
