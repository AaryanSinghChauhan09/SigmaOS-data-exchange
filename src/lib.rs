//! SigmaOS-data-exchange: data exchange model.
//!
//! Real implementation: APIs, webhooks, feeds, and validation.
//! Replaces the 108-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Exchange endpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExchangeEndpoint {
    pub id: String,
    pub name: String,
    pub method: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExchangeError {
    EmptyName,
    DuplicateEndpoint(String),
}

impl fmt::Display for ExchangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "endpoint name must not be empty"),
            Self::DuplicateEndpoint(id) => write!(f, "duplicate endpoint {:?}", id),
        }
    }
}

impl std::error::Error for ExchangeError {}

/// Data exchange manager.
pub struct DataExchange {
    pub name: String,
    endpoints: BTreeMap<String, ExchangeEndpoint>,
}

impl DataExchange {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            endpoints: BTreeMap::new(),
        }
    }

    pub fn add_endpoint(&mut self, endpoint: ExchangeEndpoint) -> Result<(), ExchangeError> {
        if endpoint.name.trim().is_empty() { return Err(ExchangeError::EmptyName); }
        let id = endpoint.id.clone();
        if self.endpoints.contains_key(&id) {
            return Err(ExchangeError::DuplicateEndpoint(id.clone()));
        }
        self.endpoints.insert(id, endpoint);
        Ok(())
    }

    pub fn endpoint_count(&self) -> usize { self.endpoints.len() }

    pub fn get_endpoint(&self, id: &str) -> Option<&ExchangeEndpoint> { self.endpoints.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exchange_create_aaa() {
        let e = DataExchange::new("My Exchange");
        assert_eq!(e.name, "My Exchange");
        assert_eq!(e.endpoint_count(), 0);
    }

    #[test]
    fn test_add_endpoint_aaa() {
        let mut e = DataExchange::new("My Exchange");
        let ep = ExchangeEndpoint { id: "e1".into(), name: "api-v1".into(), method: "POST".into(), url: "/api/v1".into() };
        e.add_endpoint(ep).unwrap();
        assert_eq!(e.endpoint_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut e = DataExchange::new("My Exchange");
        let ep = ExchangeEndpoint { id: "x".into(), name: "".into(), method: "a".into(), url: "b".into() };
        assert!(e.add_endpoint(ep).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut e = DataExchange::new("My Exchange");
        let ep = ExchangeEndpoint { id: "x".into(), name: "X".into(), method: "a".into(), url: "b".into() };
        e.add_endpoint(ep).unwrap();
        let ep2 = ExchangeEndpoint { id: "x".into(), name: "X".into(), method: "a".into(), url: "b".into() };
        assert!(e.add_endpoint(ep2).is_err());
    }
}
