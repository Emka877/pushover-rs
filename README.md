# Pushover RS
## Description
It's a Rust client library you can use to interact with the [Pushover](https://www.pushover.net/) messaging API.

This client is unofficial and I'm in no way linked to Pushover.

## State: Working & Maintained

## Where to get it
Get it directly from [crates.io](https://crates.io/crates/pushover-rs)

Cargo.toml:
```Cargo.toml
[dependencies]
pushover-rs = "*"
...
```

## How to use
### You need
* A Pushover account ([Create one](https://pushover.net/signup))
* An application you have created through the [dashboard](https://www.pushover.net/) ([Create one now](https://pushover.net/apps/build))
* Your user key
* Your app's token

### Usage example
See the [examples folder](/examples)

## Documentation
[It's here](https://docs.rs/pushover-rs/latest/)

## Run the tests
To run the tests, you'll need:
- Create a /testdata folder, in there:
    - Put a picture called `attachment_test.jpg`
    - Create a credentials.json with this inside:
```json
{
    "token": "your token",
    "user": "your user key"
}
```
- To recap:
```
/testdata
    /testdata/attachment_test.jpg
    /testdata/credentials.json
```

## Note
The API might change or break in the future, but i'll try my best not to break anything.
