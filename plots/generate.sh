#!/bin/bash

figure_1() {
    set -e
    rm -rf "figure-1/"
    mkdir "figure-1/"
    cd "figure-1/"
    cargo run --package analysis --bin figure-1 --release
    gnuplot "../figure-1.gp"
}


figure_3() {
    set -e
    rm -rf "figure-3/"
    mkdir "figure-3/"
    cd "figure-3/"
    cargo run --package analysis --bin figure-3 --release > value-fn-1d.dat
    gnuplot "../figure-3.gp"
}


figure_4() {
    set -e
    rm -rf "figure-4/"
    mkdir "figure-4/"
    cd "figure-4/"
    cargo run --package analysis --bin figure-4 --release
    gnuplot "../figure-4.gp"
}

figure_s1() {
    set -e
    rm -rf "figure-s1/"
    mkdir "figure-s1/"
    cd "figure-s1/"
    cargo run --package analysis --bin figure-s1 --release
    gnuplot "../figure-s1.gp"
}

(figure_1)
(figure_3)
(figure_s1)
(figure_4)

