# Compilation Unit Manager
**cum** is a simple, fast, and extensible build tool designed for C++ projects using **Clang**. It supports multiple presets, test execution, and configuration via a minimal `toml` file format.
## 📦 Features

- Compile, link, and run C++ programs with a single command
- Configurable build presets: `debug`, `release`, `test`
- Smart entry-point resolution
- Supports GoogleTest and custom test runners
- Designed for Clang with sane defaults (`lld`, static libs, etc.)
## 🧪 Usage
```
Usage:
  cum <command> [options] [files]

Commands:
  build             Compile and link with 'debug' preset
  run               Same as build, but also run executable at the end
  test              Build and run test_runner with 'test' preset
  init              Initialize new project in current directory

Options:
  -h, --help        Show this help message and exit
  -v, --version     Show version
  -f, --force       Forced build, ignored with 'test' command
  --preset=...      Specify preset for build

Variadic:
    [files]         Specify file with 'main()' function, it will be included in build proccess, and other entry points will be ignored
```
## 📌 Examples
```bash
cum build --force
cum run src/main.cpp
cum test -- --gtest_filter=MyTestSuite.*
```
## 🛠 Default Configuration (Cum.toml)
```toml
std = 20
include_dirs = ["include", "dependencies/include"]
lib_dirs = ["dependencies/lib"]
target_name = "program"
entry_points = ["src/main.cpp", "tests/test_runner.cpp"]

[presets.test]
cflags = ["-Wall", "-Wextra", "-g", "-O0", "-fno-omit-frame-pointer", "-DCUM_DEBUG"]
lflags = ["-g", "-O0", "-fuse-ld=lld", "-static-libgcc", "-static-libstdc++"]
libs = []
target_folder = "target/test_runner"

[presets.debug]
cflags = ["-Wall", "-Wextra", "-g", "-O0", "-fno-omit-frame-pointer", "-DCUM_DEBUG"]
lflags = ["-g", "-O0", "-fuse-ld=lld", "-static-libgcc", "-static-libstdc++"]
libs = []
target_folder = "target/debug"

[presets.release]
cflags = ["-Wall", "-Wextra", "-O3", "-march=native", "-DCUM_RELEASE", "-ffunction-sections", "-fdata-sections"]
lflags = ["-Wl,--gc-sections", "-fuse-ld=lld", "-static-libgcc", "-static-libstdc++", "-fvisibility=hidden"]
libs = []
target_folder = "target/release"
```
## 🧰 Requirements
* Clang compiler
* LLD linker (optional but recommended)
* gtest for test builds (optional)
