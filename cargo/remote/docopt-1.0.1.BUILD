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
  "notice", # "MIT"
  "unencumbered", # "Unlicense"
])

load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
)


# Unsupported target "cargo" with type "example" omitted
# Unsupported target "cp" with type "example" omitted
# Unsupported target "decode" with type "example" omitted

rust_library(
    name = "docopt",
    crate_root = "src/lib.rs",
    crate_type = "lib",
    srcs = glob(["**/*.rs"]),
    deps = [
        "@raze__lazy_static__1_1_0//:lazy_static",
        "@raze__regex__1_0_5//:regex",
        "@raze__serde__1_0_80//:serde",
        "@raze__serde_derive__1_0_80//:serde_derive",
        "@raze__strsim__0_7_0//:strsim",
    ],
    rustc_flags = [
        "--cap-lints allow",
    ],
    version = "1.0.1",
    crate_features = [
    ],
)

rust_binary(
    # Prefix bin name to disambiguate from (probable) collision with lib name
    # N.B.: The exact form of this is subject to change.
    name = "cargo_bin_docopt_wordlist",
    crate_root = "src/wordlist.rs",
    srcs = glob(["**/*.rs"]),
    deps = [
        # Binaries get an implicit dependency on their lib
        ":docopt",
        "@raze__lazy_static__1_1_0//:lazy_static",
        "@raze__regex__1_0_5//:regex",
        "@raze__serde__1_0_80//:serde",
        "@raze__serde_derive__1_0_80//:serde_derive",
        "@raze__strsim__0_7_0//:strsim",
    ],
    rustc_flags = [
        "--cap-lints allow",
    ],
    version = "1.0.1",
    crate_features = [
    ],
)

# Unsupported target "hashmap" with type "example" omitted
# Unsupported target "optional_command" with type "example" omitted
# Unsupported target "verbose_multiple" with type "example" omitted
