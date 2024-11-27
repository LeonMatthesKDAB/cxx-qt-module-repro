use std::{env, fs, path::PathBuf};

use cxx_qt_build::{CxxQtBuilder, QmlModule};
use qt_build_utils;

fn header_dir() -> PathBuf {
    PathBuf::from("src").canonicalize().unwrap()
}

fn main() {
    let interface =
        cxx_qt_build::Interface::default().export_include_directory(header_dir(), "qmlmod-src");
    let mut builder = CxxQtBuilder::library(interface);

    let core: QmlModule<&str, &str> = QmlModule {
        uri: "CxxQt.Repro",
        rust_files: &[
            "src/lib.rs",
        ],
        ..Default::default()
    };
    builder = builder
        .qt_module("Network")
        .qt_module("Quick")
        .qt_module("Gui")
        .qml_module(core);

    builder.build();
}
