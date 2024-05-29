#!/bin/bash
bindgen --use-core wrapper.h -- "-I./libcsp/include" "-I./cfg" > bindings.rs
