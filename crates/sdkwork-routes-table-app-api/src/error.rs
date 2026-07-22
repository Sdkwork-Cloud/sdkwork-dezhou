use axum::http::{header, HeaderName, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use sdkwork_dezhou_table_service::{DezhouError, DezhouTablePage};
use sdkwork_utils_rust::{
    offset_list_page_data, offset_list_page_params_from_values, SdkWorkApiResponse,
    SdkWorkProblemDetail, SdkWorkResourceData, SdkWorkResultCode,
};
use sdkwork_web_core::WebRequestContext;
use serde::Serialize;

fn attach_trace_header(response: &mut Response, trace_id: &str) {
    if let Ok(value) = HeaderValue::from_str(trace_id) {
        response
            .headers_mut()
            .insert(HeaderName::from_static("x-sdkwork-trace-id"), value);
    }
}

fn success_response<T: Serialize>(data: T, context: &WebRequestContext) -> Response {
    let trace_id = context.resolved_trace_id();
    let mut response = (
        StatusCode::OK,
        Json(SdkWorkApiResponse::success(data, trace_id.clone())),
    )
        .into_response();
    attach_trace_header(&mut response, &trace_id);
    response
}

pub fn ok_envelope<T: Serialize>(item: T, context: &WebRequestContext) -> Response {
    success_response(SdkWorkResourceData { item }, context)
}

pub fn ok_page_envelope(page: DezhouTablePage, context: &WebRequestContext) -> Response {
    let params =
        offset_list_page_params_from_values(i64::from(page.page), i64::from(page.page_size));
    let total_items = i64::try_from(page.total).unwrap_or(i64::MAX);
    success_response(
        offset_list_page_data(page.items, total_items, params),
        context,
    )
}

pub fn map_dezhou_error(error: DezhouError, context: &WebRequestContext) -> Response {
    let result_code = match error.code() {
        "not_found" => SdkWorkResultCode::NotFound,
        "invalid" => SdkWorkResultCode::InvalidParameter,
        _ => SdkWorkResultCode::InternalError,
    };
    let status = StatusCode::from_u16(result_code.http_status_code())
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let trace_id = context.resolved_trace_id();
    let problem = SdkWorkProblemDetail::platform_enriched(
        result_code,
        error.message(),
        trace_id.clone(),
        context.problem_correlation().routing(),
    );
    let mut response = (status, Json(problem)).into_response();
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/problem+json"),
    );
    attach_trace_header(&mut response, &trace_id);
    response
}
