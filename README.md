# Project Name

This project utilizes Diesel for migrations and provides an endpoint accessible via Basic Authorization.

## Getting Started

1. Clone this repository:
    ```bash
    $ git clone https://github.com/4prndz/rocket-posts-endpoint
    ```

2. Run migrations:
    ```bash
    $ diesel migration run --database-url ./database.sqlite
    ```

3. Run the project:
    ```bash
    $ cargo run
    ```

## Accessing the Endpoint

To access the endpoint, you need to use Basic Authorization by providing your login credentials encoded in base64.

Example using cURL:
```bash
$ curl -Ss http://localhost:8000/posts -H 'Authorization: Basic NHBybmR6OmtvdWljaGkNCg=='
