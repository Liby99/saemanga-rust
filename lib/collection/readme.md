# Macro Test Instructions

First you want to install the `cargo-expand` crate by

```
$ cargo install cargo-expand
```

Then you can expand the code using

```
$ cargo expand --test user_coll
```

To get the spanned version of `test/user_coll.rs`.

You should be able to see the `User` struct being expanded with `impl`