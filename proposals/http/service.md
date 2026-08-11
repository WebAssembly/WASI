<h1><a id="service"></a>World service</h1>
<p>The <code>wasi:http/service</code> world captures a broad category of HTTP services
including web applications, API servers, and proxies. It may be <code>include</code>d
in more specific worlds such as <code>wasi:http/middleware</code>.</p>
<ul>
<li>Imports:
<ul>
<li>interface <a href="#wasi_cli_types_0_3_1"><code>wasi:cli/types@0.3.1</code></a></li>
<li>interface <a href="#wasi_cli_stdout_0_3_1"><code>wasi:cli/stdout@0.3.1</code></a></li>
<li>interface <a href="#wasi_cli_stderr_0_3_1"><code>wasi:cli/stderr@0.3.1</code></a></li>
<li>interface <a href="#wasi_cli_stdin_0_3_1"><code>wasi:cli/stdin@0.3.1</code></a></li>
<li>interface <a href="#wasi_clocks_types_0_3_1"><code>wasi:clocks/types@0.3.1</code></a></li>
<li>interface <a href="#wasi_http_types_0_3_1"><code>wasi:http/types@0.3.1</code></a></li>
<li>interface <a href="#wasi_http_client_0_3_1"><code>wasi:http/client@0.3.1</code></a></li>
<li>interface <a href="#wasi_clocks_monotonic_clock_0_3_1"><code>wasi:clocks/monotonic-clock@0.3.1</code></a></li>
<li>interface <a href="#wasi_clocks_system_clock_0_3_1"><code>wasi:clocks/system-clock@0.3.1</code></a></li>
<li>interface <a href="#wasi_clocks_timezone_0_3_1"><code>wasi:clocks/timezone@0.3.1</code></a></li>
<li>interface <a href="#wasi_random_random_0_3_1"><code>wasi:random/random@0.3.1</code></a></li>
<li>interface <a href="#wasi_random_insecure_0_3_1"><code>wasi:random/insecure@0.3.1</code></a></li>
<li>interface <a href="#wasi_random_insecure_seed_0_3_1"><code>wasi:random/insecure-seed@0.3.1</code></a></li>
</ul>
</li>
<li>Exports:
<ul>
<li>interface <a href="#wasi_http_handler_0_3_1"><code>wasi:http/handler@0.3.1</code></a></li>
</ul>
</li>
</ul>
<h2><a id="wasi_cli_types_0_3_1"></a>Import interface wasi:cli/types@0.3.1</h2>
<hr />
<h3>Types</h3>
<h4><a id="error_code"></a><code>enum error-code</code></h4>
<h5>Enum Cases</h5>
<ul>
<li>
<p><a id="error_code.io"></a><code>io</code></p>
<p>Input/output error
</li>
<li>
<p><a id="error_code.illegal_byte_sequence"></a><code>illegal-byte-sequence</code></p>
<p>Invalid or incomplete multibyte or wide character
</li>
<li>
<p><a id="error_code.pipe"></a><code>pipe</code></p>
<p>Broken pipe
</li>
</ul>
<h2><a id="wasi_cli_stdout_0_3_1"></a>Import interface wasi:cli/stdout@0.3.1</h2>
<hr />
<h3>Types</h3>
<h4><a id="error_code"></a><code>type error-code</code></h4>
<p><a href="#error_code"><a href="#error_code"><code>error-code</code></a></a></p>
<p>
<hr />
<h3>Functions</h3>
<h4><a id="write_via_stream"></a><code>write-via-stream: func</code></h4>
<p>Write the given stream to stdout.</p>
<p>If the stream's writable end is dropped this function will either return
success once the entire contents of the stream have been written or an
error-code representing a failure.</p>
<p>Otherwise if there is an error the readable end of the stream will be
dropped and this function will return an error-code.</p>
<h5>Params</h5>
<ul>
<li><a id="write_via_stream.data"></a><code>data</code>: stream&lt;<code>u8</code>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="write_via_stream.0"></a> future&lt;result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;</li>
</ul>
<h2><a id="wasi_cli_stderr_0_3_1"></a>Import interface wasi:cli/stderr@0.3.1</h2>
<hr />
<h3>Types</h3>
<h4><a id="error_code"></a><code>type error-code</code></h4>
<p><a href="#error_code"><a href="#error_code"><code>error-code</code></a></a></p>
<p>
<hr />
<h3>Functions</h3>
<h4><a id="write_via_stream"></a><code>write-via-stream: func</code></h4>
<p>Write the given stream to stderr.</p>
<p>If the stream's writable end is dropped this function will either return
success once the entire contents of the stream have been written or an
error-code representing a failure.</p>
<p>Otherwise if there is an error the readable end of the stream will be
dropped and this function will return an error-code.</p>
<h5>Params</h5>
<ul>
<li><a id="write_via_stream.data"></a><code>data</code>: stream&lt;<code>u8</code>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="write_via_stream.0"></a> future&lt;result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;</li>
</ul>
<h2><a id="wasi_cli_stdin_0_3_1"></a>Import interface wasi:cli/stdin@0.3.1</h2>
<hr />
<h3>Types</h3>
<h4><a id="error_code"></a><code>type error-code</code></h4>
<p><a href="#error_code"><a href="#error_code"><code>error-code</code></a></a></p>
<p>
<hr />
<h3>Functions</h3>
<h4><a id="read_via_stream"></a><code>read-via-stream: func</code></h4>
<p>Return a stream for reading from stdin.</p>
<p>This function returns a stream which provides data read from stdin,
and a future to signal read results.</p>
<p>If the stream's readable end is dropped the future will resolve to success.</p>
<p>If the stream's writable end is dropped the future will either resolve to
success if stdin was closed by the writer or to an error-code if reading
failed for some other reason.</p>
<p>Multiple streams may be active at the same time. The behavior of concurrent
reads is implementation-specific.</p>
<h5>Return values</h5>
<ul>
<li><a id="read_via_stream.0"></a> (stream&lt;<code>u8</code>&gt;, future&lt;result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;)</li>
</ul>
<h2><a id="wasi_clocks_types_0_3_1"></a>Import interface wasi:clocks/types@0.3.1</h2>
<p>This interface common types used throughout wasi:clocks.</p>
<hr />
<h3>Types</h3>
<h4><a id="duration"></a><code>type duration</code></h4>
<p><code>u64</code></p>
<p>A duration of time, in nanoseconds.
<h2><a id="wasi_http_types_0_3_1"></a>Import interface wasi:http/types@0.3.1</h2>
<p>This interface defines all of the types and methods for implementing HTTP
Requests and Responses, as well as their headers, trailers, and bodies.</p>
<hr />
<h3>Types</h3>
<h4><a id="duration"></a><code>type duration</code></h4>
<p><a href="#duration"><a href="#duration"><code>duration</code></a></a></p>
<p>
<h4><a id="method"></a><code>variant method</code></h4>
<p>This type corresponds to HTTP standard Methods.</p>
<h5>Variant Cases</h5>
<ul>
<li><a id="method.get"></a><code>get</code></li>
<li><a id="method.head"></a><code>head</code></li>
<li><a id="method.post"></a><code>post</code></li>
<li><a id="method.put"></a><code>put</code></li>
<li><a id="method.delete"></a><code>delete</code></li>
<li><a id="method.connect"></a><code>connect</code></li>
<li><a id="method.options"></a><code>options</code></li>
<li><a id="method.trace"></a><code>trace</code></li>
<li><a id="method.patch"></a><code>patch</code></li>
<li><a id="method.other"></a><code>other</code>: <code>string</code></li>
</ul>
<h4><a id="scheme"></a><code>variant scheme</code></h4>
<p>This type corresponds to HTTP standard Related Schemes.</p>
<h5>Variant Cases</h5>
<ul>
<li><a id="scheme.http"></a><code>HTTP</code></li>
<li><a id="scheme.https"></a><code>HTTPS</code></li>
<li><a id="scheme.other"></a><code>other</code>: <code>string</code></li>
</ul>
<h4><a id="dns_error_payload"></a><code>record DNS-error-payload</code></h4>
<p>Defines the case payload type for <code>DNS-error</code> above:</p>
<h5>Record Fields</h5>
<ul>
<li><a id="dns_error_payload.rcode"></a><code>rcode</code>: option&lt;<code>string</code>&gt;</li>
<li><a id="dns_error_payload.info_code"></a><code>info-code</code>: option&lt;<code>u16</code>&gt;</li>
</ul>
<h4><a id="tls_alert_received_payload"></a><code>record TLS-alert-received-payload</code></h4>
<p>Defines the case payload type for <code>TLS-alert-received</code> above:</p>
<h5>Record Fields</h5>
<ul>
<li><a id="tls_alert_received_payload.alert_id"></a><code>alert-id</code>: option&lt;<code>u8</code>&gt;</li>
<li><a id="tls_alert_received_payload.alert_message"></a><code>alert-message</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="field_size_payload"></a><code>record field-size-payload</code></h4>
<p>Defines the case payload type for <code>HTTP-response-{header,trailer}-size</code> above:</p>
<h5>Record Fields</h5>
<ul>
<li><a id="field_size_payload.field_name"></a><a href="#field_name"><code>field-name</code></a>: option&lt;<code>string</code>&gt;</li>
<li><a id="field_size_payload.field_size"></a><code>field-size</code>: option&lt;<code>u32</code>&gt;</li>
</ul>
<h4><a id="error_code"></a><code>variant error-code</code></h4>
<p>These cases are inspired by the IANA HTTP Proxy Error Types:
<a href="https://www.iana.org/assignments/http-proxy-status/http-proxy-status.xhtml#table-http-proxy-error-types">https://www.iana.org/assignments/http-proxy-status/http-proxy-status.xhtml#table-http-proxy-error-types</a></p>
<h5>Variant Cases</h5>
<ul>
<li><a id="error_code.dns_timeout"></a><code>DNS-timeout</code></li>
<li><a id="error_code.dns_error"></a><code>DNS-error</code>: <a href="#dns_error_payload"><a href="#dns_error_payload"><code>DNS-error-payload</code></a></a></li>
<li><a id="error_code.destination_not_found"></a><code>destination-not-found</code></li>
<li><a id="error_code.destination_unavailable"></a><code>destination-unavailable</code></li>
<li><a id="error_code.destination_ip_prohibited"></a><code>destination-IP-prohibited</code></li>
<li><a id="error_code.destination_ip_unroutable"></a><code>destination-IP-unroutable</code></li>
<li><a id="error_code.connection_refused"></a><code>connection-refused</code></li>
<li><a id="error_code.connection_terminated"></a><code>connection-terminated</code></li>
<li><a id="error_code.connection_timeout"></a><code>connection-timeout</code></li>
<li><a id="error_code.connection_read_timeout"></a><code>connection-read-timeout</code></li>
<li><a id="error_code.connection_write_timeout"></a><code>connection-write-timeout</code></li>
<li><a id="error_code.connection_limit_reached"></a><code>connection-limit-reached</code></li>
<li><a id="error_code.tls_protocol_error"></a><code>TLS-protocol-error</code></li>
<li><a id="error_code.tls_certificate_error"></a><code>TLS-certificate-error</code></li>
<li><a id="error_code.tls_alert_received"></a><code>TLS-alert-received</code>: <a href="#tls_alert_received_payload"><a href="#tls_alert_received_payload"><code>TLS-alert-received-payload</code></a></a></li>
<li><a id="error_code.http_request_denied"></a><code>HTTP-request-denied</code></li>
<li><a id="error_code.http_request_length_required"></a><code>HTTP-request-length-required</code></li>
<li><a id="error_code.http_request_body_size"></a><code>HTTP-request-body-size</code>: option&lt;<code>u64</code>&gt;</li>
<li><a id="error_code.http_request_method_invalid"></a><code>HTTP-request-method-invalid</code></li>
<li><a id="error_code.http_request_uri_invalid"></a><code>HTTP-request-URI-invalid</code></li>
<li><a id="error_code.http_request_uri_too_long"></a><code>HTTP-request-URI-too-long</code></li>
<li><a id="error_code.http_request_header_section_size"></a><code>HTTP-request-header-section-size</code>: option&lt;<code>u32</code>&gt;</li>
<li><a id="error_code.http_request_header_size"></a><code>HTTP-request-header-size</code>: option&lt;<a href="#field_size_payload"><a href="#field_size_payload"><code>field-size-payload</code></a></a>&gt;</li>
<li><a id="error_code.http_request_trailer_section_size"></a><code>HTTP-request-trailer-section-size</code>: option&lt;<code>u32</code>&gt;</li>
<li><a id="error_code.http_request_trailer_size"></a><code>HTTP-request-trailer-size</code>: <a href="#field_size_payload"><a href="#field_size_payload"><code>field-size-payload</code></a></a></li>
<li><a id="error_code.http_response_incomplete"></a><code>HTTP-response-incomplete</code></li>
<li><a id="error_code.http_response_header_section_size"></a><code>HTTP-response-header-section-size</code>: option&lt;<code>u32</code>&gt;</li>
<li><a id="error_code.http_response_header_size"></a><code>HTTP-response-header-size</code>: <a href="#field_size_payload"><a href="#field_size_payload"><code>field-size-payload</code></a></a></li>
<li><a id="error_code.http_response_body_size"></a><code>HTTP-response-body-size</code>: option&lt;<code>u64</code>&gt;</li>
<li><a id="error_code.http_response_trailer_section_size"></a><code>HTTP-response-trailer-section-size</code>: option&lt;<code>u32</code>&gt;</li>
<li><a id="error_code.http_response_trailer_size"></a><code>HTTP-response-trailer-size</code>: <a href="#field_size_payload"><a href="#field_size_payload"><code>field-size-payload</code></a></a></li>
<li><a id="error_code.http_response_transfer_coding"></a><code>HTTP-response-transfer-coding</code>: option&lt;<code>string</code>&gt;</li>
<li><a id="error_code.http_response_content_coding"></a><code>HTTP-response-content-coding</code>: option&lt;<code>string</code>&gt;</li>
<li><a id="error_code.http_response_timeout"></a><code>HTTP-response-timeout</code></li>
<li><a id="error_code.http_upgrade_failed"></a><code>HTTP-upgrade-failed</code></li>
<li><a id="error_code.http_protocol_error"></a><code>HTTP-protocol-error</code></li>
<li><a id="error_code.loop_detected"></a><code>loop-detected</code></li>
<li><a id="error_code.configuration_error"></a><code>configuration-error</code></li>
<li><a id="error_code.internal_error"></a><code>internal-error</code>: option&lt;<code>string</code>&gt;<p>This is a catch-all error for anything that doesn't fit cleanly into a
more specific case. It also includes an optional string for an
unstructured description of the error. Users should not depend on the
string for diagnosing errors, as it's not required to be consistent
between implementations.
</li>
</ul>
<h4><a id="header_error"></a><code>variant header-error</code></h4>
<p>This type enumerates the different kinds of errors that may occur when
setting or appending to a <a href="#fields"><code>fields</code></a> resource.</p>
<h5>Variant Cases</h5>
<ul>
<li>
<p><a id="header_error.invalid_syntax"></a><code>invalid-syntax</code></p>
<p>This error indicates that a `field-name` or `field-value` was
syntactically invalid when used with an operation that sets headers in a
`fields`.
</li>
<li>
<p><a id="header_error.forbidden"></a><code>forbidden</code></p>
<p>This error indicates that a forbidden `field-name` was used when trying
to set a header in a `fields`.
</li>
<li>
<p><a id="header_error.immutable"></a><code>immutable</code></p>
<p>This error indicates that the operation on the `fields` was not
permitted because the fields are immutable.
</li>
<li>
<p><a id="header_error.size_exceeded"></a><code>size-exceeded</code></p>
<p>This error indicates that the operation would exceed an
implementation-defined limit on field sizes. This may apply to
an individual `field-value`, a single `field-name` plus all its
values, or the total aggregate size of all fields.
</li>
<li>
<p><a id="header_error.other"></a><code>other</code>: option&lt;<code>string</code>&gt;</p>
<p>This is a catch-all error for anything that doesn't fit cleanly into a
more specific case. Implementations can use this to extend the error
type without breaking existing code. It also includes an optional
string for an unstructured description of the error. Users should not
depend on the string for diagnosing errors, as it's not required to be
consistent between implementations.
</li>
</ul>
<h4><a id="request_options_error"></a><code>variant request-options-error</code></h4>
<p>This type enumerates the different kinds of errors that may occur when
setting fields of a <a href="#request_options"><code>request-options</code></a> resource.</p>
<h5>Variant Cases</h5>
<ul>
<li>
<p><a id="request_options_error.not_supported"></a><code>not-supported</code></p>
<p>Indicates the specified field is not supported by this implementation.
</li>
<li>
<p><a id="request_options_error.immutable"></a><code>immutable</code></p>
<p>Indicates that the operation on the `request-options` was not permitted
because it is immutable.
</li>
<li>
<p><a id="request_options_error.other"></a><code>other</code>: option&lt;<code>string</code>&gt;</p>
<p>This is a catch-all error for anything that doesn't fit cleanly into a
more specific case. Implementations can use this to extend the error
type without breaking existing code. It also includes an optional
string for an unstructured description of the error. Users should not
depend on the string for diagnosing errors, as it's not required to be
consistent between implementations.
</li>
</ul>
<h4><a id="field_name"></a><code>type field-name</code></h4>
<p><code>string</code></p>
<p>Field names are always strings.
<p>Field names should always be treated as case insensitive by the <a href="#fields"><code>fields</code></a>
resource for the purposes of equality checking.</p>
<h4><a id="field_value"></a><code>type field-value</code></h4>
<p><a href="#field_value"><a href="#field_value"><code>field-value</code></a></a></p>
<p>Field values should always be ASCII strings. However, in
reality, HTTP implementations often have to interpret malformed values,
so they are provided as a list of bytes.
<h4><a id="fields"></a><code>resource fields</code></h4>
<p>This following block defines the <a href="#fields"><code>fields</code></a> resource which corresponds to
HTTP standard Fields. Fields are a common representation used for both
Headers and Trailers.</p>
<p>A <a href="#fields"><code>fields</code></a> may be mutable or immutable. A <a href="#fields"><code>fields</code></a> created using the
constructor, <code>from-list</code>, or <code>clone</code> will be mutable, but a <a href="#fields"><code>fields</code></a>
resource given by other means (including, but not limited to,
<code>request.headers</code>) might be be immutable. In an immutable fields, the
<code>set</code>, <code>append</code>, and <code>delete</code> operations will fail with
<code>header-error.immutable</code>.</p>
<p>A <a href="#fields"><code>fields</code></a> resource should store <a href="#field_name"><code>field-name</code></a>s and <a href="#field_value"><code>field-value</code></a>s in their
original casing used to construct or mutate the <a href="#fields"><code>fields</code></a> resource. The <a href="#fields"><code>fields</code></a>
resource should use that original casing when returning them from a method.
The <a href="#fields"><code>fields</code></a> resource may use a different casing when serializing the value
for transmission, as doing so may improve encoding efficiency.</p>
<p>Implementations may impose limits on individual field values and on total
aggregate field section size. Operations that would exceed these limits
fail with <code>header-error.size-exceeded</code></p>
<h4><a id="headers"></a><code>type headers</code></h4>
<p><a href="#fields"><a href="#fields"><code>fields</code></a></a></p>
<p>Headers is an alias for Fields.
<h4><a id="trailers"></a><code>type trailers</code></h4>
<p><a href="#fields"><a href="#fields"><code>fields</code></a></a></p>
<p>Trailers is an alias for Fields.
<h4><a id="request"></a><code>resource request</code></h4>
<p>Represents an HTTP Request.</p>
<h4><a id="request_options"></a><code>resource request-options</code></h4>
<p>Parameters for making an HTTP Request. Each of these parameters is
currently an optional timeout applicable to the transport layer of the
HTTP protocol.</p>
<p>These timeouts are separate from any the user may use to bound an
asynchronous call.</p>
<h4><a id="status_code"></a><code>type status-code</code></h4>
<p><code>u16</code></p>
<p>This type corresponds to the HTTP standard Status Code.
<h4><a id="response"></a><code>resource response</code></h4>
<h2>Represents an HTTP Response.</h2>
<h3>Functions</h3>
<h4><a id="constructor_fields"></a><code>[constructor]fields: func</code></h4>
<p>Construct an empty HTTP Fields.</p>
<p>The resulting <a href="#fields"><code>fields</code></a> is mutable.</p>
<h5>Return values</h5>
<ul>
<li><a id="constructor_fields.0"></a> own&lt;<a href="#fields"><a href="#fields"><code>fields</code></a></a>&gt;</li>
</ul>
<h4><a id="static_fields_from_list"></a><code>[static]fields.from-list: func</code></h4>
<p>Construct an HTTP Fields.</p>
<p>The resulting <a href="#fields"><code>fields</code></a> is mutable.</p>
<p>The list represents each name-value pair in the Fields. Names
which have multiple values are represented by multiple entries in this
list with the same name.</p>
<p>The tuple is a pair of the field name, represented as a string, and
Value, represented as a list of bytes. In a valid Fields, all names
and values are valid UTF-8 strings. However, values are not always
well-formed, so they are represented as a raw list of bytes.</p>
<p>An error result will be returned if any header or value was
syntactically invalid, if a header was forbidden, or if the
entries would exceed an implementation size limit.</p>
<h5>Params</h5>
<ul>
<li><a id="static_fields_from_list.entries"></a><code>entries</code>: list&lt;(<a href="#field_name"><a href="#field_name"><code>field-name</code></a></a>, <a href="#field_value"><a href="#field_value"><code>field-value</code></a></a>)&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="static_fields_from_list.0"></a> result&lt;own&lt;<a href="#fields"><a href="#fields"><code>fields</code></a></a>&gt;, <a href="#header_error"><a href="#header_error"><code>header-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_fields_get"></a><code>[method]fields.get: func</code></h4>
<p>Get all of the values corresponding to a name. If the name is not present
in this <a href="#fields"><code>fields</code></a>, an empty list is returned. However, if the name is
present but empty, this is represented by a list with one or more
empty field-values present.</p>
<h5>Params</h5>
<ul>
<li><a id="method_fields_get.self"></a><code>self</code>: borrow&lt;<a href="#fields"><a href="#fields"><code>fields</code></a></a>&gt;</li>
<li><a id="method_fields_get.name"></a><code>name</code>: <a href="#field_name"><a href="#field_name"><code>field-name</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_fields_get.0"></a> list&lt;<a href="#field_value"><a href="#field_value"><code>field-value</code></a></a>&gt;</li>
</ul>
<h4><a id="method_fields_has"></a><code>[method]fields.has: func</code></h4>
<p>Returns <code>true</code> when the name is present in this <a href="#fields"><code>fields</code></a>. If the name is
syntactically invalid, <code>false</code> is returned.</p>
<h5>Params</h5>
<ul>
<li><a id="method_fields_has.self"></a><code>self</code>: borrow&lt;<a href="#fields"><a href="#fields"><code>fields</code></a></a>&gt;</li>
<li><a id="method_fields_has.name"></a><code>name</code>: <a href="#field_name"><a href="#field_name"><code>field-name</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_fields_has.0"></a> <code>bool</code></li>
</ul>
<h4><a id="method_fields_set"></a><code>[method]fields.set: func</code></h4>
<p>Set all of the values for a name. Clears any existing values for that
name, if they have been set.</p>
<p>Fails with <code>header-error.immutable</code> if the <a href="#fields"><code>fields</code></a> are immutable.</p>
<p>Fails with <code>header-error.size-exceeded</code> if the name or values would
exceed an implementation-defined size limit.</p>
<h5>Params</h5>
<ul>
<li><a id="method_fields_set.self"></a><code>self</code>: borrow&lt;<a href="#fields"><a href="#fields"><code>fields</code></a></a>&gt;</li>
<li><a id="method_fields_set.name"></a><code>name</code>: <a href="#field_name"><a href="#field_name"><code>field-name</code></a></a></li>
<li><a id="method_fields_set.value"></a><code>value</code>: list&lt;<a href="#field_value"><a href="#field_value"><code>field-value</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_fields_set.0"></a> result&lt;_, <a href="#header_error"><a href="#header_error"><code>header-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_fields_delete"></a><code>[method]fields.delete: func</code></h4>
<p>Delete all values for a name. Does nothing if no values for the name
exist.</p>
<p>Fails with <code>header-error.immutable</code> if the <a href="#fields"><code>fields</code></a> are immutable.</p>
<h5>Params</h5>
<ul>
<li><a id="method_fields_delete.self"></a><code>self</code>: borrow&lt;<a href="#fields"><a href="#fields"><code>fields</code></a></a>&gt;</li>
<li><a id="method_fields_delete.name"></a><code>name</code>: <a href="#field_name"><a href="#field_name"><code>field-name</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_fields_delete.0"></a> result&lt;_, <a href="#header_error"><a href="#header_error"><code>header-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_fields_get_and_delete"></a><code>[method]fields.get-and-delete: func</code></h4>
<p>Delete all values for a name. Does nothing if no values for the name
exist.</p>
<p>Returns all values previously corresponding to the name, if any.</p>
<p>Fails with <code>header-error.immutable</code> if the <a href="#fields"><code>fields</code></a> are immutable.</p>
<h5>Params</h5>
<ul>
<li><a id="method_fields_get_and_delete.self"></a><code>self</code>: borrow&lt;<a href="#fields"><a href="#fields"><code>fields</code></a></a>&gt;</li>
<li><a id="method_fields_get_and_delete.name"></a><code>name</code>: <a href="#field_name"><a href="#field_name"><code>field-name</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_fields_get_and_delete.0"></a> result&lt;list&lt;<a href="#field_value"><a href="#field_value"><code>field-value</code></a></a>&gt;, <a href="#header_error"><a href="#header_error"><code>header-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_fields_append"></a><code>[method]fields.append: func</code></h4>
<p>Append a value for a name. Does not change or delete any existing
values for that name.</p>
<p>Fails with <code>header-error.immutable</code> if the <a href="#fields"><code>fields</code></a> are immutable.</p>
<p>Fails with <code>header-error.size-exceeded</code> if the value would exceed
an implementation-defined size limit.</p>
<h5>Params</h5>
<ul>
<li><a id="method_fields_append.self"></a><code>self</code>: borrow&lt;<a href="#fields"><a href="#fields"><code>fields</code></a></a>&gt;</li>
<li><a id="method_fields_append.name"></a><code>name</code>: <a href="#field_name"><a href="#field_name"><code>field-name</code></a></a></li>
<li><a id="method_fields_append.value"></a><code>value</code>: <a href="#field_value"><a href="#field_value"><code>field-value</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_fields_append.0"></a> result&lt;_, <a href="#header_error"><a href="#header_error"><code>header-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_fields_copy_all"></a><code>[method]fields.copy-all: func</code></h4>
<p>Retrieve the full set of names and values in the Fields. Like the
constructor, the list represents each name-value pair.</p>
<p>The outer list represents each name-value pair in the Fields. Names
which have multiple values are represented by multiple entries in this
list with the same name.</p>
<p>The names and values are always returned in the original casing and in
the order in which they will be serialized for transport.</p>
<h5>Params</h5>
<ul>
<li><a id="method_fields_copy_all.self"></a><code>self</code>: borrow&lt;<a href="#fields"><a href="#fields"><code>fields</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_fields_copy_all.0"></a> list&lt;(<a href="#field_name"><a href="#field_name"><code>field-name</code></a></a>, <a href="#field_value"><a href="#field_value"><code>field-value</code></a></a>)&gt;</li>
</ul>
<h4><a id="method_fields_clone"></a><code>[method]fields.clone: func</code></h4>
<p>Make a deep copy of the Fields. Equivalent in behavior to calling the
<a href="#fields"><code>fields</code></a> constructor on the return value of <code>copy-all</code>. The resulting
<a href="#fields"><code>fields</code></a> is mutable.</p>
<h5>Params</h5>
<ul>
<li><a id="method_fields_clone.self"></a><code>self</code>: borrow&lt;<a href="#fields"><a href="#fields"><code>fields</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_fields_clone.0"></a> own&lt;<a href="#fields"><a href="#fields"><code>fields</code></a></a>&gt;</li>
</ul>
<h4><a id="static_request_new"></a><code>[static]request.new: func</code></h4>
<p>Construct a new <a href="#request"><code>request</code></a> with a default <a href="#method"><code>method</code></a> of <code>GET</code>, and
<code>none</code> values for <code>path-with-query</code>, <a href="#scheme"><code>scheme</code></a>, and <code>authority</code>.</p>
<p><a href="#headers"><code>headers</code></a> is the HTTP Headers for the Request.</p>
<p><code>contents</code> is the optional body content stream with <code>none</code>
representing a zero-length content stream.
Once it is closed, <a href="#trailers"><code>trailers</code></a> future must resolve to a result.
If <a href="#trailers"><code>trailers</code></a> resolves to an error, underlying connection
will be closed immediately.</p>
<p><code>options</code> is optional <a href="#request_options"><code>request-options</code></a> resource to be used
if the request is sent over a network connection.</p>
<p>It is possible to construct, or manipulate with the accessor functions
below, a <a href="#request"><code>request</code></a> with an invalid combination of <a href="#scheme"><code>scheme</code></a>
and <code>authority</code>, or <a href="#headers"><code>headers</code></a> which are not permitted to be sent.
It is the obligation of the <code>handler.handle</code> implementation
to reject invalid constructions of <a href="#request"><code>request</code></a>.</p>
<p>The returned future resolves to result of transmission of this request.</p>
<h5>Params</h5>
<ul>
<li><a id="static_request_new.headers"></a><a href="#headers"><code>headers</code></a>: own&lt;<a href="#headers"><a href="#headers"><code>headers</code></a></a>&gt;</li>
<li><a id="static_request_new.contents"></a><code>contents</code>: option&lt;stream&lt;<code>u8</code>&gt;&gt;</li>
<li><a id="static_request_new.trailers"></a><a href="#trailers"><code>trailers</code></a>: future&lt;result&lt;option&lt;own&lt;<a href="#trailers"><a href="#trailers"><code>trailers</code></a></a>&gt;&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;</li>
<li><a id="static_request_new.options"></a><code>options</code>: option&lt;own&lt;<a href="#request_options"><a href="#request_options"><code>request-options</code></a></a>&gt;&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="static_request_new.0"></a> (own&lt;<a href="#request"><a href="#request"><code>request</code></a></a>&gt;, future&lt;result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;)</li>
</ul>
<h4><a id="method_request_get_method"></a><code>[method]request.get-method: func</code></h4>
<p>Get the Method for the Request.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_get_method.self"></a><code>self</code>: borrow&lt;<a href="#request"><a href="#request"><code>request</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_get_method.0"></a> <a href="#method"><a href="#method"><code>method</code></a></a></li>
</ul>
<h4><a id="method_request_set_method"></a><code>[method]request.set-method: func</code></h4>
<p>Set the Method for the Request. Fails if the string present in a
<code>method.other</code> argument is not a syntactically valid method.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_set_method.self"></a><code>self</code>: borrow&lt;<a href="#request"><a href="#request"><code>request</code></a></a>&gt;</li>
<li><a id="method_request_set_method.method"></a><a href="#method"><code>method</code></a>: <a href="#method"><a href="#method"><code>method</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_set_method.0"></a> result</li>
</ul>
<h4><a id="method_request_get_path_with_query"></a><code>[method]request.get-path-with-query: func</code></h4>
<p>Get the combination of the HTTP Path and Query for the Request.  When
<code>none</code>, this represents an empty Path and empty Query.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_get_path_with_query.self"></a><code>self</code>: borrow&lt;<a href="#request"><a href="#request"><code>request</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_get_path_with_query.0"></a> option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="method_request_set_path_with_query"></a><code>[method]request.set-path-with-query: func</code></h4>
<p>Set the combination of the HTTP Path and Query for the Request.  When
<code>none</code>, this represents an empty Path and empty Query. Fails is the
string given is not a syntactically valid path and query uri component.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_set_path_with_query.self"></a><code>self</code>: borrow&lt;<a href="#request"><a href="#request"><code>request</code></a></a>&gt;</li>
<li><a id="method_request_set_path_with_query.path_with_query"></a><code>path-with-query</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_set_path_with_query.0"></a> result</li>
</ul>
<h4><a id="method_request_get_scheme"></a><code>[method]request.get-scheme: func</code></h4>
<p>Get the HTTP Related Scheme for the Request. When <code>none</code>, the
implementation may choose an appropriate default scheme.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_get_scheme.self"></a><code>self</code>: borrow&lt;<a href="#request"><a href="#request"><code>request</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_get_scheme.0"></a> option&lt;<a href="#scheme"><a href="#scheme"><code>scheme</code></a></a>&gt;</li>
</ul>
<h4><a id="method_request_set_scheme"></a><code>[method]request.set-scheme: func</code></h4>
<p>Set the HTTP Related Scheme for the Request. When <code>none</code>, the
implementation may choose an appropriate default scheme. Fails if the
string given is not a syntactically valid uri scheme.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_set_scheme.self"></a><code>self</code>: borrow&lt;<a href="#request"><a href="#request"><code>request</code></a></a>&gt;</li>
<li><a id="method_request_set_scheme.scheme"></a><a href="#scheme"><code>scheme</code></a>: option&lt;<a href="#scheme"><a href="#scheme"><code>scheme</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_set_scheme.0"></a> result</li>
</ul>
<h4><a id="method_request_get_authority"></a><code>[method]request.get-authority: func</code></h4>
<p>Get the authority of the Request's target URI. A value of <code>none</code> may be used
with Related Schemes which do not require an authority. The HTTP and
HTTPS schemes always require an authority.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_get_authority.self"></a><code>self</code>: borrow&lt;<a href="#request"><a href="#request"><code>request</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_get_authority.0"></a> option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="method_request_set_authority"></a><code>[method]request.set-authority: func</code></h4>
<p>Set the authority of the Request's target URI. A value of <code>none</code> may be used
with Related Schemes which do not require an authority. The HTTP and
HTTPS schemes always require an authority. Fails if the string given is
not a syntactically valid URI authority.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_set_authority.self"></a><code>self</code>: borrow&lt;<a href="#request"><a href="#request"><code>request</code></a></a>&gt;</li>
<li><a id="method_request_set_authority.authority"></a><code>authority</code>: option&lt;<code>string</code>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_set_authority.0"></a> result</li>
</ul>
<h4><a id="method_request_get_options"></a><code>[method]request.get-options: func</code></h4>
<p>Get the <a href="#request_options"><code>request-options</code></a> to be associated with this request</p>
<p>The returned <a href="#request_options"><code>request-options</code></a> resource is immutable: <code>set-*</code> operations
will fail if invoked.</p>
<p>This <a href="#request_options"><code>request-options</code></a> resource is a child: it must be dropped before
the parent <a href="#request"><code>request</code></a> is dropped, or its ownership is transferred to
another component by e.g. <code>handler.handle</code>.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_get_options.self"></a><code>self</code>: borrow&lt;<a href="#request"><a href="#request"><code>request</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_get_options.0"></a> option&lt;own&lt;<a href="#request_options"><a href="#request_options"><code>request-options</code></a></a>&gt;&gt;</li>
</ul>
<h4><a id="method_request_get_headers"></a><code>[method]request.get-headers: func</code></h4>
<p>Get the headers associated with the Request.</p>
<p>The returned <a href="#headers"><code>headers</code></a> resource is immutable: <code>set</code>, <code>append</code>, and
<code>delete</code> operations will fail with <code>header-error.immutable</code>.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_get_headers.self"></a><code>self</code>: borrow&lt;<a href="#request"><a href="#request"><code>request</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_get_headers.0"></a> own&lt;<a href="#headers"><a href="#headers"><code>headers</code></a></a>&gt;</li>
</ul>
<h4><a id="static_request_consume_body"></a><code>[static]request.consume-body: func</code></h4>
<p>Get body of the Request.</p>
<p>Stream returned by this method represents the contents of the body.
Once the stream is reported as closed, callers should await the returned
future to determine whether the body was received successfully.
The future will only resolve after the stream is reported as closed.</p>
<p>This function takes a <code>res</code> future as a parameter, which can be used to
communicate an error in handling of the request.</p>
<p>Note that function will move the <a href="#request"><code>request</code></a>, but references to headers or
request options acquired from it previously will remain valid.</p>
<h5>Params</h5>
<ul>
<li><a id="static_request_consume_body.this"></a><code>this</code>: own&lt;<a href="#request"><a href="#request"><code>request</code></a></a>&gt;</li>
<li><a id="static_request_consume_body.res"></a><code>res</code>: future&lt;result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="static_request_consume_body.0"></a> (stream&lt;<code>u8</code>&gt;, future&lt;result&lt;option&lt;own&lt;<a href="#trailers"><a href="#trailers"><code>trailers</code></a></a>&gt;&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;)</li>
</ul>
<h4><a id="constructor_request_options"></a><code>[constructor]request-options: func</code></h4>
<p>Construct a default <a href="#request_options"><code>request-options</code></a> value.</p>
<h5>Return values</h5>
<ul>
<li><a id="constructor_request_options.0"></a> own&lt;<a href="#request_options"><a href="#request_options"><code>request-options</code></a></a>&gt;</li>
</ul>
<h4><a id="method_request_options_get_connect_timeout"></a><code>[method]request-options.get-connect-timeout: func</code></h4>
<p>The timeout for the initial connect to the HTTP Server.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_options_get_connect_timeout.self"></a><code>self</code>: borrow&lt;<a href="#request_options"><a href="#request_options"><code>request-options</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_options_get_connect_timeout.0"></a> option&lt;<a href="#duration"><a href="#duration"><code>duration</code></a></a>&gt;</li>
</ul>
<h4><a id="method_request_options_set_connect_timeout"></a><code>[method]request-options.set-connect-timeout: func</code></h4>
<p>Set the timeout for the initial connect to the HTTP Server. An error
return value indicates that this timeout is not supported or that this
handle is immutable.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_options_set_connect_timeout.self"></a><code>self</code>: borrow&lt;<a href="#request_options"><a href="#request_options"><code>request-options</code></a></a>&gt;</li>
<li><a id="method_request_options_set_connect_timeout.duration"></a><a href="#duration"><code>duration</code></a>: option&lt;<a href="#duration"><a href="#duration"><code>duration</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_options_set_connect_timeout.0"></a> result&lt;_, <a href="#request_options_error"><a href="#request_options_error"><code>request-options-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_request_options_get_first_byte_timeout"></a><code>[method]request-options.get-first-byte-timeout: func</code></h4>
<p>The timeout for receiving the first byte of the Response body.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_options_get_first_byte_timeout.self"></a><code>self</code>: borrow&lt;<a href="#request_options"><a href="#request_options"><code>request-options</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_options_get_first_byte_timeout.0"></a> option&lt;<a href="#duration"><a href="#duration"><code>duration</code></a></a>&gt;</li>
</ul>
<h4><a id="method_request_options_set_first_byte_timeout"></a><code>[method]request-options.set-first-byte-timeout: func</code></h4>
<p>Set the timeout for receiving the first byte of the Response body. An
error return value indicates that this timeout is not supported or that
this handle is immutable.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_options_set_first_byte_timeout.self"></a><code>self</code>: borrow&lt;<a href="#request_options"><a href="#request_options"><code>request-options</code></a></a>&gt;</li>
<li><a id="method_request_options_set_first_byte_timeout.duration"></a><a href="#duration"><code>duration</code></a>: option&lt;<a href="#duration"><a href="#duration"><code>duration</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_options_set_first_byte_timeout.0"></a> result&lt;_, <a href="#request_options_error"><a href="#request_options_error"><code>request-options-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_request_options_get_between_bytes_timeout"></a><code>[method]request-options.get-between-bytes-timeout: func</code></h4>
<p>The timeout for receiving subsequent chunks of bytes in the Response
body stream.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_options_get_between_bytes_timeout.self"></a><code>self</code>: borrow&lt;<a href="#request_options"><a href="#request_options"><code>request-options</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_options_get_between_bytes_timeout.0"></a> option&lt;<a href="#duration"><a href="#duration"><code>duration</code></a></a>&gt;</li>
</ul>
<h4><a id="method_request_options_set_between_bytes_timeout"></a><code>[method]request-options.set-between-bytes-timeout: func</code></h4>
<p>Set the timeout for receiving subsequent chunks of bytes in the Response
body stream. An error return value indicates that this timeout is not
supported or that this handle is immutable.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_options_set_between_bytes_timeout.self"></a><code>self</code>: borrow&lt;<a href="#request_options"><a href="#request_options"><code>request-options</code></a></a>&gt;</li>
<li><a id="method_request_options_set_between_bytes_timeout.duration"></a><a href="#duration"><code>duration</code></a>: option&lt;<a href="#duration"><a href="#duration"><code>duration</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_options_set_between_bytes_timeout.0"></a> result&lt;_, <a href="#request_options_error"><a href="#request_options_error"><code>request-options-error</code></a></a>&gt;</li>
</ul>
<h4><a id="method_request_options_clone"></a><code>[method]request-options.clone: func</code></h4>
<p>Make a deep copy of the <a href="#request_options"><code>request-options</code></a>.
The resulting <a href="#request_options"><code>request-options</code></a> is mutable.</p>
<h5>Params</h5>
<ul>
<li><a id="method_request_options_clone.self"></a><code>self</code>: borrow&lt;<a href="#request_options"><a href="#request_options"><code>request-options</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_request_options_clone.0"></a> own&lt;<a href="#request_options"><a href="#request_options"><code>request-options</code></a></a>&gt;</li>
</ul>
<h4><a id="static_response_new"></a><code>[static]response.new: func</code></h4>
<p>Construct a new <a href="#response"><code>response</code></a>, with a default <a href="#status_code"><code>status-code</code></a> of <code>200</code>.
If a different <a href="#status_code"><code>status-code</code></a> is needed, it must be set via the
<code>set-status-code</code> method.</p>
<p><a href="#headers"><code>headers</code></a> is the HTTP Headers for the Response.</p>
<p><code>contents</code> is the optional body content stream with <code>none</code>
representing a zero-length content stream.
Once it is closed, <a href="#trailers"><code>trailers</code></a> future must resolve to a result.
If <a href="#trailers"><code>trailers</code></a> resolves to an error, underlying connection
will be closed immediately.</p>
<p>The returned future resolves to result of transmission of this response.</p>
<h5>Params</h5>
<ul>
<li><a id="static_response_new.headers"></a><a href="#headers"><code>headers</code></a>: own&lt;<a href="#headers"><a href="#headers"><code>headers</code></a></a>&gt;</li>
<li><a id="static_response_new.contents"></a><code>contents</code>: option&lt;stream&lt;<code>u8</code>&gt;&gt;</li>
<li><a id="static_response_new.trailers"></a><a href="#trailers"><code>trailers</code></a>: future&lt;result&lt;option&lt;own&lt;<a href="#trailers"><a href="#trailers"><code>trailers</code></a></a>&gt;&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="static_response_new.0"></a> (own&lt;<a href="#response"><a href="#response"><code>response</code></a></a>&gt;, future&lt;result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;)</li>
</ul>
<h4><a id="method_response_get_status_code"></a><code>[method]response.get-status-code: func</code></h4>
<p>Get the HTTP Status Code for the Response.</p>
<h5>Params</h5>
<ul>
<li><a id="method_response_get_status_code.self"></a><code>self</code>: borrow&lt;<a href="#response"><a href="#response"><code>response</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_response_get_status_code.0"></a> <a href="#status_code"><a href="#status_code"><code>status-code</code></a></a></li>
</ul>
<h4><a id="method_response_set_status_code"></a><code>[method]response.set-status-code: func</code></h4>
<p>Set the HTTP Status Code for the Response. Fails if the status-code
given is not a valid http status code.</p>
<h5>Params</h5>
<ul>
<li><a id="method_response_set_status_code.self"></a><code>self</code>: borrow&lt;<a href="#response"><a href="#response"><code>response</code></a></a>&gt;</li>
<li><a id="method_response_set_status_code.status_code"></a><a href="#status_code"><code>status-code</code></a>: <a href="#status_code"><a href="#status_code"><code>status-code</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_response_set_status_code.0"></a> result</li>
</ul>
<h4><a id="method_response_get_headers"></a><code>[method]response.get-headers: func</code></h4>
<p>Get the headers associated with the Response.</p>
<p>The returned <a href="#headers"><code>headers</code></a> resource is immutable: <code>set</code>, <code>append</code>, and
<code>delete</code> operations will fail with <code>header-error.immutable</code>.</p>
<h5>Params</h5>
<ul>
<li><a id="method_response_get_headers.self"></a><code>self</code>: borrow&lt;<a href="#response"><a href="#response"><code>response</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="method_response_get_headers.0"></a> own&lt;<a href="#headers"><a href="#headers"><code>headers</code></a></a>&gt;</li>
</ul>
<h4><a id="static_response_consume_body"></a><code>[static]response.consume-body: func</code></h4>
<p>Get body of the Response.</p>
<p>Stream returned by this method represents the contents of the body.
Once the stream is reported as closed, callers should await the returned
future to determine whether the body was received successfully.
The future will only resolve after the stream is reported as closed.</p>
<p>This function takes a <code>res</code> future as a parameter, which can be used to
communicate an error in handling of the response.</p>
<p>Note that function will move the <a href="#response"><code>response</code></a>, but references to headers
acquired from it previously will remain valid.</p>
<h5>Params</h5>
<ul>
<li><a id="static_response_consume_body.this"></a><code>this</code>: own&lt;<a href="#response"><a href="#response"><code>response</code></a></a>&gt;</li>
<li><a id="static_response_consume_body.res"></a><code>res</code>: future&lt;result&lt;_, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="static_response_consume_body.0"></a> (stream&lt;<code>u8</code>&gt;, future&lt;result&lt;option&lt;own&lt;<a href="#trailers"><a href="#trailers"><code>trailers</code></a></a>&gt;&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;&gt;)</li>
</ul>
<h2><a id="wasi_http_client_0_3_1"></a>Import interface wasi:http/client@0.3.1</h2>
<p>This interface defines an HTTP client for sending &quot;outgoing&quot; requests.</p>
<p>Most components are expected to import this interface to provide the
capability to send HTTP requests to arbitrary destinations on a network.</p>
<p>The type signature of <code>client.send</code> is the same as <code>handler.handle</code>. This
duplication is currently necessary because some Component Model tooling
(including WIT itself) is unable to represent a component importing two
instances of the same interface. A <code>client.send</code> import may be linked
directly to a <code>handler.handle</code> export to bypass the network.</p>
<hr />
<h3>Types</h3>
<h4><a id="request"></a><code>type request</code></h4>
<p><a href="#request"><a href="#request"><code>request</code></a></a></p>
<p>
<h4><a id="response"></a><code>type response</code></h4>
<p><a href="#response"><a href="#response"><code>response</code></a></a></p>
<p>
<h4><a id="error_code"></a><code>type error-code</code></h4>
<p><a href="#error_code"><a href="#error_code"><code>error-code</code></a></a></p>
<p>
<hr />
<h3>Functions</h3>
<h4><a id="send"></a><code>send: func</code></h4>
<p>This function may be used to either send an outgoing request over the
network or to forward it to another component.</p>
<h5>Params</h5>
<ul>
<li><a id="send.request"></a><a href="#request"><code>request</code></a>: own&lt;<a href="#request"><a href="#request"><code>request</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="send.0"></a> result&lt;own&lt;<a href="#response"><a href="#response"><code>response</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
<h2><a id="wasi_clocks_monotonic_clock_0_3_1"></a>Import interface wasi:clocks/monotonic-clock@0.3.1</h2>
<p>WASI Monotonic Clock is a clock API intended to let users measure elapsed
time.</p>
<p>It is intended to be portable at least between Unix-family platforms and
Windows.</p>
<p>A monotonic clock is a clock which has an unspecified initial value, and
successive reads of the clock will produce non-decreasing values.</p>
<hr />
<h3>Types</h3>
<h4><a id="duration"></a><code>type duration</code></h4>
<p><a href="#duration"><a href="#duration"><code>duration</code></a></a></p>
<p>
<h4><a id="mark"></a><code>type mark</code></h4>
<p><code>u64</code></p>
<p>A mark on a monotonic clock is a number of nanoseconds since an
unspecified initial value, and can only be compared to instances from
the same monotonic-clock.
<hr />
<h3>Functions</h3>
<h4><a id="now"></a><code>now: func</code></h4>
<p>Read the current value of the clock.</p>
<p>The clock is monotonic, therefore calling this function repeatedly will
produce a sequence of non-decreasing values.</p>
<p>For completeness, this function traps if it's not possible to represent
the value of the clock in a <a href="#mark"><code>mark</code></a>. Consequently, implementations
should ensure that the starting time is low enough to avoid the
possibility of overflow in practice.</p>
<h5>Return values</h5>
<ul>
<li><a id="now.0"></a> <a href="#mark"><a href="#mark"><code>mark</code></a></a></li>
</ul>
<h4><a id="get_resolution"></a><code>get-resolution: func</code></h4>
<p>Query the resolution of the clock. Returns the duration of time
corresponding to a clock tick.</p>
<h5>Return values</h5>
<ul>
<li><a id="get_resolution.0"></a> <a href="#duration"><a href="#duration"><code>duration</code></a></a></li>
</ul>
<h4><a id="wait_until"></a><code>wait-until: func</code></h4>
<p>Wait until the specified mark has occurred.</p>
<h5>Params</h5>
<ul>
<li><a id="wait_until.when"></a><code>when</code>: <a href="#mark"><a href="#mark"><code>mark</code></a></a></li>
</ul>
<h4><a id="wait_for"></a><code>wait-for: func</code></h4>
<p>Wait for the specified duration to elapse.</p>
<h5>Params</h5>
<ul>
<li><a id="wait_for.how_long"></a><code>how-long</code>: <a href="#duration"><a href="#duration"><code>duration</code></a></a></li>
</ul>
<h2><a id="wasi_clocks_system_clock_0_3_1"></a>Import interface wasi:clocks/system-clock@0.3.1</h2>
<p>WASI System Clock is a clock API intended to let users query the current
time. The clock is not necessarily monotonic as it may be reset.</p>
<p>It is intended to be portable at least between Unix-family platforms and
Windows.</p>
<p>External references may be reset, so this clock is not necessarily
monotonic, making it unsuitable for measuring elapsed time.</p>
<p>It is intended for reporting the current date and time for humans.</p>
<hr />
<h3>Types</h3>
<h4><a id="duration"></a><code>type duration</code></h4>
<p><a href="#duration"><a href="#duration"><code>duration</code></a></a></p>
<p>
<h4><a id="instant"></a><code>record instant</code></h4>
<p>An &quot;instant&quot;, or &quot;exact time&quot;, is a point in time without regard to any
time zone: just the time since a particular external reference point,
often called an &quot;epoch&quot;.</p>
<p>Here, the epoch is 1970-01-01T00:00:00Z, also known as
<a href="https://pubs.opengroup.org/onlinepubs/9699919799/xrat/V4_xbd_chap04.html#tag_21_04_16">POSIX's Seconds Since the Epoch</a>, also known as <a href="https://en.wikipedia.org/wiki/Unix_time">Unix Time</a>.</p>
<p>Note that even if the seconds field is negative, incrementing
nanoseconds always represents moving forwards in time.
For example, <code>{ -1 seconds, 999999999 nanoseconds }</code> represents the
instant one nanosecond before the epoch.
For more on various different ways to represent time, see
https://tc39.es/proposal-temporal/docs/timezone.html</p>
<h5>Record Fields</h5>
<ul>
<li><a id="instant.seconds"></a><code>seconds</code>: <code>s64</code></li>
<li><a id="instant.nanoseconds"></a><code>nanoseconds</code>: <code>u32</code></li>
</ul>
<hr />
<h3>Functions</h3>
<h4><a id="now"></a><code>now: func</code></h4>
<p>Read the current value of the clock.</p>
<p>This clock is not monotonic, therefore calling this function repeatedly
will not necessarily produce a sequence of non-decreasing values.</p>
<p>The nanoseconds field of the output is always less than 1000000000.</p>
<h5>Return values</h5>
<ul>
<li><a id="now.0"></a> <a href="#instant"><a href="#instant"><code>instant</code></a></a></li>
</ul>
<h4><a id="get_resolution"></a><code>get-resolution: func</code></h4>
<p>Query the resolution of the clock. Returns the smallest duration of time
that the implementation permits distinguishing.</p>
<h5>Return values</h5>
<ul>
<li><a id="get_resolution.0"></a> <a href="#duration"><a href="#duration"><code>duration</code></a></a></li>
</ul>
<h2><a id="wasi_clocks_timezone_0_3_1"></a>Import interface wasi:clocks/timezone@0.3.1</h2>
<hr />
<h3>Types</h3>
<h4><a id="instant"></a><code>type instant</code></h4>
<p><a href="#instant"><a href="#instant"><code>instant</code></a></a></p>
<p>
<hr />
<h3>Functions</h3>
<h4><a id="iana_id"></a><code>iana-id: func</code></h4>
<p>Return the IANA identifier of the currently configured timezone. This
should be an identifier from the IANA Time Zone Database.</p>
<p>For displaying to a user, the identifier should be converted into a
localized name by means of an internationalization API.</p>
<p>If the implementation does not expose an actual timezone, or is unable
to provide mappings from times to deltas between the configured timezone
and UTC, or determining the current timezone fails, or the timezone does
not have an IANA identifier, this returns nothing.</p>
<h5>Return values</h5>
<ul>
<li><a id="iana_id.0"></a> option&lt;<code>string</code>&gt;</li>
</ul>
<h4><a id="utc_offset"></a><code>utc-offset: func</code></h4>
<p>The number of nanoseconds difference between UTC time and the local
time of the currently configured timezone, at the exact time of
<a href="#instant"><code>instant</code></a>.</p>
<p>The magnitude of the returned value will always be less than
86,400,000,000,000 which is the number of nanoseconds in a day
(24<em>60</em>60*1e9).</p>
<p>If the implementation does not expose an actual timezone, or is unable
to provide mappings from times to deltas between the configured timezone
and UTC, or determining the current timezone fails, this returns
nothing.</p>
<h5>Params</h5>
<ul>
<li><a id="utc_offset.when"></a><code>when</code>: <a href="#instant"><a href="#instant"><code>instant</code></a></a></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="utc_offset.0"></a> option&lt;<code>s64</code>&gt;</li>
</ul>
<h4><a id="to_debug_string"></a><code>to-debug-string: func</code></h4>
<p>Returns a string that is suitable to assist humans in debugging whether
any timezone is available, and if so, which. This may be the same string
as <a href="#iana_id"><code>iana-id</code></a>, or a formatted representation of the UTC offset such as
<code>-04:00</code>, or something else.</p>
<p>WARNING: The returned string should not be consumed mechanically! It may
change across platforms, hosts, or other implementation details. Parsing
this string is a major platform-compatibility hazard.</p>
<h5>Return values</h5>
<ul>
<li><a id="to_debug_string.0"></a> <code>string</code></li>
</ul>
<h2><a id="wasi_random_random_0_3_1"></a>Import interface wasi:random/random@0.3.1</h2>
<p>WASI Random is a random data API.</p>
<p>It is intended to be portable at least between Unix-family platforms and
Windows.</p>
<hr />
<h3>Functions</h3>
<h4><a id="get_random_bytes"></a><code>get-random-bytes: func</code></h4>
<p>Return up to <code>max-len</code> cryptographically-secure random or pseudo-random
bytes.</p>
<p>This function must produce data at least as cryptographically secure and
fast as an adequately seeded cryptographically-secure pseudo-random
number generator (CSPRNG). It must not block, from the perspective of
the calling program, under any circumstances, including on the first
request and on requests for numbers of bytes. The returned data must
always be unpredictable.</p>
<p>Implementations MAY return fewer bytes than requested (a short read).
Callers that require exactly <code>max-len</code> bytes MUST call this function in
a loop until the desired number of bytes has been accumulated.
Implementations MUST return at least 1 byte when <code>max-len</code> is greater
than zero. When <code>max-len</code> is zero, implementations MUST return an empty
list without trapping.</p>
<p>This function must always return fresh data. Deterministic environments
must omit this function, rather than implementing it with deterministic
data.</p>
<h5>Params</h5>
<ul>
<li><a id="get_random_bytes.max_len"></a><code>max-len</code>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="get_random_bytes.0"></a> list&lt;<code>u8</code>&gt;</li>
</ul>
<h4><a id="get_random_u64"></a><code>get-random-u64: func</code></h4>
<p>Return a cryptographically-secure random or pseudo-random <code>u64</code> value.</p>
<p>This function returns the same type of data as <a href="#get_random_bytes"><code>get-random-bytes</code></a>,
represented as a <code>u64</code>.</p>
<h5>Return values</h5>
<ul>
<li><a id="get_random_u64.0"></a> <code>u64</code></li>
</ul>
<h2><a id="wasi_random_insecure_0_3_1"></a>Import interface wasi:random/insecure@0.3.1</h2>
<p>The insecure interface for insecure pseudo-random numbers.</p>
<p>It is intended to be portable at least between Unix-family platforms and
Windows.</p>
<hr />
<h3>Functions</h3>
<h4><a id="get_insecure_random_bytes"></a><code>get-insecure-random-bytes: func</code></h4>
<p>Return up to <code>max-len</code> insecure pseudo-random bytes.</p>
<p>This function is not cryptographically secure. Do not use it for
anything related to security.</p>
<p>There are no requirements on the values of the returned bytes, however
implementations are encouraged to return evenly distributed values with
a long period.</p>
<p>Implementations MAY return fewer bytes than requested (a short read).
Callers that require exactly <code>max-len</code> bytes MUST call this function in
a loop until the desired number of bytes has been accumulated.
Implementations MUST return at least 1 byte when <code>max-len</code> is greater
than zero. When <code>max-len</code> is zero, implementations MUST return an empty
list without trapping.</p>
<h5>Params</h5>
<ul>
<li><a id="get_insecure_random_bytes.max_len"></a><code>max-len</code>: <code>u64</code></li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="get_insecure_random_bytes.0"></a> list&lt;<code>u8</code>&gt;</li>
</ul>
<h4><a id="get_insecure_random_u64"></a><code>get-insecure-random-u64: func</code></h4>
<p>Return an insecure pseudo-random <code>u64</code> value.</p>
<p>This function returns the same type of pseudo-random data as
<a href="#get_insecure_random_bytes"><code>get-insecure-random-bytes</code></a>, represented as a <code>u64</code>.</p>
<h5>Return values</h5>
<ul>
<li><a id="get_insecure_random_u64.0"></a> <code>u64</code></li>
</ul>
<h2><a id="wasi_random_insecure_seed_0_3_1"></a>Import interface wasi:random/insecure-seed@0.3.1</h2>
<p>The insecure-seed interface for seeding hash-map DoS resistance.</p>
<p>It is intended to be portable at least between Unix-family platforms and
Windows.</p>
<hr />
<h3>Functions</h3>
<h4><a id="get_insecure_seed"></a><code>get-insecure-seed: func</code></h4>
<p>Return a 128-bit value that may contain a pseudo-random value.</p>
<p>The returned value is not required to be computed from a CSPRNG, and may
even be entirely deterministic. Host implementations are encouraged to
provide pseudo-random values to any program exposed to
attacker-controlled content, to enable DoS protection built into many
languages' hash-map implementations.</p>
<p>This function is intended to only be called once, by a source language
to initialize Denial Of Service (DoS) protection in its hash-map
implementation.</p>
<h1>Expected future evolution</h1>
<p>This will likely be changed to a value import, to prevent it from being
called multiple times and potentially used for purposes other than DoS
protection.</p>
<h5>Return values</h5>
<ul>
<li><a id="get_insecure_seed.0"></a> (<code>u64</code>, <code>u64</code>)</li>
</ul>
<h2><a id="wasi_http_handler_0_3_1"></a>Export interface wasi:http/handler@0.3.1</h2>
<hr />
<h3>Types</h3>
<h4><a id="request"></a><code>type request</code></h4>
<p><a href="#request"><a href="#request"><code>request</code></a></a></p>
<p>
<h4><a id="response"></a><code>type response</code></h4>
<p><a href="#response"><a href="#response"><code>response</code></a></a></p>
<p>
<h4><a id="error_code"></a><code>type error-code</code></h4>
<p><a href="#error_code"><a href="#error_code"><code>error-code</code></a></a></p>
<p>
<hr />
<h3>Functions</h3>
<h4><a id="handle"></a><code>handle: func</code></h4>
<p>This function may be called with either an incoming request read from the
network or a request synthesized or forwarded by another component.</p>
<h5>Params</h5>
<ul>
<li><a id="handle.request"></a><a href="#request"><code>request</code></a>: own&lt;<a href="#request"><a href="#request"><code>request</code></a></a>&gt;</li>
</ul>
<h5>Return values</h5>
<ul>
<li><a id="handle.0"></a> result&lt;own&lt;<a href="#response"><a href="#response"><code>response</code></a></a>&gt;, <a href="#error_code"><a href="#error_code"><code>error-code</code></a></a>&gt;</li>
</ul>
