# Hello World #

## Build ##

1. Switch `source.crate-io` in `~/.cargo/config` to a near mirror
2. Update local vendor dir by

   ```
   cargo vendor --no-delete \
   --respect-source-config \
   --versioned-dirs \
   ~/projects/ori/cargo-vendor
   ```
   that `~/projects/ori/cargo-vendor` is the local vendor dir

3. Switch `source.crate-io` in `~/.cargo/config` to local
4. `cargo build`
