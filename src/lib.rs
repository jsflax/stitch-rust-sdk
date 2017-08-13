pub mod core;
pub mod auth;
#[macro_use]
extern crate serde_json;
extern crate futures;

#[cfg(test)]
mod tests {
    use core::client::StitchClient;
    use auth::providers::anonymous::AnonymousAuthProvider;
    use core::client::StitchServer;
    use core::error::StitchError;
    use core::{Future, BoxFuture};
    
    #[test]
    fn it_works() {
        let client = StitchClient { 
            app_id: "test-jsf-fpleb",
            base_url: "https://stitch-dev.mongodb.com/"
        };
        
        println!("{}", "flerp");

        let result: BoxFuture<bool, StitchError> = client.login(AnonymousAuthProvider {});
        result.wait();
    }
}
