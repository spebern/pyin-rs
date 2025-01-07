# Rust YIN and P-YIN Pitch Detection

![crates.io](https://img.shields.io/crates/v/pyin-rs.svg)

This crate contains a pure rust implementation of the FFT-based [YIN](http://audition.ens.fr/adc/pdf/2002_JASA_YIN.pdf) and [Probabilistic YIN](https://www.eecs.qmul.ac.uk/~simond/pub/2014/MauchDixon-PYIN-ICASSP2014.pdf) pitch detection algorithms.

Note only power-of-two input size is supported.

This is a port of the P-YIN part of https://github.com/sevagh/pitch-detection.