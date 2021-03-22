use crate::note::*;

//todo: refactor, return Result<>
pub fn parse_notes(input: &str) -> Option<Vec<Note>> {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let tokens = tokens.iter().map(|s|{s.to_uppercase()});

    let mut result: Vec<Note> = Vec::new();
    let mut expecting_note = true;
    let mut temp_note_value = Value::C;
    let mut temp_note_octave = 4;

    for token in tokens {
        if expecting_note {
            if token.len() > 2 {
                return None
            }
            let value = token.chars().nth(0)?;
            
            temp_note_value = match value {
                'A' => Value::A,
                'B' => Value::B,
                'C' => Value::C,
                'D' => Value::D,
                'E' => Value::E,
                'F' => Value::F,
                'G' => Value::G,
                '_' => Value::Rest,
                
                _   => {
                    return None
                },
            };
            
            if token.len() == 2 {
                let octave = token.chars().nth(1)?;
                temp_note_octave = octave.to_digit(10)?;
            }
        } else {
            if let Ok(length) = token.parse::<u32>() {
                result.push(Note::new(temp_note_value, temp_note_octave, length));
            } else {
                return None;
            }
        }
        expecting_note = !expecting_note;
    }

    Some(result)
}