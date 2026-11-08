// Wait for LPTIM update interval (1 s if clock enabled)
// 

// includes function that maps time/date from RTC to lit LEDs, 
// second function that maps lit LED to pin +/- activation,
// third that bilds time-division multiplexing table 
//  (sequence of pins to activate/deactivate, for DMA to cycle through)

fn rtc_to_leds() {
    
}