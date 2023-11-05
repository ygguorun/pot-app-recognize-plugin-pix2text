use base64::{engine::general_purpose, Engine as _};
use reqwest::blocking::multipart::{Form, Part};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[no_mangle]
pub fn recognize(
    base64: &str, // 图像Base64
    lang: &str,   // 识别语言
    // (pot会根据info.json 中的 language 字段传入插件需要的语言代码，无需再次转换)
    needs: HashMap<String, String>, // 插件需要的其他参数,由info.json定义
) -> Result<Value, Box<dyn Error>> {
    let _ = lang;
    let client = reqwest::blocking::ClientBuilder::new().build()?;

    let session_id = match needs.get("session_id") {
        Some(session_id) => session_id.to_string(),
        None => return Err("session_id not found".into()),
    };

    let base64 = general_purpose::STANDARD.decode(base64)?;

    let form_data = Form::new()
        .text("session_id", session_id.to_string())
        .part("image", Part::bytes(base64).file_name("image.png"));

    let res: Value = client
        .post("https://p2t.breezedeus.com/api/pix2text")
        .header("authority", "p2t.breezedeus.com")
        .header("dnt", "1")
        .multipart(form_data)
        .send()?
        .json()?;

    fn parse_result(res: Value) -> Option<Result<Value, Box<dyn Error>>> {
        println!("{res:?}");
        if let Some(error) = res.as_object()?.get("ErrorMessage") {
            return Some(Err(error.to_string().into()));
        }
        let result = res.as_object()?.get("results")?.as_str()?.to_string();
        Some(Ok(Value::String(result)))
    }

    if let Some(result) = parse_result(res) {
        return result;
    } else {
        return Err("Response Parse Error".into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use dotenv::dotenv;
    #[test]
    fn try_request() {
        dotenv().ok();
        let mut needs = HashMap::new();
        needs.insert("session_id".to_string(), env::var("SESSION_ID").unwrap().to_string());
        let result = recognize("iVBORw0KGgoAAAANSUhEUgAAADsAAAAeCAYAAACSRGY2AAAAAXNSR0IArs4c6QAAArNJREFUWEftl19IU1Ecxz+O5uQiNTCJkNj0ZWhkSOyh7CEy0CWZQQoTWYgvk17KFAdr9GBBYGb/qD0oUpgSCZViGkTRQ/hwEVOYIIhlMF8kUjbGZGPFdGtrGvcWzTa79/Gec+79fb7fc36/38nQ6/Xf+E+eDAV2mzqdns6WtDNRqYP5UQ71D8i2RoGVLdW/mqg4K6287G3sqHtEdYEP8clrdpZXYdCCxzWE/dkHjp5poXa/AMEVZodvU+ea2/Dn0n2NnK8wYsgVQAWEAng+TfHiZTddy75NI83LtdBRfSS2xruIONKNNftccs9sFPbLkpqcXUCmei1At2uO3YU6CKnR7AhDLDJ204bdH4u/tKSdjkodmvCrEKz6A2iE9fWEVhAftmF1JwBnmxm0msjPinzHH2A1U42GFcSJZYzGJCaodVhYnRqgZngUCmw8rStC419gzOnA7iuio8HG8b3wccTC2clIkFkWhppPkKcK4H7bTev7cWbDQ5kHcZxqorpQAO8M929dp+eHPgJtNXepNajh6wx9j+9E3BeoONBCc7mOnCx18rJxFDYGYmbwson85Sm67nXSB9SXO7loFPCIDzj2anwtdOPhTpxlueB+h7W3BzF+w6pM9F8wYxACTPc30jAfHTTR22ymeMP78HicEMkqPX8Ku5kAMV6Ba/VOKvQJu4GIkCzx5sYlWuOOxE8CphcsbBQxjBOFXeD5VQftiekr2aUnOc4qsNvV2W12ZuVlYx9irxWrO82zMXLqbFz5WseVqLNlOnKyU7DOhkP/qx2Uysf05BLFJVvQQf1uUxHdmIY9Fq5UxfW5wQCezxK9sbYKx+mTGPMi/fRW9cbSd4rUnyH71pP6KNIRKrDSGqXnDMXZ9PRNOmrF2USNtFotXq+XYDAoLV8Kz5DlrAKbwg7+KrTvuhRWXxXeDuUAAAAASUVORK5CYII=", "eng", needs).unwrap();
        println!("{result}");
    }
}
