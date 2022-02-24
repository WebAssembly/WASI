# [Proposal Template] API

[This document contains the actual specification. It should be written in the WIT interface definition format. You can find more documentation on the WIT syntax (coming soon!).]

[Note that all comments inside of WIT code blocks will be included in the developer facing documentation for language bindings generated using this WIT file. If there is additional information that needs to be communicated to implementers of the API, then these should be captured in text directly below the code block.]

[If you want to include examples of the API in use, these should be in the README and linked to from this file.]

## api_type_one

```wit
/// Short description
///
/// Explanation for developers using the API.
record api-type-one {
    property1: u64,
    property2: string,
}
```

More rigorous specification details for the implementer go here, if needed.

## api_function_one

```wit
/// Short description
///
/// Explanation for developers using the API.
api-function-one: function() -> api-type-one
```

If needed, this would explain what a compliant implementation MUST do, such as never returning an earlier result from a later call.
