use super::{AllChainsResponse, Contract, ContractSummary, DEFAULT_BASE_URL};
use crate::{Error, Result};
use reqwest::Client as HttpClient;
use std::fmt::Display;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Client {
    http: Arc<HttpClient>,
    base_url: String,
}

impl Client {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self::with_base_url(http, DEFAULT_BASE_URL)
    }

    pub fn with_base_url(http: Arc<HttpClient>, base_url: impl Into<String>) -> Self {
        Self {
            http,
            base_url: base_url.into().trim_end_matches('/').to_string(),
        }
    }

    pub async fn get_contract(
        &self,
        chain_id: impl Display,
        address: impl AsRef<str>,
    ) -> Result<Option<Contract>> {
        self.get_contract_with_fields(chain_id, address, &[]).await
    }

    pub async fn get_contract_with_fields(
        &self,
        chain_id: impl Display,
        address: impl AsRef<str>,
        fields: &[&str],
    ) -> Result<Option<Contract>> {
        let address = normalize_address(address.as_ref())?;
        let url = format!("{}/v2/contract/{}/{}", self.base_url, chain_id, address);
        let request = self.http.get(url);
        let request = if fields.is_empty() {
            request
        } else {
            request.query(&[("fields", fields.join(","))])
        };
        let response = request.send().await?;

        if response.status().as_u16() == 404 {
            return Ok(None);
        }

        if !response.status().is_success() {
            return Err(Error::ApiError(format!(
                "Sourcify v2 contract lookup failed with {}",
                response.status()
            )));
        }

        let contract = response.json::<Contract>().await?;
        Ok(contract.is_verified().then_some(contract))
    }

    pub async fn get_contract_all_chains(
        &self,
        address: impl AsRef<str>,
    ) -> Result<Vec<ContractSummary>> {
        let address = normalize_address(address.as_ref())?;
        let url = format!("{}/v2/contract/all-chains/{}", self.base_url, address);
        let response = self.http.get(url).send().await?;

        if !response.status().is_success() {
            return Err(Error::ApiError(format!(
                "Sourcify v2 all-chain lookup failed with {}",
                response.status()
            )));
        }

        Ok(response.json::<AllChainsResponse>().await?.results)
    }

    pub async fn is_verified(
        &self,
        chain_id: impl Display,
        address: impl AsRef<str>,
    ) -> Result<bool> {
        Ok(self.get_contract(chain_id, address).await?.is_some())
    }
}

fn normalize_address(address: &str) -> Result<&str> {
    let address = address.trim();
    let hex = address
        .strip_prefix("0x")
        .or_else(|| address.strip_prefix("0X"));

    match hex {
        Some(hex) if hex.len() == 40 && hex.chars().all(|c| c.is_ascii_hexdigit()) => Ok(address),
        _ => Err(Error::InvalidAddress(address.to_string())),
    }
}
