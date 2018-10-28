"""
cargo-raze crate build file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""
package(default_visibility = [
  # Public for visibility by "@raze__crate__version//" targets.
  #
  # Prefer access through "//cargo", which limits external
  # visibility to explicit Cargo.toml dependencies.
  "//visibility:public",
])

licenses([
  "restricted", # "no license"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "module" with type "test" omitted
# Unsupported target "read_str" with type "test" omitted
# Unsupported target "test_read" with type "example" omitted
# Unsupported target "test_read_simple" with type "example" omitted
# Unsupported target "test_scan" with type "example" omitted
# Unsupported target "test_scan_simple" with type "example" omitted
# Unsupported target "test_try_read" with type "example" omitted
# Unsupported target "test_try_scan" with type "example" omitted

rust_library(
    name = "text_io",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
    ],
    rustc_flags = [
        "--cap-lints allow",
    ],
    version = "0.1.7",
    crate_features = [
    ],
)

# Unsupported target "tuple" with type "test" omitted
