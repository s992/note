git_repository(
    name = "bazel_skylib",
    remote = "https://github.com/bazelbuild/bazel-skylib.git",
    tag = "0.5.0",  # change this to use a different release
)

git_repository(
    name = "io_bazel_rules_rust",
    commit = "4a9d0e0b6c66f1e98d15cbd3cccc8100a0454fc9",
    remote = "https://github.com/bazelbuild/rules_rust.git",
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")
rust_repositories()

load("//cargo:crates.bzl", "raze_fetch_remote_crates")
raze_fetch_remote_crates()

load(":workspace.bzl", "bazel_version")
bazel_version(name = "bazel_version")
