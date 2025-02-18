### **Route: Create Password**

#### **Description**

This route allows clients to create and store a new password entry in the database. The password is encrypted and validated before being saved.

#### **Endpoint**

- **Method:** `POST`
- **Path:** `/passwords`

#### **Request Body**

The request body should be a JSON object with the following fields:

| Field        | Type            | Description                                                                |
| ------------ | --------------- | -------------------------------------------------------------------------- |
| `service`    | `String`        | The name of the service or application associated with the password.       |
| `nonce`      | `String`        | A unique nonce used for encryption. Must be valid as per `is_valid_nonce`. |
| `cipher`     | `String`        | The encrypted password. Must be valid as per `is_valid_cipher`.            |
| `created_at` | `DateTime<Utc>` | The timestamp when the password was created.                               |
| `updated_at` | `DateTime<Utc>` | The timestamp when the password was last updated.                          |

**Example Request Body:**

```json
{
  "service": "example.com",
  "nonce": "valid-nonce-123",
  "cipher": "encrypted-password-456",
  "created_at": "2023-10-01T12:00:00Z",
  "updated_at": "2023-10-01T12:00:00Z"
}
```

#### **Response**

- **Success Response:**

  - **Status Code:** `200 OK`
  - **Body:** A JSON object with a success message.
    ```json
    {
      "message": "Password saved successfully"
    }
    ```

- **Error Responses:**

  - **Status Code:** `400 Bad Request`

    - **Body:** A JSON object with an error message if the `nonce` or `cipher` is invalid.
      ```json
      {
        "message": "Invalid nonce provided."
      }
      ```
      or
      ```json
      {
        "message": "Invalid cipher provided."
      }
      ```

  - **Status Code:** `500 Internal Server Error`
    - **Body:** A JSON object with an error message if the database operation fails.
      ```json
      {
        "message": "Database error: <error details>"
      }
      ```

#### **Validation**

1. **Nonce Validation:**

   - The `nonce` is validated using the `is_valid_nonce` function. If the nonce is invalid, the route returns a `400 Bad Request` error.

2. **Cipher Validation:**
   - The `cipher` is validated using the `is_valid_cipher` function. If the cipher is invalid, the route returns a `400 Bad Request` error.

#### **Database Interaction**

- The password is saved to the database using the `save` method of the `Database` struct.
- The `Database` struct is provided via Axum's `State` extractor.

#### **Example Usage**

```bash
curl -X POST http://localhost:3000/passwords \
-H "Content-Type: application/json" \
-d '{
  "service": "example.com",
  "nonce": "valid-nonce-123",
  "cipher": "encrypted-password-456",
  "created_at": "2023-10-01T12:00:00Z",
  "updated_at": "2023-10-01T12:00:00Z"
}'
```

**Successful Response:**

```json
{
  "message": "Password saved successfully"
}
```

**Error Response (Invalid Nonce):**

```json
{
  "message": "Invalid nonce provided."
}
```

### **Route: Delete Password**

#### **Description**

This route allows clients to delete a stored password entry from the database using a unique password ID. The request must include a valid password ID. If the password entry is found and successfully deleted, a success message is returned. If the ID is invalid or the deletion fails, an appropriate error message is returned.

#### **Endpoint**

- **Method:** `POST`
- **Path:** `/passwords/delete`

#### **Request Body**

The request body should be a JSON object with the following field:

| Field | Type     | Description                                                   |
| ----- | -------- | ------------------------------------------------------------- |
| `id`  | `String` | The unique identifier (UUID) of the password entry to delete. |

**Example Request Body:**

```json
{
  "id": "b9b7f790-d9f1-4e16-a2a9-b9b0b3f924f5"
}
```

#### **Response**

- **Success Response:**

  - **Status Code:** `200 OK`
  - **Body:** A JSON object with a success message.
    ```json
    {
      "message": "Password deleted successfully"
    }
    ```

- **Error Responses:**

  - **Status Code:** `400 Bad Request`

    - **Body:** A JSON object with an error message if the `id` is invalid (not a valid UUID).
      ```json
      {
        "message": "Invalid password ID."
      }
      ```

  - **Status Code:** `500 Internal Server Error`
    - **Body:** A JSON object with an error message if the database operation fails.
      ```json
      {
        "message": "Database error: <error details>"
      }
      ```

#### **Validation**

1. **ID Validation:**

   - The `id` field must be a valid UUID string. If it is not, the route returns a `400 Bad Request` error with the message `"Invalid password ID."`

#### **Database Interaction**

- The `delete` method of the `Database` struct is used to remove the password entry from the database.
- The `Database` struct is provided via Axum's `State` extractor.

#### **Example Usage**

```bash
curl -X POST http://localhost:3000/passwords/delete \
-H "Content-Type: application/json" \
-d '{
  "id": "b9b7f790-d9f1-4e16-a2a9-b9b0b3f924f5"
}'
```

**Successful Response:**

```json
{
  "message": "Password deleted successfully"
}
```

**Error Response (Invalid ID):**

```json
{
  "message": "Invalid password ID."
}
```

### **Route: Get Password**

#### **Description**

This route allows clients to retrieve a stored password entry from the database by providing the unique password ID. The ID must be a valid UUID. If the password entry is found, it will be returned in the response. If the ID is invalid or the retrieval fails, an appropriate error message will be provided.

#### **Endpoint**

- **Method:** `GET`
- **Path:** `/passwords`

#### **Query Parameters**

The query parameters should include the following field:

| Field | Type     | Description                                                     |
| ----- | -------- | --------------------------------------------------------------- |
| `id`  | `String` | The unique identifier (UUID) of the password entry to retrieve. |

**Example Query:**

```http
GET /passwords?id=b9b7f790-d9f1-4e16-a2a9-b9b0b3f924f5
```

#### **Response**

- **Success Response:**

  - **Status Code:** `200 OK`
  - **Body:** A JSON object containing the password entry.
    ```json
    {
      "service": "example.com",
      "nonce": "valid-nonce-123",
      "cipher": "encrypted-password-456",
      "created_at": "2023-10-01T12:00:00Z",
      "updated_at": "2023-10-01T12:00:00Z"
    }
    ```

- **Error Responses:**

  - **Status Code:** `400 Bad Request`

    - **Body:** A JSON object with an error message if the `id` is invalid (not a valid UUID).
      ```json
      {
        "message": "Invalid password ID."
      }
      ```

  - **Status Code:** `500 Internal Server Error`
    - **Body:** A JSON object with an error message if the database operation fails.
      ```json
      {
        "message": "Database error: <error details>"
      }
      ```

#### **Validation**

1. **ID Validation:**

   - The `id` query parameter must be a valid UUID string. If it is not, the route returns a `400 Bad Request` error with the message `"Invalid password ID."`

#### **Database Interaction**

- The `get_by_id` method of the `Database` struct is used to retrieve the password entry from the database.
- The `Database` struct is provided via Axum's `State` extractor.

#### **Example Usage**

```bash
curl -X GET "http://localhost:3000/passwords?id=b9b7f790-d9f1-4e16-a2a9-b9b0b3f924f5"
```

**Successful Response:**

```json
{
  "service": "example.com",
  "nonce": "valid-nonce-123",
  "cipher": "encrypted-password-456",
  "created_at": "2023-10-01T12:00:00Z",
  "updated_at": "2023-10-01T12:00:00Z"
}
```

**Error Response (Invalid ID):**

```json
{
  "message": "Invalid password ID."
}
```

### **Route: Search Password**

#### **Description**

This route allows clients to search for stored password entries in the database using a search term. The search term is used to match against the service names, and the results are paginated. The client can specify the page number and page size, with a default of 1 for the page number and a configurable maximum size for the page size. If the search term is invalid or the pagination exceeds the maximum allowed, an error message will be returned.

#### **Endpoint**

- **Method:** `GET`
- **Path:** `/passwords/search`

#### **Query Parameters**

The query parameters should include the following fields:

| Field         | Type     | Description                                                               |
| ------------- | -------- | ------------------------------------------------------------------------- |
| `search_term` | `String` | The term used to search for matching services.                            |
| `page`        | `u32`    | (Optional) The page number for pagination (default: 1).                   |
| `page_size`   | `u32`    | (Optional) The number of results per page (default: configured max size). |

**Example Query:**

```http
GET /passwords/search?search_term=example&page=1&page_size=10
```

#### **Response**

- **Success Response:**

  - **Status Code:** `200 OK`
  - **Body:** A JSON array containing the list of matching password entries.
    ```json
    [
      {
        "service": "example.com",
        "nonce": "valid-nonce-123",
        "cipher": "encrypted-password-456",
        "created_at": "2023-10-01T12:00:00Z",
        "updated_at": "2023-10-01T12:00:00Z"
      },
      {
        "service": "example.org",
        "nonce": "valid-nonce-124",
        "cipher": "encrypted-password-457",
        "created_at": "2023-10-02T12:00:00Z",
        "updated_at": "2023-10-02T12:00:00Z"
      }
    ]
    ```

- **Error Responses:**

  - **Status Code:** `400 Bad Request`

    - **Body:** A JSON object with an error message if the pagination exceeds the maximum allowed size.
      ```json
      {
        "message": "Max Pagination Size Exceeded"
      }
      ```

  - **Status Code:** `500 Internal Server Error`
    - **Body:** A JSON object with an error message if the database operation fails.
      ```json
      {
        "message": "Database error: <error details>"
      }
      ```

#### **Validation**

1. **Pagination Size:**

   - If the `page_size` exceeds the configured maximum size, the route returns a `400 Bad Request` error with the message `"Max Pagination Size Exceeded"`.

2. **Search Term:**

   - The `search_term` is used to filter services by name. If there are no matching results, an empty array will be returned without an error.

#### **Database Interaction**

- The `search_by_service` method of the `Database` struct is used to search for password entries by their associated service name. The results are paginated based on the `page` and `page_size` parameters.
- The `Database` struct is provided via Axum's `State` extractor.

#### **Example Usage**

```bash
curl -X GET "http://localhost:3000/passwords/search?search_term=example&page=1&page_size=10"
```

**Successful Response:**

```json
[
  {
    "service": "example.com",
    "nonce": "valid-nonce-123",
    "cipher": "encrypted-password-456",
    "created_at": "2023-10-01T12:00:00Z",
    "updated_at": "2023-10-01T12:00:00Z"
  },
  {
    "service": "example.org",
    "nonce": "valid-nonce-124",
    "cipher": "encrypted-password-457",
    "created_at": "2023-10-02T12:00:00Z",
    "updated_at": "2023-10-02T12:00:00Z"
  }
]
```

**Error Response (Max Pagination Size Exceeded):**

```json
{
  "message": "Max Pagination Size Exceeded"
}
```

### **Route: Sort Passwords**

#### **Description**

This route allows clients to sort stored password entries in the database based on a specific criterion, such as creation date or last update date. The sorting is performed based on the `sort_by` query parameter. Additionally, pagination is supported with configurable page numbers and page sizes. The client can request passwords to be sorted according to the provided field and retrieve the results in pages.

#### **Endpoint**

- **Method:** `GET`
- **Path:** `/passwords/sort`

#### **Query Parameters**

The query parameters should include the following fields:

| Field       | Type     | Description                                                                       |
| ----------- | -------- | --------------------------------------------------------------------------------- |
| `sort_by`   | `String` | The field by which the results will be sorted (e.g., `created_at`, `updated_at`). |
| `page`      | `u32`    | (Optional) The page number for pagination (default: 1).                           |
| `page_size` | `u32`    | (Optional) The number of results per page (default: configured max size).         |

**Example Query:**

```http
GET /passwords/sort?sort_by=created_at&page=1&page_size=10
```

#### **Response**

- **Success Response:**

  - **Status Code:** `200 OK`
  - **Body:** A JSON array containing the sorted list of password entries.
    ```json
    [
      {
        "service": "example.com",
        "nonce": "valid-nonce-123",
        "cipher": "encrypted-password-456",
        "created_at": "2023-10-01T12:00:00Z",
        "updated_at": "2023-10-01T12:00:00Z"
      },
      {
        "service": "example.org",
        "nonce": "valid-nonce-124",
        "cipher": "encrypted-password-457",
        "created_at": "2023-10-02T12:00:00Z",
        "updated_at": "2023-10-02T12:00:00Z"
      }
    ]
    ```

- **Error Responses:**

  - **Status Code:** `400 Bad Request`

    - **Body:** A JSON object with an error message if the `sort_by` parameter is invalid or if pagination exceeds the maximum allowed size.
      ```json
      {
        "message": "Max Pagination Size Exceeded"
      }
      ```
      or
      ```json
      {
        "message": "Invalid sort parameter: <error details>"
      }
      ```

  - **Status Code:** `500 Internal Server Error`
    - **Body:** A JSON object with an error message if the database operation fails.
      ```json
      {
        "message": "Database error: <error details>"
      }
      ```

#### **Validation**

1. **Pagination Size:**

   - If the `page_size` exceeds the configured maximum size, the route returns a `400 Bad Request` error with the message `"Max Pagination Size Exceeded"`.

2. **Sort Parameter:**

   - The `sort_by` query parameter must be a valid field by which the passwords can be sorted. If it is invalid, the route returns a `400 Bad Request` error with the message `"Invalid sort parameter"`.

#### **Database Interaction**

- The `list_sorted` method of the `Database` struct is used to retrieve password entries sorted according to the `sort_by` parameter. The results are paginated based on the `page` and `page_size` parameters.
- The `Database` struct is provided via Axum's `State` extractor.

#### **Example Usage**

```bash
curl -X GET "http://localhost:3000/passwords/sort?sort_by=created_at&page=1&page_size=10"
```

**Successful Response:**

```json
[
  {
    "service": "example.com",
    "nonce": "valid-nonce-123",
    "cipher": "encrypted-password-456",
    "created_at": "2023-10-01T12:00:00Z",
    "updated_at": "2023-10-01T12:00:00Z"
  },
  {
    "service": "example.org",
    "nonce": "valid-nonce-124",
    "cipher": "encrypted-password-457",
    "created_at": "2023-10-02T12:00:00Z",
    "updated_at": "2023-10-02T12:00:00Z"
  }
]
```

**Error Response (Max Pagination Size Exceeded):**

```json
{
  "message": "Max Pagination Size Exceeded"
}
```

**Error Response (Invalid Sort Parameter):**

```json
{
  "message": "Invalid sort parameter: <error details>"
}
```
