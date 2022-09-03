use youtube_channel_feed_scraper::{WEBHOOK_URL, RUN_FREQUENCY, CHANNEL_IDS};
use chrono::{Utc, DateTime, Duration};
use lambda_runtime::{service_fn, LambdaEvent, Error};
use reqwest::Client;
use serde_json::{json, Value};
use xmltojson::{to_json};


// converts returned xml string into json
fn text_to_json(s: &str) -> Value {
    let result = to_json(s);
    let body = match result {
        Ok(_) => result.unwrap(),
        Err(ref _e) => result.expect("Error on GET request"),
    };

    // this pattern will differ depending on content source:
    // news article feeds, youtube channel feeds, etc.
    json!(body["feed"]["entry"])
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(_event: LambdaEvent<Value>) -> Result<Value, Error> {
    // init request client
    let client = Client::new();

    let url_base = "https://www.youtube.com/feeds/videos.xml?channel_id=";
    for channel_id in &CHANNEL_IDS {
        // GET request to youtube channel
        let feed_url = format!("{}{}", url_base, *channel_id);
        //println!("Feed URL: {}", feed_url);
        let get_resp = client.get(feed_url)
        .send()
        .await?
        .text()
        .await?;

        // debug
        //println!("Get resp: {}", get_resp);

        // convert text response to json
        let entries = text_to_json(&get_resp);
        println!("Video json entry: {}", entries);

        // loop over video entries in youtube channel
        for idx in 0..entries.as_array().unwrap().len() {
            // grab current datetime
            let time_now = Utc::now();
            time_now.to_rfc3339();

            // grab datetime of published article
            let mut pub_date_slice: &str = &entries[idx]["published"].to_string();
            pub_date_slice = pub_date_slice.trim_matches('\"');
            
            // ensure time is in proper format
            let result = DateTime::parse_from_rfc3339(pub_date_slice);
            let pub_date = match result {
                Ok(_o) => result.unwrap(),
                Err(ref _e) => result.expect("Error parsing rfc3339 pub_date string"),
            }; 
            
            // Check if video was published since the last time the script was ran
            // I have a cron that runs this script at an interval matching RUN_FREQUENCY
            if time_now.signed_duration_since(pub_date) <= Duration::seconds(RUN_FREQUENCY as i64) {
                // prep body for POST request
                let post_body = json!({
                    "username": "CryptoBot",
                    "content": entries[idx]["link"]["@href"]
                });

                // POST request to webhook
                client.post(WEBHOOK_URL)
                    .header("Content-Type", "application/json")
                    .body(post_body.to_string())
                    .send()
                    .await?;
            }
        }
    }

    Ok(json!({}))

}
