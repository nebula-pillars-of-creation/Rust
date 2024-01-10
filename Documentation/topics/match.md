# match

The `match` keyword is used to match a value against a series of patterns. The `match` keyword is followed by an
expression, and then a series of `arms`. Each arm consists of a pattern and the code that should be run if the value
matches that pattern.

```text
fn main() {
    let number = 3;

    match number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
```

In this example, the `match` keyword is followed by the `number` variable. The first arm has a pattern of `1` and the
code to run if the value matches that pattern is `println!("one")`. The second arm has a pattern of `2` and the code to
run if the value matches that pattern is `println!("two")`. The third arm has a pattern of `3` and the code to run if
the value matches that pattern is `println!("three")`. The final arm has a pattern of `_` which is a catch-all pattern
that will match any value. The code to run if the value matches that pattern is `println!("anything")`.

## Practical Examples

### Scenario: Handling Configurations

Imagine you are writing a program that requires different configurations based on the environment. You want to use
`match` to handle the different configurations.

#### Example of Handling Configurations

```text
enum ConfigMode {
    Fast,
    Safe,
    Balanced,
}

fn main() {
    let config = ConfigMode::Safe; // This might be determined from a file or user input

    match config {
        ConfigMode::Fast => println!("Running in fast mode: Performance optimized"),
        ConfigMode::Safe => println!("Running in safe mode: Security optimized"),
        ConfigMode::Balanced => println!("Running in balanced mode: Trade-off between performance and security"),
    }
}

```

In this example, the `match` keyword is followed by the `config` variable. The first arm has a pattern of
`ConfigMode::Fast` and the code to run if the value matches that pattern is `println!("Running in fast mode:
Performance optimized")`. The second arm has a pattern of `ConfigMode::Safe` and the code to run if the value matches
that pattern is `println!("Running in safe mode: Security optimized")`. The third arm has a pattern of
`ConfigMode::Balanced` and the code to run if the value matches that pattern is `println!("Running in balanced mode:
Trade-off between performance and security")`.

### Scenario: Handling Errors

Imagine you are writing a program that requires different error handling based on the type of error. You want to use
`match` to handle the different errors.

#### Example of Handling Errors

```text
enum Error {
    NotFound,
    PermissionDenied,
    Unknown,
}

fn main() {
    let error = Error::NotFound; // This might be determined from a file or user input

    match error {
        Error::NotFound => println!("Error: Not found"),
        Error::PermissionDenied => println!("Error: Permission denied"),
        Error::Unknown => println!("Error: Unknown"),
    }
}

```

In this example, the `match` keyword is followed by the `error` variable. The first arm has a pattern of
`Error::NotFound` and the code to run if the value matches that pattern is `println!("Error: Not found")`. The second
arm has a pattern of `Error::PermissionDenied` and the code to run if the value matches that pattern is
`println!("Error: Permission denied")`. The third arm has a pattern of `Error::Unknown` and the code to run if the value
matches that pattern is `println!("Error: Unknown")`.

### Scenario: Handling HTTP Status Codes

Imagine you are writing a program that requires different handling based on the HTTP status code. You want to use
`match` to handle the different status codes.

#### Example of Handling HTTP Status Codes

```text
enum StatusCode {
    Ok,
    Created,
    Accepted,
    NoContent,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
}


fn main() {
    let status_code = StatusCode::Ok; // This might be determined from a file or user input

    match status_code {
        StatusCode::Ok => println!("Status code: 200"),
        StatusCode::Created => println!("Status code: 201"),
        StatusCode::Accepted => println!("Status code: 202"),
        StatusCode::NoContent => println!("Status code: 204"),
        StatusCode::BadRequest => println!("Status code: 400"),
        StatusCode::Unauthorized => println!("Status code: 401"),
        StatusCode::Forbidden => println!("Status code: 403"),
        StatusCode::NotFound => println!("Status code: 404"),
        StatusCode::MethodNotAllowed => println!("Status code: 405"),
        StatusCode::InternalServerError => println!("Status code: 500"),
        StatusCode::NotImplemented => println!("Status code: 501"),
        StatusCode::BadGateway => println!("Status code: 502"),
        StatusCode::ServiceUnavailable => println!("Status code: 503"),
        StatusCode::GatewayTimeout => println!("Status code: 504"),
    }
}

```