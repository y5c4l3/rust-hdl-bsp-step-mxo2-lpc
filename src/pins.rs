use rust_hdl::core::prelude::*;

pub const CLOCK_SPEED_12MHZ: u64 = 12_000_000;

#[derive(LogicInterface, Default)]
pub struct SegmentDisplay {
    pub segments: Signal<Out, Bits<8>>,
    pub dimmed: Signal<Out, Bit>,
}

pub fn clock() -> Signal<In, Clock> {
    let mut signal = Signal::default();
    signal.add_location(0, "C1");
    signal.add_signal_type(0, SignalType::LowVoltageCMOS_3v3);
    signal.connect();
    signal
}

pub fn keys() -> Signal<In, Bits<4>> {
    let mut signal = Signal::default();
    let locs = ["L14", "M13", "M14", "N14"];
    for (i, loc) in locs.iter().enumerate() {
        signal.add_location(i, loc);
        signal.add_signal_type(i, SignalType::LowVoltageCMOS_3v3);
    }
    signal
}

pub fn dip_switch() -> Signal<In, Bits<4>> {
    let mut signal = Signal::default();
    let locs = ["M7", "M8", "M9", "M10"];
    for (i, loc) in locs.iter().enumerate() {
        signal.add_location(i, loc);
    }
    signal
}

pub fn leds() -> Signal<Out, Bits<8>> {
    let mut signal = Signal::default();
    let locs = ["N13", "M12", "P12", "M11", "P11", "N10", "N9", "P9"];
    for (i, loc) in locs.iter().enumerate() {
        signal.add_location(i, loc);
        signal.add_signal_type(i, SignalType::LowVoltageCMOS_3v3);
    }
    signal
}

pub fn rgb_leds() -> [Signal<Out, Bits<3>>; 2] {
    let mut leds = <[Signal<_, _>; 2]>::default();
    for (i, loc) in ["P2", "N2", "M2"].iter().enumerate() {
        leds[0].add_location(i, loc);
        leds[0].add_signal_type(i, SignalType::LowVoltageCMOS_3v3);
    }
    for (i, loc) in ["P4", "N3", "M3"].iter().enumerate() {
        leds[1].add_location(i, loc);
        leds[1].add_signal_type(i, SignalType::LowVoltageCMOS_3v3);
    }
    leds
}

pub fn segment_displays() -> [SegmentDisplay; 2] {
    let mut displays = <[SegmentDisplay; 2]>::default();
    displays[0].dimmed.add_location(0, "C9");
    let locs = ["A10", "C11", "F2", "E1", "E2", "A9", "B9", "F1"];
    for (i, loc) in locs.iter().enumerate() {
        displays[0].segments.add_location(i, loc);
        displays[0]
            .segments
            .add_signal_type(i, SignalType::LowVoltageCMOS_3v3);
    }
    displays[1].dimmed.add_location(0, "A12");
    let locs = ["C12", "B14", "J1", "H1", "H2", "B12", "A11", "K1"];
    for (i, loc) in locs.iter().enumerate() {
        displays[1].segments.add_location(i, loc);
        displays[1]
            .segments
            .add_signal_type(i, SignalType::LowVoltageCMOS_3v3);
    }
    displays
}
