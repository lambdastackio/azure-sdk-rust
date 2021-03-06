// Copyright 2017 LambdaStack All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use azure::core::errors::{AzureError, UnexpectedHTTPResult};

use hyper;
use hyper::header::{Headers, ContentLength};
use hyper::status::StatusCode;

use chrono;
use time::Duration;

use std::ops::Add;
use serialize::base64::{STANDARD, ToBase64};

use url::percent_encoding::{utf8_percent_encode, HTTP_VALUE_ENCODE_SET, FORM_URLENCODED_ENCODE_SET};

use crypto::sha2::Sha256;
use crypto::hmac::Hmac;
use crypto::mac::Mac;

use url::Url;
use std::io::Read;

mod client;
pub use self::client::Client;

header! { (Authorization, "Authorization") => [String] }

fn send_event(namespace: &str,
              event_hub: &str,
              policy_name: &str,
              hmac: &mut Hmac<Sha256>,
              event_body: (&mut Read, u64),
              duration: Duration)
              -> Result<(), AzureError> {

    // prepare the url to call
    let url = format!("https://{}.servicebus.windows.net/{}/messages",
                      namespace,
                      event_hub);
    let url = try!(Url::parse(&url));
    debug!("url == {:?}", url);

    // create content

    // generate sas signature based on key name, key value, url and duration.
    let sas = generate_signature(&policy_name, hmac, &url.to_string(), duration);
    debug!("sas == {}", sas);

    // add required headers (in this case just the Authorization and Content-Length).
    let client = hyper::client::Client::new();
    let mut headers = Headers::new();
    headers.set(Authorization(sas));
    headers.set(ContentLength(event_body.1));

    let body = hyper::client::Body::SizedBody(event_body.0, event_body.1);

    // Post the request along with the headers and the body.
    let url_string = url.to_string();
    let mut response = try!(client.post(&url_string).body(body).headers(headers).send());
    info!("response.status == {}", response.status);
    debug!("response.headers == {:?}", response.headers);

    if response.status != StatusCode::Created {
        debug!("response status unexpected, returning Err");
        let mut resp_s = String::new();
        try!(response.read_to_string(&mut resp_s));
        return Err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(
            StatusCode::Created,
            response.status,
            &resp_s)));
    }

    debug!("response status ok, returning Ok");
    Ok(())
}

fn generate_signature(policy_name: &str,
                      hmac: &mut Hmac<Sha256>,
                      url: &str,
                      ttl: Duration)
                      -> String {
    let expiry = chrono::UTC::now().add(ttl).timestamp();
    debug!("expiry == {:?}", expiry);

    let url_encoded = utf8_percent_encode(url, HTTP_VALUE_ENCODE_SET);
    debug!("url_encoded == {:?}", url_encoded);

    let str_to_sign = format!("{}\n{}", url_encoded, expiry);
    debug!("str_to_sign == {:?}", str_to_sign);

    hmac.reset();
    hmac.input(str_to_sign.as_bytes());
    let sig = hmac.result().code().to_base64(STANDARD);
    let sig = utf8_percent_encode(&sig, FORM_URLENCODED_ENCODE_SET);
    debug!("sig == {:?}", sig);

    format!("SharedAccessSignature sr={}&sig={}&se={}&skn={}",
            &url_encoded,
            sig,
            expiry,
            policy_name)
}
