use reqwest::Client;
use std::time::Duration;
use tokio::{time, task};
#[tokio::main]
async fn main() {
    let client = Client::new(); //reusable clinet 
    let urls = vec![
        "https://example.com",
        "https://httpbin.org/get",
        "https://jsonplaceholder.typicode.com/posts/1",
        "https://httpbin.org/delay/3", //slow
        "https://doesnotexist.typ" //invalid url
    ];

    let mut handles = vec![];
    for url in urls {
        let client = client.clone(); //move a clone of the client into the async block
        let url = url.to_string(); // move a copy of the url into the async block
        let handle = task::spawn(async move {
            let mut retries = 0; // retry counter
            let mut success = false; // success flag
            while retries < 2 && !success {
                retries += 1; //increment retry counter
                //outer timeout for each request
                let res = time::timeout(Duration::from_secs(2), async {
                    // actual request
                    let url_clone = url.clone(); //clone the url for 
                    let resp = client.get(&url_clone).send().await?; // Could fail if the url is invalid
                    let text = resp.text().await?; //could fail if the response is not text
                    let title_start = text.find("<title>").unwrap_or(0) + 7; // find the start of the title tag
                    let title_end = text.find("</title>").unwrap_or(text.len()); // find the end
                    println!("TEXT FROM {}: {}...", &url_clone, &text[title_start..title_end]); // get the first 50 characters of response text
                    Ok::<_,reqwest::Error>((url_clone, text.len())) // return the url and length of text
                }).await;
                //handle outer timeout result
                match res {
                    Ok(Ok((url, len))) => {
                        success = true; // set success flag
                        println!("✅ {} ({} bytes)", url, len); //Success
                    },
                    Ok(Err(e)) => println!("❌ {} failed: {}", url, e), // Error in request
                    Err(_) => println!("⏱️ {} timed out", url), // Timeout
                }
            }
        });
        handles.push(handle); // save asyncronous task handle
    }
    for h in handles {
        h.await.unwrap(); //Join all handles and unwrap any panics
    }
}
