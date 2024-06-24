pub mod authenticated_user_world {
    use cucumber::World;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use tracing::info;
    use base64::{Engine as _, engine::general_purpose};
    use hmac::{Hmac, Mac};
    use serde_urlencoded;
    use sha2::{Digest, Sha256, Sha512};
    use std::env;

    #[derive(Debug, Default)]
    pub struct User {
        pub public_key: String,
        pub private_key: String,
        pub api_signature: String,
    }

    impl User {
        pub fn init(&mut self) {
            self.public_key = env::var("PUBLIC_KEY")
                .expect("Failed to get public key. Ensure you have set the PUBLIC_KEY variable");
            self.private_key = env::var("PRIVATE_KEY")
                .expect("Failed to get private key. Ensure you have set the PRIVATE_KEY variable");
        }

        pub fn generate_signature(urlpath: &str, data: &OpenOrdersRequest, secret: &str) -> String {

            type HmacSha512 = Hmac<Sha512>;

            let postdata = serde_urlencoded::to_string(data).expect("Failed to encode data");
            let encoded = format!("{}{}", data.nonce, postdata);
            let sha256_hash = Sha256::digest(encoded.as_bytes());
            let message = [urlpath.as_bytes(), &sha256_hash].concat();
            let secret_decoded = general_purpose::STANDARD.decode(secret)
                .expect("Failed to decode secret");
            let mut mac = HmacSha512::new_from_slice(&secret_decoded)
                .expect("Failed to create HMAC instance");
            mac.update(&message);
            let sigdigest = general_purpose::STANDARD.encode(mac.finalize().into_bytes());
            sigdigest
        }
    }

    pub struct ApiHost {
        pub host: String,
    }

    impl Default for ApiHost {
        fn default() -> Self {
            ApiHost {
                host: env::var("API_HOST")
                .expect("Failed to get API host. Ensure you have set the API_HOST variable"),
            }
        }
    }

    #[derive(Debug, Default, Serialize)]
    pub struct OpenOrdersRequest {
        pub nonce: u64,
    }

    #[derive(Serialize)]
    pub struct AddOrderRequest {
        pub nonce: u64,
        pub pair: String,
        #[serde(rename = "type")]
        pub order_type: String,
        pub ordertype: String,
        pub price: String,
        pub volume: String,
        pub leverage: String,
        pub time_in_force: String,
        pub trigger: String,
    }
    
    #[derive(Serialize)]
    pub struct CancelOrderRequest {
        pub nonce: u64,
        pub txid: String,
    }

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    pub struct OpenOrdersResponse {
        pub error: Vec<String>,
        pub result: Option<OpenOrdersResultData>,
    }

    impl Default for OpenOrdersResponse {
        fn default() -> Self {
            OpenOrdersResponse {
                error: Vec::new(),
                result: None,
            }
        }
    }
    
    #[derive(Debug, Deserialize, Clone)]
    pub struct OpenOrdersResultData {
        pub open: HashMap<String, Order>,
    }
    
    #[allow(dead_code)]
    #[derive(Debug, Default, Deserialize, Clone)]
    pub struct Order {
        pub refid: Option<String>,
        pub userref: u32,
        pub status: String,
        pub opentm: f64,
        pub starttm: u32,
        pub expiretm: u32,
        pub descr: OrderDescr,
        pub vol: String,
        pub vol_exec: String,
        pub cost: String,
        pub fee: String,
        pub price: String,
        pub stopprice: String,
        pub limitprice: String,
        pub misc: String,
        pub oflags: String,
        pub trades: Option<Vec<String>>,
    }
    
    #[allow(dead_code)]
    #[derive(Debug, Default, Deserialize, Clone)]
    pub struct OrderDescr {
        pub pair: String,
        #[serde(rename = "type")]
        pub order_type: String,
        pub ordertype: String,
        pub price: String,
        pub price2: String,
        pub leverage: String,
        pub order: String,
        pub close: String,
    }

    // `World` is your shared, likely mutable state.
    // Cucumber constructs it via `Default::default()` for each scenario. 
    #[derive(Debug, Default, World)]
    pub struct AuthenticatedWorld {
        pub user: User,
        pub open_orders_response: OpenOrdersResponse,
        pub nonce: u64,
    }

    impl AuthenticatedWorld {
        pub fn cleanup(&mut self) {
            info!("Open orders: {:?}", self
                .open_orders_response
                .result
                .as_mut()
                .expect("No open order response. Ensure you have fetched open orders.")
                .open
            );
            // cancel any working orders here
            self.user = User::default();
            self.open_orders_response = OpenOrdersResponse::default();
            self.nonce = 0;
        }

        pub fn not_implemented(&mut self) {
            unimplemented!("Step not implemented");
        }
    }

}

pub mod authenticated_user_steps {
    use chrono::Utc;
    use cucumber::{given, when, then};
    use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
    use reqwest::Error;
    use super::authenticated_user_world::{
        ApiHost,
        AuthenticatedWorld, 
        OpenOrdersRequest, 
        OpenOrdersResponse, 
        User
    };
    use tracing::error;

    #[given("I am authenticated")]
    async fn given_authenicated(world: &mut AuthenticatedWorld) {
        world.user.init();
        let nonce = Utc::now().timestamp_millis().to_string();
        world.nonce = nonce.parse().unwrap();


        // generate signature
        world.user.api_signature = User::generate_signature("/0/private/OpenOrders", 
            &OpenOrdersRequest {
                nonce: world.nonce 
            }, 
            &world.user.private_key);
    }

    #[when("I have no open orders")]
    async fn when_no_open_orders(world: &mut AuthenticatedWorld) -> Result<(), Error> {
        // request open orders
        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert("API-Key", HeaderValue::from_str(&world.user.public_key)
            .unwrap());
        headers.insert("API-Sign", HeaderValue::from_str(&world.user.api_signature)
            .unwrap());
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded; charset=utf-8"));

        let postdata: String = serde_urlencoded::to_string(
            &OpenOrdersRequest { 
                nonce: world.nonce 
            })
            .expect("Failed to encode data");

        let client: reqwest::Client = reqwest::Client::new();
        let response: reqwest::Response = client
            .post(format!("{}/0/private/OpenOrders", ApiHost::default().host))
            .headers(headers)
            .body(postdata)
            .send()
            .await?;

        if response.status().is_success() {
            let open_orders_response: OpenOrdersResponse = response.json().await?;
            world.open_orders_response = open_orders_response;
        } else {
            error!("Failed to fetch data: {}", response.status());
        }

        Ok(())
    }

    #[when("I have open orders")]
    async fn when_request_open_orders(world: &mut AuthenticatedWorld) -> Result<(), Error> {
        world.not_implemented();

        // generate signature
        // create a conditional stop loss order with a price higher than the current price

        // let _postdata: String = serde_urlencoded::to_string(
        //     &AddOrderRequest {
        //         nonce: world.nonce,
        //         pair: "XBTGBP".to_string(),
        //         order_type: "buy".to_string(),
        //         ordertype: "stop-loss".to_string(),
        //         price: env::var("TRIGGER").expect("600000"),
        //         volume: "0.0001".to_string(),
        //         leverage: "none".to_string(),
        //         time_in_force: "gtc".to_string(),
        //         trigger: "last".to_string(),
        //     })
        // .expect("Failed to encode data");

        // request open orders
        // add to world object
        Ok(())
    }

    #[then("I should see no orders")]
    async fn then_assert_no_orders(world: &mut AuthenticatedWorld) {
        assert!(world.open_orders_response.result.as_mut().unwrap().open.is_empty());
    }

    #[then("I should see my open orders")]
    fn then_assert_expected_number_of_orders(world: &mut AuthenticatedWorld) {
        world.not_implemented();

        // assert that the open orders has expected count
        assert_eq!(world.open_orders_response.result.as_mut().unwrap().open.len(), 1);

        // only one order id so lets get it from the open orders response
        
        // let mut open_order_id = world
        //     .open_orders_response
        //     .result
        //     .as_mut()
        //     .unwrap()
        //     .open
        //     .keys()
        //     .clone()
        //     .filter(|order_id| !order_id.is_empty())
        //     .collect::<Vec<_>>();

        // assert_eq!(world.open_orders_response.result.as_mut().unwrap().open.get(open_order_id.remove(1)).unwrap()
        //     .descr.pair, "XBTGBP");
        
        // assert_eq!(world.open_orders_response.result.as_mut().unwrap().open.get(open_order_id.remove(1)).unwrap()
        //     .descr.order_type, "stop-loss");

        // assert_eq!(world.open_orders_response.result.as_mut().unwrap().open.get(open_order_id.remove(1)).unwrap()
        //     .descr.order, "buy 0.00010000 XBTGBP @ stop loss 60000.0");

        // assert_eq!(world.open_orders_response.result.as_mut().unwrap().open.get(open_order_id.remove(1)).unwrap()
        //     .descr.leverage, "none");

        // assert_eq!(world.open_orders_response.result.as_mut().unwrap().open.get(open_order_id.remove(1)).unwrap()
        //     .descr.price, "60000.0");

        // assert_eq!(world.open_orders_response.result.as_mut().unwrap().open.get(open_order_id.remove(1)).unwrap()
        //     .vol, "0.00010000");

        // assert_eq!(world.open_orders_response.result.as_mut().unwrap().open.get(open_order_id.remove(1)).unwrap()
        //     .trades, None);
    }
}
