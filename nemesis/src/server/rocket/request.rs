//! Nemesis [`Request`](crate::request::Request) implementation using
//! [Rocket](rocket).

use rocket::http::uri::Origin;
use rocket::http::{Method, Status};
use rocket::request::{self as rocketreq, FromRequest};
use rocket::Outcome;

use crate::request as nemesisreq;
use crate::Error;

pub struct Request<'a> {
    method: Method,
    origin: &'a Origin<'a>,
    base: &'a str,
}

impl<'a> nemesisreq::Request for Request<'a> {
    fn origin(&self) -> String {
        self.origin.path().to_owned()
    }

    fn http_version(&self) -> Option<String> {
        // Rocket does not expose HTTP version
        // https://github.com/SergioBenitez/Rocket/issues/1019
        None
    }

    fn request_method(&self) -> String {
        format!("{}", self.method)
    }

    fn script_name(&self) -> String {
        self.base.to_owned()
    }

    fn path_info(&self) -> String {
        let uri = self.origin.path();
        // This &str slice is safe because URIs are guaranteed to be ASCII
        // (single byte characters).
        uri[self.base.len()..].to_owned()
    }

    fn query_string(&self) -> String {
        self.origin.query().unwrap_or_default().to_owned()
    }

    fn server_name(&self) -> String {
        // TODO: implement using config fairing, see GH-61.
        "localhost".to_owned()
    }

    fn server_port(&self) -> u16 {
        // TODO: implement using config fairing, see GH-61.
        8000
    }

    fn url_scheme(&self) -> String {
        // TODO: implement using config fairing, see GH-61.
        "http".to_owned()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Request<'a> {
    type Error = Error;

    fn from_request(request: &'a rocket::Request<'r>) -> rocketreq::Outcome<Self, Self::Error> {
        if let Some(route) = request.route() {
            Outcome::Success(Request {
                method: request.method(),
                origin: request.uri(),
                base: route.base(),
            })
        } else {
            Outcome::Failure((Status::NotFound, Error::NoRoute))
        }
    }
}
