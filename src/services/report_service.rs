use tonic::{async_trait, Request, Response, Status};

use crate::api::peoplesmarkets::report::v1::report_service_server::{
    self, ReportServiceServer,
};
use crate::api::peoplesmarkets::report::v1::{
    CreateReportRequest, CreateReportResponse, ReportType,
};

pub struct ReportService {}

impl ReportService {
    const GH_OWNER: &str = "peoplesmarkets";
    const GH_REPO: &str = "Project";

    const GH_TAG_BUG: &str = "bug";
    const GH_TAG_FEATURE_REQUEST: &str = "feature request";
    const GH_TAG_QUESTION: &str = "question";

    fn new() -> Self {
        Self {}
    }

    pub fn build() -> ReportServiceServer<Self> {
        ReportServiceServer::new(Self::new())
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

        let response = gh_client
            .issues(Self::GH_OWNER, Self::GH_REPO)
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
            link: Some(response.url.to_string()),
        }))
    }
}
