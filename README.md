# 🧱 strbuilder
StringBuilder for Rust without reallocations

## 📦 Installation
This project is published on [crates.io](https://crates.io/crates/strbuilder). You can add it to your project by running:
```bash
cargo add strbuilder
# or
cargo add strbuilder --features safe
```

## 🌟 Key features
- **No Reallocations**: Data never gets reallocated to a new, larger memory segment when the text grows. It simply links a new chunk onto the stack/heap
- **Cache-Line Alignment**: Every text chunk (`StringChunk`) is forced to align tightly to **64-byte boundaries**
- **Two sides**: You can use unsafe but perfomant (the default) or safe but more slower variants of code by switching the `safe` feature

## 🚀 Usage Example
```Rust
use strbuilder::StringBuilder;

let mut string_builder = StringBuilder::from("Hello,");
// or
let mut string_builder = StringBuilder::new();

string_builder.push_str(" ");
string_builder.push_str("world!");

let result = string_builder.to_string();
// or
println!("{:?}", string_builder);
```

## ⚙️ How it works
- **StringChunk**: A backward-linked list under the hood, utilizing byte chunks that fit perfectly into a single CPU cache-line
- **StringBuilder**: A lightweight controller containing a pointer to the last node of the linked list and the total byte count

```ASCII
[ first StringChunk (64B) ] <--- [ second StringChunk (64B) ] <--- [ third StringChunk (64B) ]
  |-- bytes: [u8; 56]              |-- bytes: [u8; 56]               |-- bytes: [u8; 56]
  |-- prev: null                   |-- prev: *const Chunk (first)    |-- prev: *const Chunk (second)

[ StringBuilder (16B) ]
  |-- bytes_count: usize
  |-- last_chunk: *mut StringChunk (third)
```

## 👍 Use recommendations
| Case | std::string::String | strbuilder::StringBuilder | Comment |
|------|---------------------|---------------------------|---------|
| Few `push_str` calls | ✅ | ❌ | `String` stores data contiguously in memory, offering better cache locality and zero overhead when allocations are rare |
| Many `push_str` calls | ❌ | ✅ | A large number of `push_str` calls increases the frequency of reallocations in a standard `String` |
| The size is known | ✅ | ❌ | Use `String::with_capacity(...)` instead |
| Large strings in `push_str` | ✅ | ❌ | For large inputs, `String` triggers very few reallocations and uses fast bulk memory copying, whereas StringBuilder is forced to allocate many nodes at once. |

Despite these specific limitations, this implementation of `StringBuilder` shines in several real-world scenarios:

- **Format Serializers / Deserializers** — Often perform a massive number of `push_str` calls with very short strings (e.g., syntax tokens like quotes, commas, and brackets).
- **Logging & Tracing Frameworks** — Avoids unpredictable latency spikes caused by `String` reallocations when continuously appending short runtime messages.
- **HTML Templating Engines** — Dynamically assemble complex pages from thousands of tiny static fragments (tags, attributes) and variables.
- **SQL Query Generators (ORMs)** — Frequently concatenate multiple small structural fragments (clauses, identifiers, parameters) where the final query length cannot be predicted.

*We are working to expand the usage areas with new versions...*

## 🧑‍⚖️ License
```
Copyright (c) 2026 Illia Diadenchuk
Licensed under Zlib license
```
