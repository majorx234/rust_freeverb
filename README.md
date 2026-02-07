# Info
- reverb coding tutorial
- using jack as backend
- create UI dynamically
  - almost done
- lv2 plugin

# usage of lv2 plugin
- build: `cargo build --release`
- copy turtle files (`*.ttl`) to lv2-folder:
  - `cp -r freeverb_lv2/freeverb.lv2 ~/.lv2`
- copy libfreeverb_lv2.so to this folder too:
  - `cp target/release/libfreeverb_lv2.so ~/.lv2/freeverb.lv2`
- test with `lv2lint`:
  - `lv2lint -I ~/.lv2/freeverb.lv2/ https://github.com/majorx234/rust_freeverb`  
- notes:
  - all params are working, just `freeze`, acts as a boolean
    - value `0` means of, value > `0` means on (e.g: `0.1`)

# ToDo
- add plugin concept
  - host with jack backend
  - client as module
- later as clap plugin
- hot code reloading
  - https://github.com/irh/rust-hot-reloading

# References
- based on freeverb:
  - https://ccrma.stanford.edu/~jos/pasp/Freeverb.html
- idea from Ian Hobson talk at ADC 2018:
  - https://www.youtube.com/watch?v=Yom9E-67bdI
  - https://github.com/irh/freeverb-rs
