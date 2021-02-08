# Rust API client for openapi

Tezos client RPC API.

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 7.5
- Package version: 1.0.0
- Build package: org.openapitools.codegen.languages.RustClientCodegen

## Installation

Put the package under your project folder and add the following to `Cargo.toml` under `[dependencies]`:

```
    openapi = { path = "./generated" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**context_big_maps_big_map_id_script_expr_get**](docs/DefaultApi.md#context_big_maps_big_map_id_script_expr_get) | **get** /context/big_maps/{big_map_id}/{script_expr} | 


## Documentation For Models

 - [Bytes](docs/Bytes.md)
 - [GenericPrimAnyNumberOfArgsWithOrWithoutAnnot](docs/GenericPrimAnyNumberOfArgsWithOrWithoutAnnot.md)
 - [Int](docs/Int.md)
 - [Micheline007PsDelph1MichelsonV1Expression](docs/Micheline007PsDelph1MichelsonV1Expression.md)
 - [Model007PsDelph1MichelsonV1Primitives](docs/Model007PsDelph1MichelsonV1Primitives.md)
 - [String](docs/String.md)
 - [Unistring](docs/Unistring.md)
 - [UnistringOneOf](docs/UnistringOneOf.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author


