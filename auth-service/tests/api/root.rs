use crate::helpers::TestApp;

#[tokio::test]
async fn root_returns_auth_ui() {
    // boot the TestApp (server & client)
    let app = TestApp::new().await;

    // GET-Request the '/' (root) of the auth service (server) and obtain the Response
    let response = app.get_root().await;

    // validate that the Response from the server matches our expectations
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}