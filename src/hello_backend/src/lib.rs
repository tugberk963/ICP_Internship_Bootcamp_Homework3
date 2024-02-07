use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext, TransformFunc,
};
use ic_cdk_macros::{query, update};

#[update]
async fn get_weather_data(city: String) -> String {
    let host = "https://api.weatherapi.com/v1";
    let api_key = "key=fd015b7cfda74ccb8c3194513240702";

    let url = format!("{}/current.json?{}&q={}&aqi=no", host, api_key, city);

    let request_headers = vec![
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "exchange_rate_canister".to_string(),
        },
    ];

    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        method: HttpMethod::GET,
        body: None,              
        max_response_bytes: None,
        transform: Some(TransformContext {
            function: TransformFunc(candid::Func {
                principal: ic_cdk::api::id(),
                method: "transform".to_string(),
            }),
            context: vec![],
        }),
        headers: request_headers,
    };

    let cycles = 230_949_972_000;
    match http_request(request, cycles).await {
        Ok((response,)) => {
            String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.")
        }
        Err((r, m)) => {
            let message = format!(
                "The http_request resulted into error. RejectionCode: {:?}, Error: {}",
                r, m
            );

            message
        }
    }
}

#[update]
async fn get_current_temp(city: String) -> String {
    let host = "https://api.weatherapi.com/v1";
    let api_key = "key=fd015b7cfda74ccb8c3194513240702";
    let url = format!("{}/current.json?{}&q={}&aqi=no", host, api_key, city);

    let request_headers = vec![
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "exchange_rate_canister".to_string(),
        },
    ];

    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        method: HttpMethod::GET,
        body: None,          
        max_response_bytes: None,
        transform: Some(TransformContext {
            function: TransformFunc(candid::Func {
                principal: ic_cdk::api::id(),
                method: "transform".to_string(),
            }),
            context: vec![],
        }),
        headers: request_headers,
    };

    let cycles = 230_949_972_000;
    match http_request(request, cycles).await {
        Ok((response,)) => {
            let json_response = String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.");
            let parsed_response: serde_json::Value = serde_json::from_str(&json_response).expect("Failed to parse JSON response");
            let temp_c = parsed_response["current"]["temp_c"].as_f64().unwrap();
            let temp_f = parsed_response["current"]["temp_f"].as_f64().unwrap();
            format!("Current temperature in {}: {:.1}°C / {:.1}°F.",city, temp_c, temp_f)
        }
        Err((r, m)) => {
            let message = format!(
                "The http_request resulted into error. RejectionCode: {:?}, Error: {}",
                r, m
            );

            message
        }
    }
}

#[update]
async fn get_current_condition(city: String) -> String {

    let host = "https://api.weatherapi.com/v1";
    let api_key = "key=fd015b7cfda74ccb8c3194513240702";
    let url = format!("{}/current.json?{}&q={}&aqi=no", host, api_key, city);

    let request_headers = vec![
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "exchange_rate_canister".to_string(),
        },
    ];

    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        method: HttpMethod::GET,
        body: None,              
        max_response_bytes: None, 
        transform: Some(TransformContext {
            function: TransformFunc(candid::Func {
                principal: ic_cdk::api::id(),
                method: "transform".to_string(),
            }),
            context: vec![],
        }),
        headers: request_headers,
    };

    let cycles = 230_949_972_000;
    match http_request(request, cycles).await {
        Ok((response,)) => {
            let json_response = String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.");
            let parsed_response: serde_json::Value = serde_json::from_str(&json_response).expect("Failed to parse JSON response");
            let condition = parsed_response["current"]["condition"]["text"].as_str().unwrap();
            format!("Current weather condition in {}: {}", city, condition)
        }
        Err((r, m)) => {
            let message = format!(
                "The http_request resulted into error. RejectionCode: {:?}, Error: {}",
                r, m
            );

            message
        }
    }
}

#[query]
fn transform(raw: TransformArgs) -> HttpResponse {
    let headers = vec![
        HttpHeader {
            name: "Content-Security-Policy".to_string(),
            value: "default-src 'self'".to_string(),
        },
        HttpHeader {
            name: "Referrer-Policy".to_string(),
            value: "strict-origin".to_string(),
        },
        HttpHeader {
            name: "Permissions-Policy".to_string(),
            value: "geolocation=(self)".to_string(),
        },
        HttpHeader {
            name: "Strict-Transport-Security".to_string(),
            value: "max-age=63072000".to_string(),
        },
        HttpHeader {
            name: "X-Frame-Options".to_string(),
            value: "DENY".to_string(),
        },
        HttpHeader {
            name: "X-Content-Type-Options".to_string(),
            value: "nosniff".to_string(),
        },
    ];

    let mut res = HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers,
    };

    if res.status == 200u64 {
        res.body = raw.response.body;
    } else {
        ic_cdk::api::print(format!("Received an error from coinbase: err = {:?}", raw));
    }
    res
}
