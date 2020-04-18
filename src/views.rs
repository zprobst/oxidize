use crate::routing::{Request, Response};
use std::collections::HashMap;

pub struct View<'a> {
    py_path: &'a str,
}

impl<'a> View<'a> {
    pub fn new(py_path: &'a str) -> View<'a> {
        View { py_path: py_path }
    }

    pub fn get_response(
        &self,
        request: &Request,
        params: &HashMap<String, Option<String>>,
    ) -> Response {
        return Response::new();
    }
}
