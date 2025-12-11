use crate::config::CraterConfig;
use crate::crater::types::{CreateExperimentRequest, Experiment, ExperimentList};
use crate::error::{BotError, Result};
use reqwest::Client;
use tracing::{debug, error, info};

pub struct CraterClient {
    client: Client,
    config: CraterConfig,
}

impl CraterClient {
    pub fn new(config: CraterConfig) -> Result<Self> {
        let client = Client::builder()
            .build()
            .map_err(|e| BotError::Internal(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { client, config })
    }

    fn auth_header(&self) -> String {
        format!("Bearer {}", self.config.api_token)
    }

    pub async fn create_experiment(&self, req: CreateExperimentRequest) -> Result<Experiment> {
        let url = format!("{}/api/v1/experiments", self.config.api_url);
        info!("Creating experiment: {}", req.name);
        debug!("Request: {:?}", req);

        let response = self
            .client
            .post(&url)
            .header("Authorization", self.auth_header())
            .json(&req)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Failed to create experiment: {} - {}", status, body);
            return Err(BotError::CraterApi(format!(
                "Failed to create experiment: {} - {}",
                status, body
            )));
        }

        let experiment = response.json::<Experiment>().await?;
        info!("Experiment created: {}", experiment.name);
        Ok(experiment)
    }

    pub async fn list_experiments(&self) -> Result<Vec<Experiment>> {
        let url = format!("{}/api/v1/experiments", self.config.api_url);
        info!("Listing experiments");

        let response = self
            .client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Failed to list experiments: {} - {}", status, body);
            return Err(BotError::CraterApi(format!(
                "Failed to list experiments: {} - {}",
                status, body
            )));
        }

        let list = response.json::<ExperimentList>().await?;
        Ok(list.experiments)
    }

    pub async fn get_experiment(&self, name: &str) -> Result<Experiment> {
        let url = format!("{}/api/v1/experiments/{}", self.config.api_url, name);
        info!("Getting experiment: {}", name);

        let response = self
            .client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Failed to get experiment: {} - {}", status, body);
            return Err(BotError::CraterApi(format!(
                "Failed to get experiment: {} - {}",
                status, body
            )));
        }

        let experiment = response.json::<Experiment>().await?;
        Ok(experiment)
    }

    pub async fn run_experiment(&self, name: &str) -> Result<()> {
        let url = format!("{}/api/v1/experiments/{}/run", self.config.api_url, name);
        info!("Running experiment: {}", name);

        let response = self
            .client
            .post(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Failed to run experiment: {} - {}", status, body);
            return Err(BotError::CraterApi(format!(
                "Failed to run experiment: {} - {}",
                status, body
            )));
        }

        info!("Experiment started: {}", name);
        Ok(())
    }

    pub async fn abort_experiment(&self, name: &str) -> Result<()> {
        let url = format!("{}/api/v1/experiments/{}/abort", self.config.api_url, name);
        info!("Aborting experiment: {}", name);

        let response = self
            .client
            .post(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Failed to abort experiment: {} - {}", status, body);
            return Err(BotError::CraterApi(format!(
                "Failed to abort experiment: {} - {}",
                status, body
            )));
        }

        info!("Experiment aborted: {}", name);
        Ok(())
    }

    pub async fn delete_experiment(&self, name: &str) -> Result<()> {
        let url = format!("{}/api/v1/experiments/{}", self.config.api_url, name);
        info!("Deleting experiment: {}", name);

        let response = self
            .client
            .delete(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Failed to delete experiment: {} - {}", status, body);
            return Err(BotError::CraterApi(format!(
                "Failed to delete experiment: {} - {}",
                status, body
            )));
        }

        info!("Experiment deleted: {}", name);
        Ok(())
    }
}
