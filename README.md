gd32f30x-pac
============

Peripheral access API for GD32F30x microcontrollers.

### How to re-generate
```
svd patch svds/xd-patch.yaml
svd2rust -i svds/GD32F30x_XD.svd.patched --target cortex-m
rm -rf src
form -i lib.rs -o src
rm lib.rs
cargo fmt
```
