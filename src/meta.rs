pub const SHORT_HELP: &str = "Call 'cum --help' for help";

pub const HELP_MSG: &str = "Usage:
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

Examples:
  prog build --force
  prog run src/main.cpp
  prog test -- --gtest_filter=MyTestSuite.*";

pub const VERSION_MSG: &str =
    "C.U.M. 0.1.0\nCopyright (c) 2025 Zloy Kot\nCompilation unit manager for clang++.";

pub const COMPILER: &str = "clang++";
