use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: u32,
    id: u32,
    title: String,
    completed: bool,
}

async fn fetch_todo_api(url: &str) -> Result<Todo, Error> {
    let response = reqwest::get(url).await?;
    response.json::<Todo>().await
}

fn todo_details(todo: &Todo) -> String {
    format!(
        "Todo:\nUserId: {}\nId: {}\nTitle: {}\nCompleted: {}",
        todo.user_id, todo.id, todo.title, todo.completed
    )
}

#[tokio::main]
async fn main() {
    if let Some(todo_id) = std::env::args().nth(1) {
        let base_url = "https://jsonplaceholder.typicode.com/todos/";
        let url = format!("{}{}", base_url, todo_id);
        match fetch_todo_api(&url).await {
            Ok(todo) => println!("{}", todo_details(&todo)),
            Err(err) => eprintln!("Error: {}", err),
        }
    } else {
        eprintln!("Error: Todo ID not provided");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_todo_api() {
        let mut server = mockito::Server::new_async().await;
        let path = "/todos/1";
        let json_body = r#"{"userId": 1, "id": 1, "title": "delectus aut autem", "completed": false}"#;

        let mock = server.mock("GET", path)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json_body)
            .create_async().await;

        let url = server.url() + path;
        let todo: Todo = fetch_todo_api(&url).await.unwrap();

        assert_eq!(todo.user_id, 1);
        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "delectus aut autem");
        assert!(!todo.completed);

        mock.assert_async().await;
    }

    #[test]
    fn test_todo_details() {
        let todo = Todo {
            user_id: 1,
            id: 2,
            title: "quis ut nam facilis et officia qui".to_string(),
            completed: false,
        };

        let expected = "Todo:\nUserId: 1\nId: 2\nTitle: quis ut nam facilis et officia qui\nCompleted: false";

        assert_eq!(todo_details(&todo), expected);
    }
}
