This crate contains structures for
[Google Cloud Structured logging](https://cloud.google.com/logging/docs/structured-logging).
This allows for adding more metadata to log statements that will be interpreted by the
[Google Cloud "Logging"][Cloud_Logging] service and can be viewed in the "Logs Explorer".

Some errors can also be formatted so the ["Error Reporting"][Error_Reporting] service will
group them.

[Cloud_Logging]: https://cloud.google.com/logging/
[Error_Reporting]: https://cloud.google.com/error-reporting/.

Here you can see a snippet of how you can use it in you logging library.
```rust
let log_entry = GoogleCloudStructLog {
    severity: Some(match level {
        Level::Error => GCLogSeverity::Error,
        Level::Warn => GCLogSeverity::Warning,
        Level::Info => GCLogSeverity::Info,
        Level::Debug => GCLogSeverity::Debug,
        Level::Trace => GCLogSeverity::Default,
    }),
    report_type: match level {
        // More info see: https://cloud.google.com/error-reporting/docs/formatting-error-messages#@type
        Level::Error => Some("type.googleapis.com/google.devtools.clouderrorreporting.v1beta1.ReportedErrorEvent".to_owned()),
        _ => None,
    },
    message: Some(
        format!(
            "{}{}", 
            record.args(),
            example_backtrace(),
        )
    ),
    operation: Some(GCOperation {
        id: Some("My Service"),
        producer: Some("MyService.Backend"),
        ..Default::default()
    }),
    source_location: Some(GCSourceLocation {
        file: record.file_static(),
        line: record.line().map(|s| s.to_string()),
        function: record.module_path_static(),
    }),
    time: Some(Utc::now()),
    ..Default::default()
};
println!(
    "{}",
    serde_json::to_string(&log_entry).expect("Error during logging")
);
```

To run the example use: `cargo run --example log`
This will result in the following output:
```json
{"severity":"info","message":"Start logging:\n   at services::module_name::he77c0bac773c93b4 line: 42\n   at services::module_name::h7ad5e699ac5d6658","time":"2021-12-20T16:33:41.643966093Z","logging.googleapis.com/operation":{"id":"My Service","producer":"MyService.Backend"},"logging.googleapis.com/sourceLocation":{"file":"examples/log/main.rs","line":"11","function":"log"}}
{"severity":"warning","message":"Oh no, things might go wrong soon.:\n   at services::module_name::he77c0bac773c93b4 line: 42\n   at services::module_name::h7ad5e699ac5d6658","time":"2021-12-20T16:33:41.644085317Z","logging.googleapis.com/operation":{"id":"My Service","producer":"MyService.Backend"},"logging.googleapis.com/sourceLocation":{"file":"examples/log/main.rs","line":"12","function":"log"}}
{"severity":"error","message":"Yeah, this is not good.:\n   at services::module_name::he77c0bac773c93b4 line: 42\n   at services::module_name::h7ad5e699ac5d6658","@type":"type.googleapis.com/google.devtools.clouderrorreporting.v1beta1.ReportedErrorEvent","time":"2021-12-20T16:33:41.644179397Z","logging.googleapis.com/operation":{"id":"My Service","producer":"MyService.Backend"},"logging.googleapis.com/sourceLocation":{"file":"examples/log/main.rs","line":"13","function":"log"}}
{"severity":"default","message":"Something went wrong in `my service`.:\n   at services::module_name::he77c0bac773c93b4 line: 42\n   at services::module_name::h7ad5e699ac5d6658","time":"2021-12-20T16:33:41.644276526Z","logging.googleapis.com/operation":{"id":"My Service","producer":"MyService.Backend"},"logging.googleapis.com/sourceLocation":{"file":"examples/log/main.rs","line":"14","function":"log"}}
```
Each line in the output above is 1 log message. Each log message is a json string.
Note that this is **not** an array of json messages. Each json object is separated by a newline.

<details>
    <summary>View pretty print:</summary>

```json
{
  "severity": "info",
  "message": "Start logging:\n   at services::module_name::he77c0bac773c93b4 line: 42\n   at services::module_name::h7ad5e699ac5d6658",
  "time": "2021-12-20T16:38:13.654978059Z",
  "logging.googleapis.com/operation": {
    "id": "My Service",
    "producer": "MyService.Backend"
  },
  "logging.googleapis.com/sourceLocation": {
    "file": "examples/log/main.rs",
    "line": "11",
    "function": "log"
  }
}
{
  "severity": "warning",
  "message": "Oh no, things might go wrong soon.:\n   at services::module_name::he77c0bac773c93b4 line: 42\n   at services::module_name::h7ad5e699ac5d6658",
  "time": "2021-12-20T16:38:13.655138074Z",
  "logging.googleapis.com/operation": {
    "id": "My Service",
    "producer": "MyService.Backend"
  },
  "logging.googleapis.com/sourceLocation": {
    "file": "examples/log/main.rs",
    "line": "12",
    "function": "log"
  }
}
{
  "severity": "error",
  "message": "Yeah, this is not good.:\n   at services::module_name::he77c0bac773c93b4 line: 42\n   at services::module_name::h7ad5e699ac5d6658",
  "@type": "type.googleapis.com/google.devtools.clouderrorreporting.v1beta1.ReportedErrorEvent",
  "time": "2021-12-20T16:38:13.655236672Z",
  "logging.googleapis.com/operation": {
    "id": "My Service",
    "producer": "MyService.Backend"
  },
  "logging.googleapis.com/sourceLocation": {
    "file": "examples/log/main.rs",
    "line": "13",
    "function": "log"
  }
}
{
  "severity": "default",
  "message": "Something went wrong in `my service`.:\n   at services::module_name::he77c0bac773c93b4 line: 42\n   at services::module_name::h7ad5e699ac5d6658",
  "time": "2021-12-20T16:38:13.655335729Z",
  "logging.googleapis.com/operation": {
    "id": "My Service",
    "producer": "MyService.Backend"
  },
  "logging.googleapis.com/sourceLocation": {
    "file": "examples/log/main.rs",
    "line": "14",
    "function": "log"
  }
}
```

</details>

## Tips

When logging your Rust service you might also want to capture panic message and format them
the same way. This can be done using a
[panic hook](https://doc.rust-lang.org/std/panic/fn.set_hook.html).

## License

The code in this project is licensed under the MIT or Apache 2.0 license.
All contributions, code and documentation, to this project will be similarly licensed.