use warp::Filter;

#[tokio::main]
async fn main() {
    // 創建一個路由
    let hello = warp::path::end()
        .map(|| "hello world");

    // 啟動服務
    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test() {
        let client = reqwest::Client::new();
        let response = client.get("http://localhost:3030")
            .send()
            .await
            .unwrap();

        assert!(response.status().is_success());
        assert_eq!("hello world", response.text().await.unwrap());
    }
}
