# Cookie Value Macro Expansion

Consider the following example

``` rust
pub enum Value {
  A,
  B,
  C
}
```

This will be turn into cookies with:

- Key: `"value"`
- Possible Values:
  - `"a"`,
  - `"b"`,
  - `"c"`.
- Default Value if not presented in Cookie:
  - `"a"`.

Note that all of the names will be snake-ify, that means names like `"ThisIsAValue"` will be turned into `"this_is_a_value"`.

Also the first value of the enum will be treated as the default value.

## Usage

Consider a `HandMode` enum with two values `Left` and `Right`. Let's give it a default of `Right`. Then

``` rust
#[derive(CookieValue)]
pub enum HandMode {
  Right,
  Left
}
```

This will make `HandMode` have a key of `hand_mode` and the values `left` and `right` while the later one being the default value.