<div align="center">

# Night Raven

<a href="/LICENSE"> ![License](https://img.shields.io/badge/license-GPL%20v3-blueviolet?style=for-the-badge)</a>
<a href="#todos"> ![TODOs](https://img.shields.io/badge/status-WIP-informational?style=for-the-badge&color=ff69b4) </a>

[Summary](#summary)
•
[Todos](#todos)
•
[Docs](https://docs.rs/asset-derive/latest/asset_derive/)

</div>

<div align="center">

<br>

# Summary

</div>

> A blazingly fast procedural macro in Rust for nesting types ergonomically!
> The bird of the shadows has found its nest, the Night Raven!

Please feel free to create a ticket for new features/bugs or create a pull request.

## TODOs

> List of ideas I have at the moment for this project's expansion.
> Please create an issue for a new item to add to this list, using
> `todo` label.

- [ ] Derive macro for type features and manipulation
- [ ] Support structs/unions
- [ ] Support named variants and multiple fields in unnamed variants

### Code Tree

```
nightraven/             <-- Main crate (public interface)
    src/
    examples/           <-- Houses examples using the trait and macro itself.
    nightraven-macros/  <-- Internal macros crate. Will be pulled in by main crate.
        src/
```

## Example

```rust
nightraven! {
    enum TopLevel {
        SubVariant(
            enum SecondLevel {
                SubVariant(
                    #[derive(Debug)]
                    enum ThirdLevel {
                        SomeVariant { field1: bool, field2: u32 },
                        TestI,
                        TestII,
                    }
                ),
                TestA,
            }
        ),
        Test1,
        Test2,
    }
}
```
