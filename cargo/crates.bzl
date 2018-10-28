"""
cargo-raze crate workspace functions

DO NOT EDIT! Replaced on runs of cargo-raze
"""

def _new_http_archive(name, **kwargs):
    if not native.existing_rule(name):
        native.new_http_archive(name=name, **kwargs)

def _new_git_repository(name, **kwargs):
    if not native.existing_rule(name):
        native.new_git_repository(name=name, **kwargs)

def raze_fetch_remote_crates():

    _new_http_archive(
        name = "raze__aho_corasick__0_6_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/aho-corasick/aho-corasick-0.6.8.crate",
        type = "tar.gz",
        sha256 = "68f56c7353e5a9547cbd76ed90f7bb5ffc3ba09d4ea9bd1d8c06c8b1142eeb5a",
        strip_prefix = "aho-corasick-0.6.8",
        build_file = "//cargo/remote:aho-corasick-0.6.8.BUILD"
    )

    _new_http_archive(
        name = "raze__cfg_if__0_1_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/cfg-if/cfg-if-0.1.6.crate",
        type = "tar.gz",
        sha256 = "082bb9b28e00d3c9d39cc03e64ce4cea0f1bb9b3fde493f0cbc008472d22bdf4",
        strip_prefix = "cfg-if-0.1.6",
        build_file = "//cargo/remote:cfg-if-0.1.6.BUILD"
    )

    _new_http_archive(
        name = "raze__docopt__1_0_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/docopt/docopt-1.0.1.crate",
        type = "tar.gz",
        sha256 = "d60c92df70dfaaabecc14b409fd79f55ba0f247780529db1d73bfa601e1d3ac0",
        strip_prefix = "docopt-1.0.1",
        build_file = "//cargo/remote:docopt-1.0.1.BUILD"
    )

    _new_http_archive(
        name = "raze__lazy_static__1_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/lazy_static/lazy_static-1.1.0.crate",
        type = "tar.gz",
        sha256 = "ca488b89a5657b0a2ecd45b95609b3e848cf1755da332a0da46e2b2b1cb371a7",
        strip_prefix = "lazy_static-1.1.0",
        build_file = "//cargo/remote:lazy_static-1.1.0.BUILD"
    )

    _new_http_archive(
        name = "raze__libc__0_2_43",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/libc/libc-0.2.43.crate",
        type = "tar.gz",
        sha256 = "76e3a3ef172f1a0b9a9ff0dd1491ae5e6c948b94479a3021819ba7d860c8645d",
        strip_prefix = "libc-0.2.43",
        build_file = "//cargo/remote:libc-0.2.43.BUILD"
    )

    _new_http_archive(
        name = "raze__memchr__2_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/memchr/memchr-2.1.0.crate",
        type = "tar.gz",
        sha256 = "4b3629fe9fdbff6daa6c33b90f7c08355c1aca05a3d01fa8063b822fcf185f3b",
        strip_prefix = "memchr-2.1.0",
        build_file = "//cargo/remote:memchr-2.1.0.BUILD"
    )

    _new_http_archive(
        name = "raze__proc_macro2__0_4_20",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/proc-macro2/proc-macro2-0.4.20.crate",
        type = "tar.gz",
        sha256 = "3d7b7eaaa90b4a90a932a9ea6666c95a389e424eff347f0f793979289429feee",
        strip_prefix = "proc-macro2-0.4.20",
        build_file = "//cargo/remote:proc-macro2-0.4.20.BUILD"
    )

    _new_http_archive(
        name = "raze__quote__0_6_8",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/quote/quote-0.6.8.crate",
        type = "tar.gz",
        sha256 = "dd636425967c33af890042c483632d33fa7a18f19ad1d7ea72e8998c6ef8dea5",
        strip_prefix = "quote-0.6.8",
        build_file = "//cargo/remote:quote-0.6.8.BUILD"
    )

    _new_http_archive(
        name = "raze__redox_syscall__0_1_40",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/redox_syscall/redox_syscall-0.1.40.crate",
        type = "tar.gz",
        sha256 = "c214e91d3ecf43e9a4e41e578973adeb14b474f2bee858742d127af75a0112b1",
        strip_prefix = "redox_syscall-0.1.40",
        build_file = "//cargo/remote:redox_syscall-0.1.40.BUILD"
    )

    _new_http_archive(
        name = "raze__redox_termios__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/redox_termios/redox_termios-0.1.1.crate",
        type = "tar.gz",
        sha256 = "7e891cfe48e9100a70a3b6eb652fef28920c117d366339687bd5576160db0f76",
        strip_prefix = "redox_termios-0.1.1",
        build_file = "//cargo/remote:redox_termios-0.1.1.BUILD"
    )

    _new_http_archive(
        name = "raze__regex__1_0_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex/regex-1.0.5.crate",
        type = "tar.gz",
        sha256 = "2069749032ea3ec200ca51e4a31df41759190a88edca0d2d86ee8bedf7073341",
        strip_prefix = "regex-1.0.5",
        build_file = "//cargo/remote:regex-1.0.5.BUILD"
    )

    _new_http_archive(
        name = "raze__regex_syntax__0_6_2",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/regex-syntax/regex-syntax-0.6.2.crate",
        type = "tar.gz",
        sha256 = "747ba3b235651f6e2f67dfa8bcdcd073ddb7c243cb21c442fc12395dfcac212d",
        strip_prefix = "regex-syntax-0.6.2",
        build_file = "//cargo/remote:regex-syntax-0.6.2.BUILD"
    )

    _new_http_archive(
        name = "raze__serde__1_0_80",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/serde/serde-1.0.80.crate",
        type = "tar.gz",
        sha256 = "15c141fc7027dd265a47c090bf864cf62b42c4d228bbcf4e51a0c9e2b0d3f7ef",
        strip_prefix = "serde-1.0.80",
        build_file = "//cargo/remote:serde-1.0.80.BUILD"
    )

    _new_http_archive(
        name = "raze__serde_derive__1_0_80",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/serde_derive/serde_derive-1.0.80.crate",
        type = "tar.gz",
        sha256 = "225de307c6302bec3898c51ca302fc94a7a1697ef0845fcee6448f33c032249c",
        strip_prefix = "serde_derive-1.0.80",
        build_file = "//cargo/remote:serde_derive-1.0.80.BUILD"
    )

    _new_http_archive(
        name = "raze__strsim__0_7_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/strsim/strsim-0.7.0.crate",
        type = "tar.gz",
        sha256 = "bb4f380125926a99e52bc279241539c018323fab05ad6368b56f93d9369ff550",
        strip_prefix = "strsim-0.7.0",
        build_file = "//cargo/remote:strsim-0.7.0.BUILD"
    )

    _new_http_archive(
        name = "raze__syn__0_15_14",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/syn/syn-0.15.14.crate",
        type = "tar.gz",
        sha256 = "baaba45c6bf60fe29aaf241fa33306c0b75c801edea8378263a8f043b09a5634",
        strip_prefix = "syn-0.15.14",
        build_file = "//cargo/remote:syn-0.15.14.BUILD"
    )

    _new_http_archive(
        name = "raze__termion__1_5_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/termion/termion-1.5.1.crate",
        type = "tar.gz",
        sha256 = "689a3bdfaab439fd92bc87df5c4c78417d3cbe537487274e9b0b2dce76e92096",
        strip_prefix = "termion-1.5.1",
        build_file = "//cargo/remote:termion-1.5.1.BUILD"
    )

    _new_http_archive(
        name = "raze__text_io__0_1_7",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/text_io/text_io-0.1.7.crate",
        type = "tar.gz",
        sha256 = "9658b61ebd1d2a40c276ba2335890b9eb6550b67458a6fbce2022e58c3350a50",
        strip_prefix = "text_io-0.1.7",
        build_file = "//cargo/remote:text_io-0.1.7.BUILD"
    )

    _new_http_archive(
        name = "raze__thread_local__0_3_6",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/thread_local/thread_local-0.3.6.crate",
        type = "tar.gz",
        sha256 = "c6b53e329000edc2b34dbe8545fd20e55a333362d0a321909685a19bd28c3f1b",
        strip_prefix = "thread_local-0.3.6",
        build_file = "//cargo/remote:thread_local-0.3.6.BUILD"
    )

    _new_http_archive(
        name = "raze__ucd_util__0_1_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/ucd-util/ucd-util-0.1.1.crate",
        type = "tar.gz",
        sha256 = "fd2be2d6639d0f8fe6cdda291ad456e23629558d466e2789d2c3e9892bda285d",
        strip_prefix = "ucd-util-0.1.1",
        build_file = "//cargo/remote:ucd-util-0.1.1.BUILD"
    )

    _new_http_archive(
        name = "raze__unicode_xid__0_1_0",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/unicode-xid/unicode-xid-0.1.0.crate",
        type = "tar.gz",
        sha256 = "fc72304796d0818e357ead4e000d19c9c174ab23dc11093ac919054d20a6a7fc",
        strip_prefix = "unicode-xid-0.1.0",
        build_file = "//cargo/remote:unicode-xid-0.1.0.BUILD"
    )

    _new_http_archive(
        name = "raze__utf8_ranges__1_0_1",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/utf8-ranges/utf8-ranges-1.0.1.crate",
        type = "tar.gz",
        sha256 = "fd70f467df6810094968e2fce0ee1bd0e87157aceb026a8c083bcf5e25b9efe4",
        strip_prefix = "utf8-ranges-1.0.1",
        build_file = "//cargo/remote:utf8-ranges-1.0.1.BUILD"
    )

    _new_http_archive(
        name = "raze__version_check__0_1_5",
        url = "https://crates-io.s3-us-west-1.amazonaws.com/crates/version_check/version_check-0.1.5.crate",
        type = "tar.gz",
        sha256 = "914b1a6776c4c929a602fafd8bc742e06365d4bcbe48c30f9cca5824f70dc9dd",
        strip_prefix = "version_check-0.1.5",
        build_file = "//cargo/remote:version_check-0.1.5.BUILD"
    )

