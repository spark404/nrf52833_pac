#!/bin/bash

svd2rust -i nrf52833.svd --target=cortex-m --reexport-core-peripherals -f peripheral_singleton::c:
form -i lib.rs -o src/ && rm lib.rs
cargo fmt