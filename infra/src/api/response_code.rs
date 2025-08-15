#[derive(Debug,Clone)]
pub enum CodeEnum {
    Success,

    NotFound,
    BadRequest,
    Unauthorized,
    Forbidden,
    MethodNotAllowed,
    FileInvaild,

    ServiceError,
    Other,

    Ban,
}

pub fn get_code(code: CodeEnum) -> u16 {
    match code {
        CodeEnum::Success => 0,
        CodeEnum::Other => 1,
        CodeEnum::FileInvaild=>1000,

        CodeEnum::Ban => 500,
        CodeEnum::ServiceError => 500,

        CodeEnum::NotFound => 404,
        CodeEnum::BadRequest => 400,
        CodeEnum::Unauthorized => 401,
        CodeEnum::Forbidden => 403,
        CodeEnum::MethodNotAllowed => 405,
    }
}
