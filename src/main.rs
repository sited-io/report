use http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use http::{HeaderName, Method};
use tonic::transport::Server;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::trace::TraceLayer;

use report::api::sited_io::report::v1::report_service_server::ReportServiceServer;
use report::logging::{LogOnFailure, LogOnRequest, LogOnResponse};
use report::{get_env_var, ReportService};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize logging
    tracing_subscriber::fmt::init();

    // get required environment variables
    let host = get_env_var("HOST");

    // initialize octocrab client (GitHub client)
    let gh_app_id = get_env_var("GH_APP_ID").parse::<u64>().unwrap().into();
    let gh_app_private_key = jsonwebtoken::EncodingKey::from_rsa_pem(
        get_env_var("GH_APP_PRIVATE_KEY")
            .replace("\\n", "\n")
            .as_bytes(),
    )
    .unwrap();

    octocrab::initialise(
        octocrab::Octocrab::builder()
            .app(gh_app_id, gh_app_private_key)
            .build()?,
    );

    // configure gRPC health reporter
    let (mut health_reporter, health_service) =
        tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<ReportServiceServer<ReportService>>()
        .await;

    // configure gRPC reflection service
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(
            tonic_health::pb::FILE_DESCRIPTOR_SET,
        )
        .register_encoded_file_descriptor_set(
            report::api::sited_io::FILE_DESCRIPTOR_SET,
        )
        .build()
        .unwrap();

    let report_service = ReportService::build();

    tracing::log::info!("gRPC+web server listening on {}", host);

    Server::builder()
        .layer(
            TraceLayer::new_for_grpc()
                .on_request(LogOnRequest::default())
                .on_response(LogOnResponse::default())
                .on_failure(LogOnFailure::default()),
        )
        .layer(
            CorsLayer::new()
                .allow_headers([
                    AUTHORIZATION,
                    ACCEPT,
                    CONTENT_TYPE,
                    HeaderName::from_static("grpc-status"),
                    HeaderName::from_static("grpc-message"),
                    HeaderName::from_static("x-grpc-web"),
                    HeaderName::from_static("x-user-agent"),
                ])
                .allow_methods([Method::POST])
                .allow_origin(AllowOrigin::any())
                .allow_private_network(true),
        )
        .accept_http1(true)
        .add_service(tonic_web::enable(reflection_service))
        .add_service(tonic_web::enable(health_service))
        .add_service(tonic_web::enable(report_service))
        .serve(host.parse().unwrap())
        .await?;

    Ok(())
}
