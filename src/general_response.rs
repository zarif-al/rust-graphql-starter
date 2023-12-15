use serde::Serialize;

///  This struct should be used in situations where the application
/// is not able to return the regular response.
/// For example:
/// Lets say the app failed to create a user so it can't send the
/// new user as a response. In that case it should return a general response
/// containing the error message.
#[derive(Serialize)]
pub struct GeneralResponse {
    pub code: u8,
    pub message: Option<String>,
    pub error: Option<String>,
}