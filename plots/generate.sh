#!/bin/bash

figure_1() {
    set -e
    rm -rf "figure-1/"
    mkdir "figure-1/"
    cd "figure-1/"
    cargo run --package analysis --bin figure-1 --release
    gnuplot "../figure-1.gp"
}


figure_4() {
    set -e
    rm -rf "figure-4/"
    mkdir "figure-4/"
    cd "figure-4/"
    cargo run --package analysis --bin figure-4 --release > value-fn-1d.dat
    gnuplot "../figure-4.gp"
}


figure_5() {
    set -e
    rm -rf "figure-5/"
    mkdir "figure-5/"
    cd "figure-5/"
    cargo run --package analysis --bin figure-5
    gnuplot "../figure-5.gp"
}

figure_6() {
    set -e
    rm -rf "figure-6/"
    mkdir "figure-6/"
    cd "figure-6/"

    cargo run --package analysis --bin figure-6
    for f in value*.txt; do
        gnuplot <(cat <<< "
set view map
unset surface
set contour

set samples 1000
set isosamples 1000

set cntrparam levels incr -12,1,6
set xrange [0:1]
set yrange [0:1]

set table '${f}.tab'
splot '$f' w lines
unset table
"
)
    done
    
    gnuplot "../figure-6.gp"
}

(figure_1)
(figure_4)
(figure_6)
(figure_5)

