#!/bin/bash

for i in {1..12}; do
    avvg_cpp=0
    avvg_rust=0
    for j in {1..10}; do
        _elapsed=$(./run.sh --hpx:threads=$i)
        cpp_ela=$(echo $_elapsed | grep -oE "cpp elapased: [0-9]+" | grep -oE '[0-9]+')
        rust_ela=$(echo $_elapsed | grep -oE "rust elapsed = [0-9]+" | grep -oE '[0-9]+')
        ((avvg_cpp+=cpp_ela))
        ((avvg_rust+=rust_ela))
        # echo $rust_ela
        # echo $cpp_ela
    done
    avvg_cpp=$(echo $avvg_cpp / 5 | bc -l)
    avvg_rust=$(echo $avvg_rust / 5 | bc -l)
    echo $i,$avvg_cpp,$avvg_rust
done