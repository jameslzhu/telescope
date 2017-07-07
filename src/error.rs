error_chain! {
    errors {
        Exit(status: i32)
        Eof
    }
}
