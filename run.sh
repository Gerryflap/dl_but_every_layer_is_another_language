#!/bin/sh
python3 python/layer.py | haskell/layer | rust/target/release/nn_layer_rust | java javaCode.Layer | c/a.out