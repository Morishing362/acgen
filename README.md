# acgen
Tool that can generate boilerplates & test code for Atcoder's solution in Rust.

## Install
```bash
$ git clone https://github.com/Morishing362/acgen.git
$ cd acgen
$ cargo build --release
$ cp target/release/acgen [install directory path]
```
Please don't forget to add the directory path into PATH.

## Usage
Make a workspace for Atcoder and generate code in it.
```bash
mkdir [workspace] && cd [workspace]
acgen generate https://atcoder.jp/contests/abc200/tasks/abc200_a
cd solution_abc200_a
```
Write your solution in `src/main.rs` and then compile & test it.
```bash
cargo build
cargo test
```
Submit code if tests passed.
```bash
acgen submit
```
You'll be asked to login if you have no cookies.

## Edit header & footer
You can edit header & footer of code generation. Place those 2 files under `[workspace]/templates`.

## Cookies
Login session cookies will be stored in the parent directory of solution directory after you login to submit code with acgen. You can remove the cookie file whenever you want.
```bash
$ rm cookies.txt
```