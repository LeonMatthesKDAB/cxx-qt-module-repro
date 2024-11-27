// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

/// The bridge definition for our QObject
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        /// An alias to the QString type
        type QString = cxx_qt_lib::QString;
    }
    unsafe extern "RustQt" {
        // The QObject definition
        // We tell CXX-Qt that we want a QObject class with the name MyObject
        // based on the Rust struct MyObjectRust.
        #[qobject]
        #[qml_element]
        #[qproperty(i32, number)]
        #[qproperty(QString, string)]
        #[namespace = "my_object"]
        type MyObject = super::MyObjectRust;
    }
    unsafe extern "RustQt" {
        // Declare the invokable methods we want to expose on the QObject
        #[qinvokable]
        #[cxx_name = "incrementNumber"]
        fn increment_number(self: Pin<&mut MyObject>);

        #[qinvokable]
        #[cxx_name = "sayHi"]
        fn say_hi(self: &MyObject, string: &QString, number: i32);
    }
}
use core::pin::Pin;
use cxx_qt_lib::QString;

/// The Rust struct for the QObjec
#[derive(Default, Debug)]
pub struct MyObjectRust {
    number: i32,
    string: QString,
}
impl qobject::MyObject {
    /// Increment the number Q_PROPERTY
    pub fn increment_number(self: Pin<&mut Self>) {
        let previous = *self.number();
        self.set_number(previous + 1);
    }

    /// Print a log message with the given string and number
    pub fn say_hi(&self, string: &QString, number: i32) {
        println!("Hi from Rust! String is '{string}' and number is {number}");
    }
}
