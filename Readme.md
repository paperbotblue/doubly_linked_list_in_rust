# Doubly Linked List in Rust 🦀

A simple **doubly linked list** implemented in Rust, created for **learning and practice**.  
This project focuses on using `Arc<Mutex<>>` for shared ownership and interior mutability, which are common concepts in concurrent Rust programming.

---

## 📚 Overview

This linked list supports:

- Insertion at the front (`push_front`)
- Insertion at the back (`push_back`)
- Removal of elements by value (`remove`)
- Custom `Debug` output for visualization

> ⚠️ **Note:** This is not optimized for production use — it's for educational purposes.

---

## 🛠 Technologies

- Language: [Rust](https://www.rust-lang.org/)
- Concurrency: `Arc<Mutex<T>>` from `std::sync`
- Debugging: Custom `Debug` trait implementation

---
