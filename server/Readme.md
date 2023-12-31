# Server
This crate makes use of the following dependency crates:
- [Axum](https://docs.rs/axum/latest/axum/): Build and run the server. We **cannot** use the latest version of `axum` (0.7) as `async-graphql` does not yet support it, please see [PR-1431](https://github.com/async-graphql/async-graphql/pull/1431).
- [Async-GraphQL](https://async-graphql.github.io/async-graphql/en/introduction.html): GraphQL library.
- [Tracing](https://crates.io/crates/tracing): Logging Solution.
- [SeaORM](https://www.sea-ql.org/SeaORM/docs/index/): Database ORM.

## Database

We are using `SeaORM` as our tool to communicate with the database. This orm supports the following databases:
- MySQL
- PostgreSQL
- SQLite

The project is setup to work with `PostgreSQL` database but you can adjust it for any of the other options with minimal change to the code.

## Logging

Logging in this project is done with the `tracing` crate. I have configured tracing to only print the `app` crate logs.

It will not print any logs from `sea-orm`.

> You can always update the log setup to change which logs get printed.

Resource: [Tracing Subscriber::EnvFilter](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html)


## Modules

### Entities
This module is generated by `sea-orm` by reading the tables in the database.

It is generated using the command `sea-orm-cli generate entity --database-url {db_url}/{db_name} -o server/src/entities` in the terminal.

We use the entities defined in this module to run operations against our database.

> An alternative approach would be to have `entities` as a separate crate similar to `migration`.

> Please don't make any modifications to this module.

### GraphQL
This module contains the `Query`, `Mutation` and `Subscriptions` structs and their method implementations.

The methods defined in the above structs are used by `async-graphql` to create the graphQL server.

### Repositories
This module is similar to a concept from NestJS.

This module contains submodules for each `entity` defined in the `entities` module. For example the `User` entity will have a corresponding module in `Respositories`.

The entity-submodules will contain CRUD operations for that entity.

#### Note

Some of the datatypes used for an entities properties are not serializable. Therefore they cannot be used by `async-graphql`.

There are quite a few different solutions, for instance we can update the `entities` folder to contain wrapper types for the non-serializable types.

The drawback of this approach is the `entities` file gets overwritten when we generate the entities module. This is a likely scenario as our tables will get updated as a project proceeds.

My solution is to have a graphQL stuct for every entity struct. The graphQL struct can have all the properties of the entity struct and use serializable types for all properties.

> We also have the flexibility of inlcuding extra properties or omitting specific properties if we don't need it. We can also alter the datatype of a property.

The graphql struct will also need to implement a `From trait` to convert the entity struct model to the graphql struct.

We call the structs `GraphQL<entity-name>`, i.e `GraphQLUser`.

You can checkout the [user](/src/repositories/user/mod.rs) module in repositories to learn more.

## Misc
This module contains miscellaneous code this project needs to function such as

- A function to return the env struct
- A function to return a db connection
- Re-usable structs such as a response struct.

## Testing

I have opted to use the Mock database approach for testing. You can learn more about this from sea-orms [docs](https://www.sea-ql.org/SeaORM/docs/write-test/mock/).

> This methodology **does not** mimic a real world database, the queries are not carried out as they would have been in a real database. Instead the mock database will return the mock data that **you define**. We are **not testing** the database.

This methodology does let us make sure that our functions are processing the data from the database and returning them correctly.

## Error Codes

I think its a good idea to return custom error codes with specific meaning rather than returning error message strings.

It makes it easier to handle the error and display appropriate (even multilingual) messages in the frontend.


* **500**: `Internal server error`
* **I100**: `Invalid Input`
* **U100**: `User not found`
* **P100**: `Post not found`
* **P101**: `Post does not belong to user`
