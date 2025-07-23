
---

### 📄 `README.md`

```markdown
# 📝 Rust Logger (File + Console)

This project is a **basic modular logger in Rust** demonstrating:

- ✅ File handling
- ✅ Custom traits
- ✅ Panic handling (`abort` vs `unwind`)
- ✅ Memory safety via borrow checker
- ✅ Console + file-based logging

---

## 🚀 How to Run

1. **Clone the repo / Save files:**
```

main.rs        → src/main.rs
logger.rs      → src/logger.rs

````

2. **Run the app:**
```bash
cargo run
````

3. **Output:**

   * `Console: Writing to console...` (stdout)
   * `log.txt` contains: `Writing to file...`

---

## 💥 Simulating Failure

To test a **critical failure (like disk full)**:

Uncomment in `main.rs`:

```rust
file_logger.simulate_failure();
```

This will trigger a panic:

```
thread 'main' panicked at 'Disk full or permission denied'
```

---

## ⚙️ Panic Mode Configuration

In `Cargo.toml`, under `[profile.dev]`:

```toml
[profile.dev]
panic = "unwind"   # Allows stack unwind (default)
# panic = "abort"  # Aborts immediately (no recovery)
```

Change this to switch between **`unwind` (recoverable)** or **`abort` (non-recoverable)** panic behavior.

---

## 🔧 Modules

| File        | Purpose                             |
| ----------- | ----------------------------------- |
| `main.rs`   | Entry point                         |
| `logger.rs` | Logger trait + File/Console loggers |

---

## 📚 Borrow Checker Flow

```text
main()
 └── owns FileLogger (mut) and ConsoleLogger
         └── log(&self, ...) → borrow checked
```

Rust ensures safe access and ownership to prevent data races.

---

## 🛠 Features To Extend

* Use `Result<T, E>` instead of direct `panic!`
* Add `catch_unwind` to gracefully handle panics
* Support log levels (info, warn, error)

---

## 📦 Built With

* Rust
* std::fs, std::io
* Traits and ownership model

