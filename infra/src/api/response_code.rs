#[derive(Debug,Clone)]
pub enum CodeEnum {
    Success,

    NotFound,
    FileInvaild,
    ServiceError,
    Other,

    Ban,
    Unauthorized,
    BadRequest,
    Forbidden,
    MethodNotAllowed,
}

pub fn get_code(code: CodeEnum) -> u16 {
    match code {
        CodeEnum::Success => 0,
        CodeEnum::Other => 1,
        CodeEnum::ServiceError => 2,
        CodeEnum::NotFound => 3,
        CodeEnum::FileInvaild=>1000,
        CodeEnum::Ban => 2000,
        CodeEnum::Unauthorized => 2001,

        CodeEnum::BadRequest => 400,
        CodeEnum::Forbidden => 403,
        CodeEnum::MethodNotAllowed => 405,
    }
}
