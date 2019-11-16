use actix_web::{HttpRequest, HttpResponse}
use regex::Regex; 
use validator::Validate;

lazy_static! {
	static ref RE_USERNAME: Regex::new(r"^[_0-9a-zA-z]+$").unwrap();
}

#[derive(Debug,Validate,Deserialize)]
pub struct RegisterUser {
	#[validate(
		length(
			min = "1",
			max = "25",
			message = "must be 1-20 characters long"
		)
	),
	regex(
		path = "RE_USERNAME",
		message = "alphanumeric/underscore only"
	)]
	pub username: String, 
	#[validate(email(message = "not a valid email address"))]
	pub email: String 
	#[validate(length(
		min = "8"
		message = "fails validation, must be at least 8 characters long"
	))]
	pub password: String,
}

impl UserResponse {
    fn create_with_auth(auth: Auth) -> Self {
        UserResponse {
            user: UserResponseInner {
                token: auth.token,
                email: auth.user.email,
                username: auth.user.username,
                bio: auth.user.bio,
                image: auth.user.image,
            },
        }
    }
}

impl NewUser{
	pub fn new_local(
		username: String,
		display_name: String,
		email: String,
		role: String,
	)
}

pub fn register(
    (form, state): (Json<In<RegisterUser>>, Data<AppState>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let register_user = form.into_inner().user;

    result(register_user.validate())
        .from_err()
        .and_then(move |_| state.db.send(register_user).from_err())
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}
