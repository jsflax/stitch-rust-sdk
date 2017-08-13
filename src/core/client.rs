extern crate futures_cpupool;
extern crate curl;

use self::futures_cpupool::CpuPool;
use self::curl::easy::Easy;
use std::io::Read;
use std::str;

use core::error::StitchError;
use core::{Future, BoxFuture};
use auth::provider::AuthProvider;

use serde_json;
use serde_json::Map;
use serde_json::Value;

static AUTH_FIELDS_DEVICE: &'static str = "device";
static AUTH_FIELDS_OPTIONS: &'static str = "options";

static DEVICE_FIELDS_PLATFORM: &'static str = "platform";
static DEVICE_FIELDS_PLATFORM_VERSION: &'static str = "platformVersion";

static PLATFORM: &'static str = "rust";

pub struct StitchClient {
    pub app_id: &'static str,
    pub base_url: &'static str
}

trait StitchUtil {
    fn get_auth_request(&self, request: &mut Map<String, Value>) -> String;

    fn get_resource_path(&self, resource: &'static str) -> String;
}

pub trait StitchServer {
    fn login<T : AuthProvider>(&self, auth_provider: T) -> BoxFuture<bool, StitchError>;
}

impl StitchUtil for StitchClient {
    fn get_auth_request(&self, request: &mut Map<String, Value>) -> String {
        let options: Value = json!({
            AUTH_FIELDS_DEVICE: json!({
                DEVICE_FIELDS_PLATFORM: PLATFORM,
                DEVICE_FIELDS_PLATFORM_VERSION: "1.0"
            })
        });

        request.insert(String::from(AUTH_FIELDS_OPTIONS), options);
        match serde_json::to_string(request) {
            Ok(string) => return string,
            Err(error) => panic!("{:?}", error)
        }
    }

    fn get_resource_path(&self, resource: &'static str) -> String {
        return format!("{}/api/client/v1.0/app/{}/{}", self.base_url, self.app_id, resource);
    }
}

impl StitchServer for StitchClient {
    fn login<T : AuthProvider>(&self, auth_provider: T) -> BoxFuture<bool, StitchError> {
        let pool = CpuPool::new_num_cpus();

        let url = format!(
            "{}/{}/{}",
            self.get_resource_path("auth"),
            auth_provider.auth_type(),
            auth_provider.name()
        );
        
        let auth_request = self.get_auth_request(
            auth_provider.payload().as_object_mut().unwrap()
        );

        return pool.spawn_fn(move || {
            let mut data = auth_request.as_bytes();

            let mut easy = Easy::new();
            easy.url(&url).unwrap();
            easy.post(true).unwrap();
            easy.post_field_size(data.len() as u64).unwrap();

            let mut response = Vec::new();
            let mut transfer = easy.transfer();

            transfer.read_function(|buf| {
                Ok(data.read(buf).unwrap_or(0))
            }).unwrap();

            {
                let mut transfer = transfer;
                transfer.write_function(|new_data| {
                    response.extend_from_slice(new_data);
                    Ok(new_data.len())
                });
                transfer.perform().unwrap();
            }

            let json: Value = serde_json::from_str(
                str::from_utf8(&response).unwrap()
            ).unwrap();
            let access_token = &json["accessToken"];

            println!("{:?}", access_token);
            Ok(true)
        }).boxed();
    }
}
