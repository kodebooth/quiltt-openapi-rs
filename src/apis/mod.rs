use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                },
                serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String)
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            return Self::Json;
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return Self::Unsupported(content_type.to_string());
        }
    }
}

pub mod account_numbers_api;
pub mod akoya_api;
pub mod balances_api;
pub mod connections_api;
pub mod enrichment_api;
pub mod fingoal_api;
pub mod finicity_api;
pub mod institutions_api;
pub mod introspection_api;
pub mod mx_api;
pub mod ntropy_api;
pub mod pave_api;
pub mod plaid_api;
pub mod processor_tokens_api;
pub mod profiles_api;
pub mod session_tokens_api;
pub mod subscriptions_api;

pub mod configuration;

use std::sync::Arc;

pub trait Api {
    fn account_numbers_api(&self) -> &dyn account_numbers_api::AccountNumbersApi;
    fn akoya_api(&self) -> &dyn akoya_api::AkoyaApi;
    fn balances_api(&self) -> &dyn balances_api::BalancesApi;
    fn connections_api(&self) -> &dyn connections_api::ConnectionsApi;
    fn enrichment_api(&self) -> &dyn enrichment_api::EnrichmentApi;
    fn fingoal_api(&self) -> &dyn fingoal_api::FingoalApi;
    fn finicity_api(&self) -> &dyn finicity_api::FinicityApi;
    fn institutions_api(&self) -> &dyn institutions_api::InstitutionsApi;
    fn introspection_api(&self) -> &dyn introspection_api::IntrospectionApi;
    fn mx_api(&self) -> &dyn mx_api::MxApi;
    fn ntropy_api(&self) -> &dyn ntropy_api::NtropyApi;
    fn pave_api(&self) -> &dyn pave_api::PaveApi;
    fn plaid_api(&self) -> &dyn plaid_api::PlaidApi;
    fn processor_tokens_api(&self) -> &dyn processor_tokens_api::ProcessorTokensApi;
    fn profiles_api(&self) -> &dyn profiles_api::ProfilesApi;
    fn session_tokens_api(&self) -> &dyn session_tokens_api::SessionTokensApi;
    fn subscriptions_api(&self) -> &dyn subscriptions_api::SubscriptionsApi;
}

pub struct ApiClient {
    account_numbers_api: Box<dyn account_numbers_api::AccountNumbersApi>,
    akoya_api: Box<dyn akoya_api::AkoyaApi>,
    balances_api: Box<dyn balances_api::BalancesApi>,
    connections_api: Box<dyn connections_api::ConnectionsApi>,
    enrichment_api: Box<dyn enrichment_api::EnrichmentApi>,
    fingoal_api: Box<dyn fingoal_api::FingoalApi>,
    finicity_api: Box<dyn finicity_api::FinicityApi>,
    institutions_api: Box<dyn institutions_api::InstitutionsApi>,
    introspection_api: Box<dyn introspection_api::IntrospectionApi>,
    mx_api: Box<dyn mx_api::MxApi>,
    ntropy_api: Box<dyn ntropy_api::NtropyApi>,
    pave_api: Box<dyn pave_api::PaveApi>,
    plaid_api: Box<dyn plaid_api::PlaidApi>,
    processor_tokens_api: Box<dyn processor_tokens_api::ProcessorTokensApi>,
    profiles_api: Box<dyn profiles_api::ProfilesApi>,
    session_tokens_api: Box<dyn session_tokens_api::SessionTokensApi>,
    subscriptions_api: Box<dyn subscriptions_api::SubscriptionsApi>,
}

impl ApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            account_numbers_api: Box::new(account_numbers_api::AccountNumbersApiClient::new(configuration.clone())),
            akoya_api: Box::new(akoya_api::AkoyaApiClient::new(configuration.clone())),
            balances_api: Box::new(balances_api::BalancesApiClient::new(configuration.clone())),
            connections_api: Box::new(connections_api::ConnectionsApiClient::new(configuration.clone())),
            enrichment_api: Box::new(enrichment_api::EnrichmentApiClient::new(configuration.clone())),
            fingoal_api: Box::new(fingoal_api::FingoalApiClient::new(configuration.clone())),
            finicity_api: Box::new(finicity_api::FinicityApiClient::new(configuration.clone())),
            institutions_api: Box::new(institutions_api::InstitutionsApiClient::new(configuration.clone())),
            introspection_api: Box::new(introspection_api::IntrospectionApiClient::new(configuration.clone())),
            mx_api: Box::new(mx_api::MxApiClient::new(configuration.clone())),
            ntropy_api: Box::new(ntropy_api::NtropyApiClient::new(configuration.clone())),
            pave_api: Box::new(pave_api::PaveApiClient::new(configuration.clone())),
            plaid_api: Box::new(plaid_api::PlaidApiClient::new(configuration.clone())),
            processor_tokens_api: Box::new(processor_tokens_api::ProcessorTokensApiClient::new(configuration.clone())),
            profiles_api: Box::new(profiles_api::ProfilesApiClient::new(configuration.clone())),
            session_tokens_api: Box::new(session_tokens_api::SessionTokensApiClient::new(configuration.clone())),
            subscriptions_api: Box::new(subscriptions_api::SubscriptionsApiClient::new(configuration.clone())),
        }
    }
}

impl Api for ApiClient {
    fn account_numbers_api(&self) -> &dyn account_numbers_api::AccountNumbersApi {
        self.account_numbers_api.as_ref()
    }
    fn akoya_api(&self) -> &dyn akoya_api::AkoyaApi {
        self.akoya_api.as_ref()
    }
    fn balances_api(&self) -> &dyn balances_api::BalancesApi {
        self.balances_api.as_ref()
    }
    fn connections_api(&self) -> &dyn connections_api::ConnectionsApi {
        self.connections_api.as_ref()
    }
    fn enrichment_api(&self) -> &dyn enrichment_api::EnrichmentApi {
        self.enrichment_api.as_ref()
    }
    fn fingoal_api(&self) -> &dyn fingoal_api::FingoalApi {
        self.fingoal_api.as_ref()
    }
    fn finicity_api(&self) -> &dyn finicity_api::FinicityApi {
        self.finicity_api.as_ref()
    }
    fn institutions_api(&self) -> &dyn institutions_api::InstitutionsApi {
        self.institutions_api.as_ref()
    }
    fn introspection_api(&self) -> &dyn introspection_api::IntrospectionApi {
        self.introspection_api.as_ref()
    }
    fn mx_api(&self) -> &dyn mx_api::MxApi {
        self.mx_api.as_ref()
    }
    fn ntropy_api(&self) -> &dyn ntropy_api::NtropyApi {
        self.ntropy_api.as_ref()
    }
    fn pave_api(&self) -> &dyn pave_api::PaveApi {
        self.pave_api.as_ref()
    }
    fn plaid_api(&self) -> &dyn plaid_api::PlaidApi {
        self.plaid_api.as_ref()
    }
    fn processor_tokens_api(&self) -> &dyn processor_tokens_api::ProcessorTokensApi {
        self.processor_tokens_api.as_ref()
    }
    fn profiles_api(&self) -> &dyn profiles_api::ProfilesApi {
        self.profiles_api.as_ref()
    }
    fn session_tokens_api(&self) -> &dyn session_tokens_api::SessionTokensApi {
        self.session_tokens_api.as_ref()
    }
    fn subscriptions_api(&self) -> &dyn subscriptions_api::SubscriptionsApi {
        self.subscriptions_api.as_ref()
    }
}

#[cfg(feature = "mockall")]
pub struct MockApiClient {
    pub account_numbers_api_mock: account_numbers_api::MockAccountNumbersApi,
    pub akoya_api_mock: akoya_api::MockAkoyaApi,
    pub balances_api_mock: balances_api::MockBalancesApi,
    pub connections_api_mock: connections_api::MockConnectionsApi,
    pub enrichment_api_mock: enrichment_api::MockEnrichmentApi,
    pub fingoal_api_mock: fingoal_api::MockFingoalApi,
    pub finicity_api_mock: finicity_api::MockFinicityApi,
    pub institutions_api_mock: institutions_api::MockInstitutionsApi,
    pub introspection_api_mock: introspection_api::MockIntrospectionApi,
    pub mx_api_mock: mx_api::MockMxApi,
    pub ntropy_api_mock: ntropy_api::MockNtropyApi,
    pub pave_api_mock: pave_api::MockPaveApi,
    pub plaid_api_mock: plaid_api::MockPlaidApi,
    pub processor_tokens_api_mock: processor_tokens_api::MockProcessorTokensApi,
    pub profiles_api_mock: profiles_api::MockProfilesApi,
    pub session_tokens_api_mock: session_tokens_api::MockSessionTokensApi,
    pub subscriptions_api_mock: subscriptions_api::MockSubscriptionsApi,
}

#[cfg(feature = "mockall")]
impl MockApiClient {
    pub fn new() -> Self {
        Self {
            account_numbers_api_mock: account_numbers_api::MockAccountNumbersApi::new(),
            akoya_api_mock: akoya_api::MockAkoyaApi::new(),
            balances_api_mock: balances_api::MockBalancesApi::new(),
            connections_api_mock: connections_api::MockConnectionsApi::new(),
            enrichment_api_mock: enrichment_api::MockEnrichmentApi::new(),
            fingoal_api_mock: fingoal_api::MockFingoalApi::new(),
            finicity_api_mock: finicity_api::MockFinicityApi::new(),
            institutions_api_mock: institutions_api::MockInstitutionsApi::new(),
            introspection_api_mock: introspection_api::MockIntrospectionApi::new(),
            mx_api_mock: mx_api::MockMxApi::new(),
            ntropy_api_mock: ntropy_api::MockNtropyApi::new(),
            pave_api_mock: pave_api::MockPaveApi::new(),
            plaid_api_mock: plaid_api::MockPlaidApi::new(),
            processor_tokens_api_mock: processor_tokens_api::MockProcessorTokensApi::new(),
            profiles_api_mock: profiles_api::MockProfilesApi::new(),
            session_tokens_api_mock: session_tokens_api::MockSessionTokensApi::new(),
            subscriptions_api_mock: subscriptions_api::MockSubscriptionsApi::new(),
        }
    }
}

#[cfg(feature = "mockall")]
impl Api for MockApiClient {
    fn account_numbers_api(&self) -> &dyn account_numbers_api::AccountNumbersApi {
        &self.account_numbers_api_mock
    }
    fn akoya_api(&self) -> &dyn akoya_api::AkoyaApi {
        &self.akoya_api_mock
    }
    fn balances_api(&self) -> &dyn balances_api::BalancesApi {
        &self.balances_api_mock
    }
    fn connections_api(&self) -> &dyn connections_api::ConnectionsApi {
        &self.connections_api_mock
    }
    fn enrichment_api(&self) -> &dyn enrichment_api::EnrichmentApi {
        &self.enrichment_api_mock
    }
    fn fingoal_api(&self) -> &dyn fingoal_api::FingoalApi {
        &self.fingoal_api_mock
    }
    fn finicity_api(&self) -> &dyn finicity_api::FinicityApi {
        &self.finicity_api_mock
    }
    fn institutions_api(&self) -> &dyn institutions_api::InstitutionsApi {
        &self.institutions_api_mock
    }
    fn introspection_api(&self) -> &dyn introspection_api::IntrospectionApi {
        &self.introspection_api_mock
    }
    fn mx_api(&self) -> &dyn mx_api::MxApi {
        &self.mx_api_mock
    }
    fn ntropy_api(&self) -> &dyn ntropy_api::NtropyApi {
        &self.ntropy_api_mock
    }
    fn pave_api(&self) -> &dyn pave_api::PaveApi {
        &self.pave_api_mock
    }
    fn plaid_api(&self) -> &dyn plaid_api::PlaidApi {
        &self.plaid_api_mock
    }
    fn processor_tokens_api(&self) -> &dyn processor_tokens_api::ProcessorTokensApi {
        &self.processor_tokens_api_mock
    }
    fn profiles_api(&self) -> &dyn profiles_api::ProfilesApi {
        &self.profiles_api_mock
    }
    fn session_tokens_api(&self) -> &dyn session_tokens_api::SessionTokensApi {
        &self.session_tokens_api_mock
    }
    fn subscriptions_api(&self) -> &dyn subscriptions_api::SubscriptionsApi {
        &self.subscriptions_api_mock
    }
}

