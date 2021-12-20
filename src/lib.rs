//! This crate contains structures for
//! [Google Cloud Structured logging](https://cloud.google.com/logging/docs/structured-logging).
//! This allows for adding more metadata to log statements that will be interpreted by the
//! [Google Cloud "Logging"][Cloud_Logging] service and can be viewed in the "Logs Explorer".
//!
//! Some errors can also be formatted so the ["Error Reporting"][Error_Reporting] service will group them.
//!
//! [Cloud_Logging]: https://cloud.google.com/logging/
//! [Error_Reporting]: https://cloud.google.com/error-reporting/
#![forbid(unsafe_code)]
#![deny(clippy::all)]

// The code below contains documentation from both this library and the
// [Google Docs](https://cloud.google.com/logging/docs/reference/v2/rest/v2/LogEntry)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The format expected by Google Cloud Platform logging service
/// https://cloud.google.com/logging/docs/structured-logging
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GoogleCloudStructLog<'a> {
    /// The Logging agent attempts to match a variety of common severity strings,
    /// which includes the list of LogSeverity strings recognized by the Logging API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<GCLogSeverity>,
    /// The message that appears on the log entry line in the Logs Explorer.
    ///
    /// Optionally add a backtrace here using following format (including newlines):
    /// ```text
    /// My normal log message goes here:
    ///    at services::module_name::he77c0bac773c93b4 line: 42
    ///    at services::module_name::h7ad5e699ac5d6658
    /// ```
    /// Note the `:` at the end of the log message and the 3 space and `at ` before each line of the
    /// backtrace. The ` line: <Nr>` is optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Can be used to set for Error reporting
    /// More info see: https://cloud.google.com/error-reporting/docs/formatting-error-messages#@type
    #[serde(rename = "@type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_type: Option<String>,
    /// A structured record in the format of the LogEntry HttpRequest field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_request: Option<GCHttpRequest>,
    /// Time of the log message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<DateTime<Utc>>,
    /// A unique identifier for the log entry.
    /// If you provide a value, then Logging considers other log entries in the same project,
    /// with the same timestamp, and with the same insertId to be duplicates which are removed
    /// in a single query result. However, there are no guarantees of de-duplication
    /// in the export of logs.
    #[serde(rename = "logging.googleapis.com/insertId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_id: Option<String>,
    /// A map of key, value pairs that provides additional information about the log entry.
    /// The labels can be user-defined or system-defined.
    #[serde(rename = "logging.googleapis.com/labels")]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, String>,
    /// Information about an operation associated with the log entry, if applicable.
    #[serde(rename = "logging.googleapis.com/operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<GCOperation<'a>>,
    /// Additional information about the source code location that produced the log entry.
    #[serde(rename = "logging.googleapis.com/sourceLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_location: Option<GCSourceLocation<'a>>,
    /// The span ID within the trace associated with the log entry.
    ///
    /// For Trace spans, this is the same format that the Trace API v2 uses:
    /// a 16-character hexadecimal encoding of an 8-byte array, such as `000000000000004a`.
    #[serde(rename = "logging.googleapis.com/spanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span_id: Option<String>,
    /// Resource name of the trace associated with the log entry, if any.
    /// If it contains a relative resource name, the name is assumed to be relative to
    /// `//tracing.googleapis.com`.
    /// Example: `projects/my-projectid/traces/06796866738c859f2f19b7cfb3214824`
    #[serde(rename = "logging.googleapis.com/trace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<String>,
    /// The sampling decision of the trace associated with the log entry.
    ///
    /// `true` means that the trace resource name in the trace field was sampled for
    /// storage in a trace backend. `false` means that the trace was not sampled for storage
    /// when this log entry was written, or the sampling decision was unknown at the time.
    /// A non-sampled trace value is still useful as a request correlation identifier.
    /// The default is `false`.
    #[serde(rename = "logging.googleapis.com/trace_sampled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_sampled: Option<bool>,
    /// Just to keep track of lifetime
    #[serde(skip_serializing)]
    pub phantom: Option<&'a str>,
}

/// The severity of the event described in a log entry, expressed as one of the standard severity
/// levels listed below.
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "camelCase")]
pub enum GCLogSeverity {
    /// The log entry has no assigned severity level.
    Default,
    /// Debug or trace information.
    Debug,
    /// Routine information, such as ongoing status or performance.
    Info,
    /// Normal but significant events, such as start up, shut down, or a configuration change.
    Notice,
    /// Warning events might cause problems.
    Warning,
    /// Error events are likely to cause problems.
    Error,
    /// Critical events cause more severe problems or outages.
    Critical,
    /// A person must take an action immediately.
    Alert,
    /// One or more systems are unusable.
    Emergency,
}

impl Default for GCLogSeverity {
    fn default() -> Self {
        GCLogSeverity::Default
    }
}

// Some values where not added because they will not be used.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GCHttpRequest {
    /// The request method. Examples: "GET", "HEAD", "PUT", "POST".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_method: Option<GCHttpMethod>,
    /// The scheme (http, https), the host name, the path and the query portion of
    /// the URL that was requested. Example: "http://example.com/some/info?color=red".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_url: Option<String>,
    /// The size of the HTTP request message in bytes,
    /// including the request headers and the request body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_size: Option<String>,
    /// The response code indicating the status of response. Examples: 200, 404.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    /// The size of the HTTP response message sent back to the client, in bytes,
    /// including the response headers and the response body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_size: Option<String>,
    /// The user agent sent by the client.
    /// Example: "Mozilla/4.0 (compatible; MSIE 6.0; Windows 98; Q312461; .NET CLR 1.0.3705)".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    /// The IP address (IPv4 or IPv6) of the client that issued the HTTP request.
    /// This field can include port information.
    /// Examples: "192.168.1.1", "10.0.0.1:80", "FE80::0202:B3FF:FE1E:8329".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_ip: Option<String>,
    /// The IP address (IPv4 or IPv6) of the origin server that the request was sent to.
    /// This field can include port information.
    /// Examples: "192.168.1.1", "10.0.0.1:80", "FE80::0202:B3FF:FE1E:8329".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_ip: Option<String>,
    /// The request processing latency on the server,
    /// from the time the request was received until the response was sent.
    ///
    /// A duration in seconds with up to nine fractional digits, terminated by 's'.
    /// Example: "3.5s".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<String>,
    /// Protocol used for the request. Examples: "HTTP/1.1", "HTTP/2", "websocket"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "camelCase")]
pub enum GCHttpMethod {
    Get,
    Head,
    Put,
    Post,
}

impl Default for GCHttpMethod {
    fn default() -> Self {
        GCHttpMethod::Get
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GCOperation<'a> {
    /// An arbitrary operation identifier.
    /// Log entries with the same identifier are assumed to be part of the same operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    /// An arbitrary producer identifier. The combination of id and producer must be globally unique.
    /// Examples for producer: "MyDivision.MyBigCompany.com", "github.com/MyProject/MyApplication".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer: Option<&'a str>,
    /// Set this to `true` if this is the first log entry in the operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<bool>,
    /// Set this to `true` if this is the last log entry in the operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GCSourceLocation<'a> {
    /// Source file name. Depending on the runtime environment,
    /// this might be a simple name or a fully-qualified name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<&'a str>,
    /// Line within the source file. 1-based; 0 indicates no line number available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<String>,
    /// Human-readable name of the function or method being invoked,
    /// with optional context such as the class or package name.
    /// This information may be used in contexts such as the logs viewer,
    /// where a file and line number are less meaningful. The format can vary by language.
    /// For example: qual.if.ied.Class.method (Java), dir/package.func (Go), function (Python).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<&'a str>,
}
