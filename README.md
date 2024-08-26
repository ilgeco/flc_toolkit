# Formal Languages and Compilers Toolkit

Work in progress!


# Coverage

### Example: How to generate source-based coverage for a Rust project

1. Install the llvm-tools or llvm-tools-preview component:

   ```sh
   rustup component add llvm-tools-preview
   ```

2. Ensure that the following environment variable is set up:

   ```sh
   export RUSTFLAGS="-Cinstrument-coverage"
   ```

3. Build your code:

   `cargo build`

4. Ensure each test runs gets its own profile information by defining the LLVM_PROFILE_FILE environment variable (%p will be replaced by the process ID, and %m by the binary signature):

   ```sh
   export LLVM_PROFILE_FILE="your_name-%p-%m.profraw"
   ```

5. Run your tests:

   `cargo test`

In the CWD, you will see a `.profraw` file has been generated. This contains the profiling information that grcov will parse, alongside with your binaries.

Generate a html coverage report like this:

```sh
grcov . -s ./src/ --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
```