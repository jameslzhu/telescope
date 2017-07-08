// error_chain! {
//     errors {
//         Eof
//         Exit(status: i32)
//     }
// }

#[derive(Debug, error_chain)]
pub enum ErrorKind {
    Msg(String),

    #[error_chain(custom)]
    Eof,

    #[error_chain(custom)]
    Exit(i32),
}