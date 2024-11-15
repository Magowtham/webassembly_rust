# WASM + WASI

## What is WebAssembly?

WebAssembly is a portable binary instruction format that allows code written in various languages to run efficiently across different platforms, especially the web.

## Advantages of WebAssembly

1. **Binary Format for Speed:**
   WebAssembly, being a binary format, loads and executes much faster than traditional JavaScript. This is particularly useful for high-performance applications like games, video editing, and scientific simulations on the web.

2. **Portable and Cross-Platform:**
   WebAssembly is designed to run on any platform—browser, server, or standalone environments. Its portability allows developers to write code once and deploy it across multiple devices and operating systems.

3. **Compiles from Multiple Languages:**
   WebAssembly supports code written in C, C++, Rust, Go, and more, enabling developers to use languages beyond JavaScript for web applications.

4. **Improved Security:**
   WebAssembly runs in a sandboxed environment, isolating it from the rest of the system. This design reduces security risks by limiting access to sensitive resources.

## Why Traditional Compiled Languages Lack Portability (Compared to WebAssembly)

1. **Platform-Specific Binaries:**
   In traditional compiled languages (like C or C++), high-level code is compiled into binaries for specific targets, such as `linux/amd64`, `linux/arm64`, `darwin/amd64`, and `windows/amd64`. Each binary is tailored to a particular operating system and processor architecture.

2. **Limited Compatibility Across Platforms:**
   A binary compiled for one specific OS and architecture (e.g., `linux/amd64`) will not run on a different environment, such as macOS or ARM-based systems, leading to limited cross-platform usability.

3. **Need for Separate Compilations:**
   To support multiple platforms, developers must compile separate binaries for each OS and architecture combination, increasing complexity and deployment overhead.

4. **WebAssembly’s Portability Advantage:**
   Unlike traditional binaries, WebAssembly compiles code into a single portable binary format that can run consistently across various platforms, operating systems, and architectures (as long as they support WebAssembly). This makes it inherently cross-platform and environment-independent.

5. **Unified Runtime Support:**
   WebAssembly code can execute on any platform that supports a WebAssembly runtime (e.g., a browser or server environment), eliminating the need for recompilation. This enhances portability and versatility.

### Compiling High-Level Code to Specific Targets

![Compiling to Specific Targets](https://github.com/Magowtham/webassembly_rust/blob/main/assets/image1.png)

### Running the Compiled Binary on Different OS and Architecture-Based Machines

![Running Binary on Different Platforms](https://github.com/Magowtham/webassembly_rust/blob/main/assets/image2.png)

### Compiling High-Level Code to a Single WebAssembly Target

![Compiling to WebAssembly](https://github.com/Magowtham/webassembly_rust/blob/main/assets/image3.png)

### Running Compiled WebAssembly Module on Different OS and Processor Architectures (Supports WebAssembly Runtime)

![Running WebAssembly Module](https://github.com/Magowtham/webassembly_rust/blob/main/assets/image4.png)
