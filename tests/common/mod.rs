#[allow(dead_code)]
mod backend_fixture;

#[allow(dead_code)]
mod integration_fixture;

#[allow(unused_imports)]
pub use backend_fixture::{user_json, BackendFixture};

#[allow(unused_imports)]
pub use integration_fixture::IntegrationFixture;
