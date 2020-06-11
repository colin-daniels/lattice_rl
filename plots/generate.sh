#!/bin/bash

for fig in figure-1 figure-3 figure-s1 figure-4; do
    (
        rm -rf "$fig/" \
            && mkdir "$fig/" \
            && cd "$fig/" \
            && cargo run --release --bin "$fig" \
            && gnuplot "../$fig.gp"
    )
done
