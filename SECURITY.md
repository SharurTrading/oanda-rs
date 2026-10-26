# Security

Inject OANDA bearer tokens from the calling application. This library does not load environment files or persist credentials. `Client` and `ClientBuilder` redact tokens in `Debug` and zeroize their owned token string on drop. Caller and HTTP transport copies have their own lifetimes; this is not a process-wide erasure guarantee. do not log request headers, complete URLs with sensitive query data, or account payloads.

Practice and Live use their paired OANDA HTTPS hosts. Endpoint overrides are limited to loopback fixtures. Redirects are never followed, so the bearer token is never sent to a redirect target; `Error::Redirected` reports the target without its userinfo, query or fragment. Keep practice credentials separate from live credentials.

When a mutation returns `AmbiguousMutation`, or its future is cancelled after sending, query OANDA account and transaction state before calling `acknowledge_reconciliation`. The client will not retry a mutation for you. A `ClientRequestID` can help correlate an operation, but does not itself prove whether it completed.

Report vulnerabilities privately to the repository maintainers before public disclosure. Do not include live tokens, account identifiers, or full provider payloads in a report.
