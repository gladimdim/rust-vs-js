# Rust WebAssembly ([Seed Framework](https://github.com/David-OConnor/seed)) versus JavaScript

**WHAT IS FASTER RENDERING 10000 DIVS FROM JSON**

Benchmark does following:

* Fetches JSON file of size 7Mb (from disk cache)
* Iterates over inner collection of *10000* elements
* Reads one property from each element
* Renders it into textContent of a div
* Cache is ON (it eliminates network delay for FetchAPI)

## Some Details
JSON has dozen of fields for each item in collection. I noticed that collection of item with 1 property is 2-3 times faster processed in JavaScript. For Rust version it makes no difference.

> - JavaScript version inserts final container into DOM only once.
> - Rust Seed inserts into DOM whenever it wants to do it.

## Code Examples

### JavaScript
```javascript
const container = document.createElement("div");
json.items.forEach((item) => {
    const div = document.createElement("div");
    div.textContent = item.long_description;
    container.appendChild(div);
});
// insert  into DOM
document.querySelector("#container").appendChild(container);
```

### Rust Seed
```rust
Some(response) => {
    let result: Vec<El<Msg>> = response
        .items
        .into_iter()
        .map(|node: model::ModelNode| div![node.long_description])
        .collect();
    result
}
```
# Result
It takes the same amount of time to fetch+parse+iterate+render in Rust WebAssembly as in pure JavaScript.

Unoptimized Rust binary is 2-3x times slower than it's optimized version.

Measurements are taken between button is clicked and appearance of strings on the screen

## JavaScript (NO FRAMEWORKS)
![JavaScript Result](js_results.PNG)

## Rust Optimized with opt-level=s
![Rust Optimized Result](rust_optimized.PNG)

## Rust Default Debug level
![Rust Debug Result](rust_debug.PNG)