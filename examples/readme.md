# Note
The examples will not work, unless you do the following steps:

* Create a `examples/data/credentials.ron` file
* Put the following inside that file:
```ron
// Don't rename the class
ExampleCredentials (
    // Put your user key here
    user: "",
    // Put your app token here
    token: "",
)
```