# [Proposal Template] API

[This document contains the actual specification. It should be written in the WAI interface definition format. You can find more documentation on the WAI syntax (coming soon!).]

[Note that all comments inside of WAI code blocks will be included in the developer facing documentation for language bindings generated using this WAI file. If there is additional information that needs to be communicated to implementers of the API, then these should be captured in text directly below the code block.]

[If you want to include examples of the API in use, these should be in the README and linked to from this file.]

## api_type_one

```wai
/// Short description
///
/// Explanation for developers using the API.
record api_type_one {
    property_1: u64,
    property_2: string,
}
```

More rigorous specification details for the implementer go here, if needed.

## api_function_one

```wai
/// Short description
///
/// Explanation for developers using the API.
api_function_one: function() -> api_type_one
```

If needed, this would explain what a compliant implementation MUST do, such as never returning an earlier result from a later call.
