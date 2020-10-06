// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

use ocaml_interop::{
    ocaml_alloc, ocaml_export, ToRust, OCaml, OCamlBytes, OCamlFloat, OCamlInt, OCamlInt32,
    OCamlInt64, OCamlList, ToOCaml,
};

ocaml_export! {
    fn rust_twice(_gc, num: OCaml<OCamlInt>) -> OCaml<OCamlInt> {
        let num: i64 = num.to_rust();
        unsafe { OCaml::of_i64(num * 2) }
    }

    fn rust_twice_boxed_i64(gc, num: OCaml<OCamlInt64>) -> OCaml<OCamlInt64> {
        let num: i64 = num.to_rust();
        let result = num * 2;
        ocaml_alloc!(result.to_ocaml(gc))
    }

    fn rust_twice_boxed_i32(gc, num: OCaml<OCamlInt32>) -> OCaml<OCamlInt32> {
        let num: i32 = num.to_rust();
        let result = num * 2;
        ocaml_alloc!(result.to_ocaml(gc))
    }

    fn rust_add_unboxed_floats_noalloc(_gc, num: f64, num2: f64) -> f64 {
        num * num2
    }

    fn rust_twice_boxed_float(gc, num: OCaml<OCamlFloat>) -> OCaml<OCamlFloat> {
        let num: f64 = num.to_rust();
        let result = num * 2.0;
        ocaml_alloc!(result.to_ocaml(gc))
    }

    fn rust_twice_unboxed_float(_gc, num: f64) -> f64 {
        num * 2.0
    }

    fn rust_increment_bytes(gc, bytes: OCaml<OCamlBytes>, first_n: OCaml<OCamlInt>) -> OCaml<OCamlBytes> {
        let first_n: i64 = first_n.to_rust();
        let first_n = first_n as usize;
        let mut vec: Vec<u8> = bytes.to_rust();

        for i in 0..first_n {
            vec[i] += 1;
        }

        ocaml_alloc!(vec.to_ocaml(gc))
    }

    fn rust_increment_ints_list(gc, ints: OCaml<OCamlList<OCamlInt>>) -> OCaml<OCamlList<OCamlInt>> {
        let mut vec: Vec<i64> = ints.to_rust();

        for i in 0..vec.len() {
            vec[i] += 1;
        }

        ocaml_alloc!(vec.to_ocaml(gc))
    }

    fn rust_make_tuple(gc, fst: OCaml<String>, snd: OCaml<OCamlInt>) -> OCaml<(String, OCamlInt)> {
        let fst: String = fst.to_rust();
        let snd: i64 = snd.to_rust();
        let tuple = (fst, snd);
        ocaml_alloc!(tuple.to_ocaml(gc))
    }

    fn rust_make_some(gc, value: OCaml<String>) -> OCaml<Option<String>> {
        let value: String = value.to_rust();
        let some_value = Some(value);
        ocaml_alloc!(some_value.to_ocaml(gc))
    }
}
