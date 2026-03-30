<p align="center">
  <img src="logo.png" width="150" alt="SilverPE Logo">
</p>

<h1 align="center">SilverPE</h1>

<p align="center">
  <strong>Minimal Windows PE Manual Loader implemented in Rust</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Language-Rust-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/Platform-Windows-blue.svg" alt="Windows">
  <img src="https://img.shields.io/badge/Arch-x86%20%2F%20x64-red.svg" alt="Architecture">
  <img src="https://img.shields.io/github/stars/hvns1414/SilverPE?style=flat-square" alt="Stars">
</p>

---

##  Overview

**SilverPE** is a high-performance, minimal Windows Portable Executable (PE) manual loader. This project explores the inner workings of the Windows OS by reimplementing the manual mapping process—originally developed in C#—using the memory safety and low-level power of **Rust**.

The primary goal is to demonstrate how a PE file can be loaded and executed entirely from memory without relying on the standard Windows loader (`LdrLoadDll`).

##  Features

- [x] **Manual Mapping:** Complete implementation of PE loading logic.
- [x] **Architecture Support:** Seamlessly handles both `x86` and `x64` PE files.
- [x] **Memory Execution:** Executes payloads directly from memory.
- [x] **Base Relocations:** Corrects image base offsets dynamically.
- [x] **Import Resolution:** Manually resolves IAT using `LoadLibrary` and `GetProcAddress`.
- [x] **Section Management:** Maps PE sections with appropriate memory

##(Running mimikatz)Demo version

<p align="center">
    <img src="1.png" width=800>
</p>
<p align="center">
    <img src="3.png" width=800>
</p>

### Unmatched Loader and PE Architecture (Error)
<p align="center">
    <img src="2.png" width=800>
</p>
