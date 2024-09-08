use tonic::{async_trait, Request, Response, Status};

use crate::api::sited_io::report::v1::report_service_server::{
    self, ReportServiceServer,
};
use crate::api::sited_io::report::v1::{
    CreateReportRequest, CreateReportResponse, ReportType,
};

pub struct ReportService {
    github_owner: String,
    github_repo: String,
}

impl ReportService {
    const GH_TAG_BUG: &'static str = "bug";
    const GH_TAG_FEATURE_REQUEST: &'static str = "feature request";
    const GH_TAG_QUESTION: &'static str = "question";

    pub fn build(
        github_owner: String,
        github_repo: String,
    ) -> ReportServiceServer<Self> {
        ReportServiceServer::new(Self {
            github_owner,
            github_repo,
        })
    }
}

#[async_trait]
impl report_service_server::ReportService for ReportService {
    async fn create_report(
        &self,
        request: Request<CreateReportRequest>,
    ) -> Result<Response<CreateReportResponse>, Status> {
        let CreateReportRequest {
            report_type,
            title,
            content,
        } = request.into_inner();

        let report_type = ReportType::try_from(report_type).map_err(|err| {
            tracing::log::error!("{err}");
            Status::invalid_argument("report_type")
        })?;

        let tag = match report_type {
            ReportType::Bug => Self::GH_TAG_BUG,
            ReportType::FeatureRequest => Self::GH_TAG_FEATURE_REQUEST,
            ReportType::Question | ReportType::Unspecified => {
                Self::GH_TAG_QUESTION
            }
        }
        .to_string();

        let gh_client = octocrab::instance();

        let installations =
            gh_client.apps().installations().send().await.unwrap();
        let installation = installations.items.first().unwrap();

        let response = gh_client
            .installation(installation.id)
            .issues(&self.github_owner, &self.github_repo)
            .create(title)
            .body(content)
            .labels(Some(vec![tag]))
            .send()
            .await
            .map_err(|err| {
                tracing::log::error!("{err}");
                Status::internal(err.to_string())
            })?;

        Ok(Response::new(CreateReportResponse {
            link: Some(response.html_url.to_string()),
        }))
    }
}
