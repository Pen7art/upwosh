mod endpoints_test {

    use reqwest::{header::CONTENT_TYPE, Body, Client};
    use serde::Serialize;

    const BASE_URL: &str = "http://localhost:8000/";

    #[tokio::test]
    async fn scrape_data() {
        let path = format!("{}", BASE_URL);
        do_get(&path, "test").await;
    }

    // async fn do_multipart_post(path: &str, file_path: &str, file_name: String) {
    //     let client = Client::new();

    //     let file = tokio::fs::File::open(file_path).await.unwrap();
    //     let stream = FramedRead::new(file, BytesCodec::new());
    //     let file_body = Body::wrap_stream(stream);

    //     let some_file = multipart::Part::stream(file_body).file_name(file_name.clone());
    //     let form = multipart::Form::new().part("file", some_file);

    //     let reqwest_response = client.post(path).multipart(form).send().await.unwrap();

    //     println!("=== Response for POST {}", path);
    //     println!(
    //         "=> Status: {} {}",
    //         reqwest_response.status(),
    //         reqwest_response.status().canonical_reason().unwrap_or("")
    //     );
    //     println!("=> Headers: {:#?}", reqwest_response.headers());
    //     println!("=> Body: {:#?}", reqwest_response.text().await);
    //     println!("===");
    // }

    async fn do_post<T: Serialize>(path: &str, auth_header: &str, body: T) {
        let client = Client::new();
        let reqwest_response = client
            .post(path)
            .header(reqwest::header::AUTHORIZATION, auth_header)
            .json(&body)
            .send()
            .await
            .unwrap();
        println!("=== Response for POST {}", path);
        println!(
            "=> Status: {} {}",
            reqwest_response.status(),
            reqwest_response.status().canonical_reason().unwrap_or("")
        );
        println!("=> Headers: {:#?}", reqwest_response.headers());
        println!("=> Body: {:#?}", reqwest_response.text().await);
        println!("===");
    }

    async fn do_post_raw(path: &str, auth_header: &str, body: Body) {
        let client = Client::new();
        let reqwest_response = client
            .post(path)
            .header(reqwest::header::AUTHORIZATION, auth_header)
            .header(CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .await
            .unwrap();
        println!("=== Response for POST {}", path);
        println!(
            "=> Status: {} {}",
            reqwest_response.status(),
            reqwest_response.status().canonical_reason().unwrap_or("")
        );
        println!("=> Headers: {:#?}", reqwest_response.headers());
        println!("=> Body: {:#?}", reqwest_response.text().await);
        println!("===");
    }

    async fn do_get(path: &str, auth_header: &str) {
        let client = Client::new();
        let reqwest_response = client
            .get(path)
            .header(reqwest::header::AUTHORIZATION, auth_header)
            .send()
            .await;
        if reqwest_response.is_err() {
            println!("Request error for GET {}", path);
            println!("Error: {:?}", reqwest_response.err());
            return;
        }

        let reqwest_response = reqwest_response.unwrap();
        println!("=== Response for GET {}", path);
        println!(
            "=> Status: {} {}",
            reqwest_response.status(),
            reqwest_response.status().canonical_reason().unwrap_or("")
        );
        println!("=> Headers: {:#?}", reqwest_response.headers());
        println!("=> Body: {:#?}", reqwest_response.text().await);
        println!("===");
    }

    async fn do_delete(path: &str, auth_header: &str) {
        let client = Client::new();
        let reqwest_response = client
            .delete(path)
            .header(reqwest::header::AUTHORIZATION, auth_header)
            .send()
            .await;
        if reqwest_response.is_err() {
            println!("Request error for DELETE {}", path);
            println!("Error: {:?}", reqwest_response.err());
            return;
        }

        let reqwest_response = reqwest_response.unwrap();
        println!("=== Response for DELETE {}", path);
        println!(
            "=> Status: {} {}",
            reqwest_response.status(),
            reqwest_response.status().canonical_reason().unwrap_or("")
        );
        println!("=> Headers: {:#?}", reqwest_response.headers());
        println!("=> Body: {:#?}", reqwest_response.text().await);
        println!("===");
    }
}
