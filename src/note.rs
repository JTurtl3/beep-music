use crate::beep;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Rest,
    C, D, E, F, G, A, B,
}
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::C => "C",
            Self::D => "D",
            Self::E => "E",
            Self::F => "F",
            Self::G => "G",
            Self::A => "A",
            Self::B => "B",
            Self::Rest => "REST"
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Note {
    value: Value,
    octave: u32,
    length: u32, // 1 for whole, 2 for half, etc
}

impl Note {

    pub fn new(value: Value, octave: u32, length: u32) -> Self {
        Self {value, octave, length}
    }

    pub fn get_frequency(&self) -> u32 {
        let multiplier = (2 as f64).powi(self.octave as i32 - 4);

        (multiplier * (match self.value {
            Value::C => 261,
            Value::D => 294,
            Value::E => 330,
            Value::F => 349,
            Value::G => 392,
            Value::A => 440,
            Value::B => 494,

            Value::Rest => 0,
        }) as f64) as u32
    }

    pub fn length_as_fraction(&self) -> f64 {
        1.0 / self.length as f64
    }

    pub fn get_length(&self) -> u32 {
        self.length
    }

    pub fn play(&self, bpm: u16) {
        let ms_per_beat = 1000.0/(bpm as f64 / 60.0);
        let length_in_ms = (self.length_as_fraction() * ms_per_beat) as u32;

        println!("{}{}\t{}",
            self.value.to_string(),
            if self.value != Value::Rest { self.octave.to_string() } else { "".to_string() },
            if self.length > 1 { format!("1/{}", self.length) } else { "1".to_string() },
        );

        if self.value != Value::Rest {
            beep(self.get_frequency(), length_in_ms);
        } else {
            let length_in_ns = length_in_ms as u32 * 1000000;
            std::thread::sleep(std::time::Duration::new(0, length_in_ns));
        }
    }
}