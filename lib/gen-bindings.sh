#!/bin/bash
bindgen --use-core wrapper.h -- "-I./libcsp/include" "-I./cfg" "-I./libcsp/src" > bindings.rs
