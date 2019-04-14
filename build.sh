#!/bin/sh
echo "Building all layers..."
echo "Building python..."
sleep 0.1
echo "Just kidding, python has no compiler"
echo "Building haskell"
cd haskell
ghc layer.hs
cd ..
echo "Building Rust"
cd rust
cargo build --release
cd ..
echo "Building Java"
cd javaCode
javac Layer.java
cd ..
echo "Building C"
cd c
gcc layer.c -lm
cd ..
echo "Build script finished"