# Pushover RS
## Description
It's a Rust client library you can use to interact with the [Pushover](https://www.pushover.net/) messaging API.

This client is unofficial and I'm in no way linked to Pushover.

## How to use
### You need
* A Pushover account
* An application you have created through the [dashboard](https://www.pushover.net/)
* Your user key
* Your app's token

### Usage example
You need to send a push notification for whatever reason, here are the steps:

1. Create a `Message` using the `MessageBuilder`.
    * `use pushover-rs::MessageBuilder;`
2. Send the message using the send_pushover_request function.
    * `use pushover-rs::send_pushover_request;`
3. Done

### Example
```rust

```

## Documentation
Coming soon

## TODO
* "device" option
* "attachment" option