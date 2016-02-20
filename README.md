# Intro to Rust

This repo accompanies the talk given at CanFP in February 2016 on an Introduction to the Rust Programming Language.
Slides for the talk are available at: https://goo.gl/j4uAFq

Under code you will find various examples that where run during the talk.

## code/adts
Simple enum types that demonstrate:
 * basic enums
 * enum types that can take a value
 * parameterised enum types
 * recursive parameterised enum types

## code/composition
Show how function composition can be achieved in Rust.

## code/currying
Demonstrates how curryied functions can be created in Rust.

## code/ffi
Demonstrates calling a shared library (written in Rust) from Python.

## code/hello_world
Shows very basic Rust syntax.

## code/immutability
Very simple example showing Rust's preference for immutable and how to work with that.

## code/ownership
Shows how to use mutability and/or borrowing to satisty Rust's borrow checker. Checking out one of several tags in this repo (called "ownership-*") will change the contents of code/ownership/src/main.rs to demonstrate a point made in the talk e.g. compliation errors form the borrow checker or how to let a function borrow a variable.

## code/security
This code was not presented in the talk but shows a basic buffer overflow in C and tries to recreate this in Rust showing how the language protects you against such errors.

## code/undefined
This is from the Q+A session and demonstrates how Rust handles taking the absolute value of the lowest possible signed number.  As pointed out, other languages treat this as 'undefined' since two's complement storage of a signed number should mean INT_MIN + INT_MAX = -1. Rust in fact `panic`'s (i.e. raise a runtime error).
