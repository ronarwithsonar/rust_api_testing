pub mod public_user_world {
    use cucumber::World;
    use serde::Deserialize;
    use std::collections::HashMap;
    use std::env;

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

    #[derive(Debug, Default)]
    pub struct Response {
        pub response_status: String,
        pub response_body: HashMap<String, String>,
    }

    impl Response {
        pub fn init(&mut self) {
            self.response_status = String::new();
            self.response_body = HashMap::new();
        }
    }

    // `World` is your shared, likely mutable state.
    // Cucumber constructs it via `Default::default()` for each scenario. 
    #[derive(Debug, Default, World)]
    pub struct PublicWorld {
        pub response: Response,
    }

    #[derive(Deserialize)]
    pub struct SystemStatusResponse {
        pub result: SystemStatusResult,
    }

    #[derive(Deserialize)]
    pub struct SystemStatusResult {
        pub status: String,
        pub timestamp: String,
    }

    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    pub struct TickerResponse {
        pub error: Vec<String>,
        pub result: TickerResult,
    }

    #[derive(Deserialize, Debug)]
    pub struct TickerResult {
        #[serde(rename = "XXBTZUSD")]
        pub ticker_info: TickerInfo,
    }

    #[derive(Deserialize, Debug)]
    pub struct TickerInfo {
        a: Vec<String>,
        b: Vec<String>,
        c: Vec<String>,
        v: Vec<String>,
        p: Vec<String>,
        t: Vec<u32>,
        l: Vec<String>,
        h: Vec<String>,
        o: String,
    }

    impl TickerInfo {
        // Method to convert TickerInfo to a HashMap
        pub fn to_hashmap(&self) -> HashMap<String, String> {
            let mut map: HashMap<String, String> = HashMap::new();

            map.insert("ask".to_string(), format!("{:?}", self.a));
            map.insert("bid".to_string(), format!("{:?}", self.b));
            map.insert("last_trade_closed".to_string(), format!("{:?}", self.c));
            map.insert("volume".to_string(), format!("{:?}", self.v));
            map.insert("weighted_average_price".to_string(), format!("{:?}", self.p));
            map.insert("trades".to_string(), format!("{:?}", self.t));
            map.insert("low".to_string(), format!("{:?}", self.l));
            map.insert("high".to_string(), format!("{:?}", self.h));
            map.insert("opening_price".to_string(), self.o.clone());

            map
        }
    }
}

pub mod public_user_steps {
    use cucumber::{given, when, then};
    use reqwest::Error;
    use super::public_user_world::{
        ApiHost,
        PublicWorld, 
        SystemStatusResponse, 
        TickerResponse
    };
    use std::collections::HashMap;
    use tracing::{debug, error};

    #[given("I am a public user")]
    fn given_public_user(world: &mut PublicWorld) {
        world.response.init();
    }

    #[when("I request the server time")]
    async fn when_request_server_time(world: &mut PublicWorld) -> Result<(), Error>  {
        let url = format!("{}/0/public/SystemStatus", ApiHost::default().host);
        let response: reqwest::Response = reqwest::get(url).await?;

        if response.status().is_success() {
            let system_status: SystemStatusResponse = response.json().await?;
            world.response.response_status = system_status.result.status;
            world.response.response_body
                .insert("timestamp".to_string(), system_status.result.timestamp);
        } else {
            error!("Failed to fetch data: {}", response.status());
        }

        Ok(())
    }

    #[when(regex = r#"I request information on "(.*)""#)]
    async fn when_request_currency_information(world: &mut PublicWorld, ccy_pair: String) -> Result<(), Error>  {
        let url: String = format!("{}/0/public/Ticker?pair={}", ApiHost::default().host, ccy_pair);
        let response: reqwest::Response = reqwest::get(&url).await?;

        if response.status().is_success() {
            let ticker_response: TickerResponse = response.json().await?;
            world.response.response_body = ticker_response.result.ticker_info.to_hashmap();
        } else {
            error!("Failed to fetch data: {}", response.status());
        }

        Ok(())
    }

    #[then(regex = r#"I should receive information on "(.*)""#)]
    async fn then_assert_currency_information(world: &mut PublicWorld, ccy_pair: String) {
        assert!(world.response.response_body.get("opening_price").is_some());
        assert!(world.response.response_body.get("ask").is_some());
        assert!(world.response.response_body.get("bid").is_some());
        assert!(world.response.response_body.get("last_trade_closed").is_some());
        assert!(world.response.response_body.get("volume").is_some());
        assert!(world.response.response_body.get("weighted_average_price").is_some());
        assert!(world.response.response_body.get("trades").is_some());
        assert!(world.response.response_body.get("low").is_some());
        assert!(world.response.response_body.get("high").is_some());

        let hashmap: HashMap<String, String> = world.response.response_body.clone();
        let json_string: String = serde_json::to_string_pretty(&hashmap).unwrap();

        debug!("Currency Pair: {}", ccy_pair);
        debug!("Currency Info: {:?}", json_string);
    }

    #[then("I should receive the current server time")]
    fn then_receive_server_time(world: &mut PublicWorld) {
        assert!(world.response.response_status.eq("online"));
        assert!(world.response.response_body.get("timestamp").is_some());

        debug!("Server Time: {:?}", world.response.response_body);
    }
}
