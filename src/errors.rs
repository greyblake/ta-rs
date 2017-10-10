error_chain! {
    errors {
        InvalidParameter { description("invalid parameter") }
        DataItemIncomplete { description("data item is incomplete") }
        DataItemInvalid { description("data item is invalid") }
    }
}
