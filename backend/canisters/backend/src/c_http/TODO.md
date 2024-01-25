TODO
1. save all main-net data(payload) to dropbox , json or COBR ok
    1. get dropbox user access token.
        use for temporary: 
    sl.BuJkQn6_xnatK6KB9t5YfJXNQxH4dMm6mAMi2uutjpuKLulYXrstBm7k5qcggSc-Wc9_DTw2t7-csBPj5bXffLxtEEOGdUSlvhzgq3HbZcgzLO4eVmvh-IKEWhZyRrVC-2yskjqakhg_
    2. save data to dropbox  http format:
        ```
        https://content.dropboxapi.com/2/files/upload
        POST
        curl -X POST https://content.dropboxapi.com/2/files/upload \
    --header "Authorization: Bearer <get access token>" \
    --header "Dropbox-API-Arg: {\"autorename\":false,\"mode\":\"add\",\"mute\":false,\"path\":\"/Homework/math/Matrices.txt\",\"strict_conflict\":false}" \
    --header "Content-Type: application/octet-stream" \
    --data-binary @local_file.txt

        {
            "autorename": false,
            "mode": "add",
            "mute": false,
            "path": "/Homework/math/Matrices.txt",
            "strict_conflict": false
        }
        ```
        construct above into rust code in canister http calls.

        use this lib or ic-http lib 
        https://github.com/omnia-network/ic-http-proxy/blob/main/examples/rust/basic/src/basic_backend/src/lib.rs



2. retrive data from dropbox http 
    step1. gen a file request url: do in cmd: 
    ```
    curl -X POST https://api.dropboxapi.com/2/file_requests/get \
        --header "Authorization: Bearer <get access token>" \
        --header "Content-Type: application/json" \
        --data "{\"id\":\"oaCAVmEyrqYnkZX9955Y\"}"
    ```
    this return a url include .
    step2. use that ret-url in canister and store  the return file in mem.


3. deserialize from file.




construct this rust instance for me :

pub struct HttpRequest {
    pub url: String,
    pub method: HttpMethod,
    pub headers: Vec<HttpHeader>,
    pub body: Option<Vec<u8>>,
}



        ```
        https://content.dropboxapi.com/2/files/upload
        POST
        curl -X POST https://content.dropboxapi.com/2/files/upload \
    --header "Authorization: Bearer <get access token>" \
    --header "Dropbox-API-Arg: {\"autorename\":false,\"mode\":\"add\",\"mute\":false,\"path\":\"/Homework/math/Matrices.txt\",\"strict_conflict\":false}" \
    --header "Content-Type: application/octet-stream" \
    --data-binary @local_file.txt

        {
            "autorename": false,
            "mode": "add",
            "mute": false,
            "path": "/Homework/math/Matrices.txt",
            "strict_conflict": false
        }
        ```



write  me a rust function to expand httprequest type into canisterhttprequestArgument 


pub struct HttpRequest {
    pub url: String,
    pub method: HttpMethod,
    pub headers: Vec<HttpHeader>,
    pub body: Option<Vec<u8>>,
}

pub struct CanisterHttpRequestArgument {
    /// The requested URL.
    pub url: String,
    /// The maximal size of the response in bytes. If None, 2MiB will be the limit.
    /// This value affects the cost of the http request and it is highly recommended
    /// to set it as low as possible to avoid unnecessary extra costs.
    /// See also the [pricing section of HTTP outcalls documentation](https://internetcomputer.org/docs/current/developer-docs/integrations/http_requests/http_requests-how-it-works#pricing).
    pub max_response_bytes: Option<u64>,
    /// The method of HTTP request.
    pub method: HttpMethod,
    /// List of HTTP request headers and their corresponding values.
    pub headers: Vec<HttpHeader>,
    /// Optionally provide request body.
    pub body: Option<Vec<u8>>,
    /// Name of the transform function which is `func (transform_args) -> (http_response) query`.
    /// Set to `None` if you are using `http_request_with` or `http_request_with_cycles_with`.
    pub transform: Option<TransformContext>,
}

