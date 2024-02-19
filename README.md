# Audio and TouchOSC controlled LED Strip in Rust

This is a quick project I threw together for a gig in Cologne:

https://github.com/TobiasGrothmann/audio_osc_controlled_led_strips/assets/28928394/944e109d-eaa3-4271-820a-57d5a98253ee

☝️ *credit for the audio performances in this excerpt:* [Joreng Boi](https://www.instagram.com/joreng_boi/) *and* [Daniel Kurosch Höpfner](https://danielkuroschhopfner.com/)


# What?

* Rust controlled, audioreactive, [touch-osc](https://play.google.com/store/apps/details?id=net.hexler.touchosc_a)-orchestrated [LED-strips](https://www.amazon.de/dp/B0868P9L1H/ref=pe_27091401_487027711_TE_SCE_dp_1)
  * lots of magic numbers
  * handwavy code
  * surprisingly performant
  * low level audio DSP stuff
  * a handful of assumptions
  * works on my machine ¯\_(ツ)_/¯

This repository is not self explanatory. Maybe you'll find this helpful if you are looking for examples on working with rust, [OSC](https://crates.io/crates/rosc), [audio dsp](cpal) or [WS2812 LED strips](https://cdn-shop.adafruit.com/datasheets/WS2812B.pdf).
