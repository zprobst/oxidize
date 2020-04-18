//! Defines the standard elements of the routing library
//! that is need for main http operations that is needed.

use crate::views::View;

use regex::{Captures, Regex};
use std::collections::HashMap;

/// Stores a Request Object that is taken from HTTP.
pub struct Request {
    /// Stores the path for a given request removing
    /// the protocol and host name and leaving only
    /// the directory path. "/api/v2/posts/12345"
    path: String,

    /// Defines all of the request headers that are
    /// supplied in lower case form for the names and
    /// in exact form for the value.
    headers: HashMap<String, String>,

    /// The body of the request if any.
    body: Option<String>,

    /// The method of the request in HTTP. Something
    /// like GET, POST, DELETE, etc.
    method: String,
}

/// Defines the response that is a completed response from
/// the http logic. The response object will be built after
/// calling into the view.
pub struct Response {
    /// Defines the status code for the requrst. 200, 401.
    status: usize,

    /// The status string to send with the code.
    status_code: String,

    /// The body of the response if anything.
    body: Option<String>,

    /// Response headers that is to be returned.
    headers: HashMap<String, String>,
}

impl Response {
    /// Creates a new request with the base not found request.
    pub fn new() -> Response {
        Response {
            status: 404,
            status_code: "Not found".to_string(),
            body: Option::None,
            headers: HashMap::new(),
        }
    }
}

/// Defines a singular route that is a part of the router
/// system. Each route needs to store the methods and the
/// pattern to match against the url.
pub struct Route {
    /// Defines the pattern in the wrap import pattern.
    /// Patterns are requires to start with the preceding
    /// slash. Followed by the path segment(s) that the
    /// path should match to.
    pattern: String,

    /// Stores the number of matches that need to be made
    /// against the pattern. We need this to compare the
    /// number of matches gotten to the capture and ensure
    /// they are the same. If not, then this route will not
    /// applied to given request.
    required_matches: usize,

    /// The name of the matches to apply to. If a result
    /// match is found, we use the names to get all of the
    /// result names that are to be loaded from the regex
    /// library.
    match_names: Vec<String>,

    /// Defines the pattern as a regex that can be
    /// used to match the route against the path
    /// stored in the request.
    compiled_pattern: String,

    /// Defines the methods the route applies to. If the
    /// list is not defined in the yaml, it will asummed
    /// to be only "GET".
    methods: Vec<String>,

    /// The pypath to the view that should be called into
    /// to render the application. This is defined in the
    /// yaml file and passed into here.
    view: String,

    /// The result of last parsing of all path parameters.
    /// When applies_to_request is called and there is a success,
    /// then `parsedPathParameters` will get populated with
    /// the processing results of matching the groups against
    /// the request's path.
    parsed_path_parameters: HashMap<String, Option<String>>,
}

impl Route {
    /// Creates a new route from the configuration object that is
    /// defined in the yaml file or other potential configuration
    /// source.
    ///
    /// The following short cut patterns are replaced by this method
    /// when passing a pattern. In addition, beginning anchor
    /// is attatched to the regex when taken from the pattern.
    ///
    /// Additionally any "/" or other regex control characters are
    /// escaped.
    ///
    /// "/api/v2/<id:int>" -> "^/api/v2/(?<id>[0-9]+)"
    /// "/api/v2/<id:slug>" -> "^/api/v2/(?<id>[a-z0-9]+(?:-[a-z0-9]+)*)"
    /// "/api/v2/<id:uuid>" -> "^/api/v2/(?<id>[0-9a-fA-F]{8}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{4}\-[0-9a-fA-F]{12})
    ///
    /// If these do not support your use case, you are able to put your own named groups or regex
    /// expressions. This expressions are all passed into the view as key word arguments
    /// in views implemented in python.
    /// 
    pub fn new(pattern: &str, view: &str, methods: Option<Vec<&str>>) -> Route {
        // Scan pattern for < variable_name : word >
        // Look up a pattern for word.
        let mut required_matches = 0;
        let mut match_names = Vec::new();
        let re = Regex::new("<(?<name>[a-zA-Z]+):(?<type>[a-z]+)>").unwrap();

        // Place the name in the pattern for each pattern.
        // Remember each name found.
        // Remember how many are found and required (we are going to say all for now)
        let compiled_pattern = re.replace_all(&pattern, |captures: &Captures| {
            let expected_type = captures.name("type").unwrap().as_str();
            let name = captures.name("name").unwrap().as_str();

            let type_pattern = match expected_type {
                "int" => "[0-9]+",
                "slug" => "[a-z0-9]+(?:-[a-z0-9]+)*",
                "uuid" => "[0-9a-fA-F]{8}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{4}\\-[0-9a-fA-F]{12}",
                _ => "[0-9]+"
            };

            required_matches += 1;
            match_names.push(name.to_string());
            return format!("(?<{}>{})", name, type_pattern);
        });

        Route {
            match_names,
            required_matches,
            view: view.to_string(),
            pattern: pattern.to_string(),
            compiled_pattern: format!("^{}\\/?$", compiled_pattern.replace("/", "\\/")),
            methods: methods
                .unwrap_or_else(|| vec!["GET"])
                .iter()
                .map(|s| s.to_string())
                .collect(),
            parsed_path_parameters: HashMap::new(),
        }
    }

    /// Saves pattern matches found during `applies_to_request`
    /// to the `parsedPathParameters` map in the struct.
    /// 
    fn save_pattern_matches(&mut self, captures: Captures) {
        let mut parsed = HashMap::new();

        for group_name in self.match_names.clone() {
            let value = captures.name(&group_name).map(|c| c.as_str());
            parsed.insert(group_name, value.map(|v| v.to_string()));
        }

        self.parsed_path_parameters = parsed;
    }

    /// Checks whether or a request is to be handled by
    /// this route. Returns true if request is handled by this
    /// route; false otherwise.
    /// 
    pub fn applies_to_request(&mut self, request: &Request) -> bool {
        let re = Regex::new(&self.compiled_pattern).unwrap();
        match re.captures(&request.path) {
            Some(captures) => {
                // if we have all the matches we need we can safe the
                // new version of `parsedPathParameters` to the
                // struct.
                if captures.len() == self.required_matches + 1
                    && self.methods.contains(&request.method.to_string())
                {
                    self.save_pattern_matches(captures);
                    return true;
                }
                false
            }
            None => false,
        }
    }

    /// Calls into the python view and calls the render function appropriate
    /// for the method and get its response.
    /// 
    pub fn to_response(&self, request: &Request) -> Response {
        let view = View::new(&self.view);
        return view.get_response(request, &self.parsed_path_parameters);
    }
}
