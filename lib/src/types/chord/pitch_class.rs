// PitchClass structure
use fraction::ToPrimitive;
use serde::{Serialize, Deserialize};

pub const SHARP_NOTES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
pub const FLAT_NOTES: [&str; 12] = ["C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B"];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PitchClass {
    pub note: String,
    pub just: i8,
    /// flat (-1), none (0) or sharp (1).
    pub accidental: i8,
    pub value: i8,
    pub sharp: bool,
}

impl PitchClass {
    pub(crate) fn from(just: i8, accidental: Option<i8>, sharp: Option<bool>) -> PitchClass {
        let mut p = PitchClass {
            just,
            accidental: 0,
            value: -1,
            sharp: true,
            note: String::with_capacity(2),
        };
        let pitch: i8;
        let accidental2: i8;
        if let Some(a) = accidental {
            pitch = p.just;
            accidental2 = a;
        } else {
            let value = p.just % 12;
            p.note = if value >= 0 {
                String::from(SHARP_NOTES[value as usize])
            } else {
                String::from(SHARP_NOTES[(12 + value).to_usize().unwrap()])
            };
            if p.note.ends_with('b') {
                accidental2 = -1;
                p.sharp = false;
            } else if p.note.ends_with('#') {
                accidental2 = 1;
            } else {
                accidental2 = 0;
            }
            pitch = value - accidental2;
        }
        p.just = pitch % 12;
        p.accidental = accidental2;
        p.value = p.just + accidental2;
        if sharp.is_none() {
            p.sharp = p.accidental >= 0;
        }
        p
    }

    #[allow(dead_code)]
    pub(crate) fn from_note(note: String) -> PitchClass {
        let mut p = PitchClass {
            note,
            just: 0,
            accidental: 0,
            value: -1,
            sharp: true,
        };
        if p.note.ends_with('b') {
            p.accidental = -1;
            p.sharp = false;
        } else if p.note.ends_with('#') {
            p.accidental = 1;
        }
        for i in 0i8..12i8 {
            if SHARP_NOTES[i as usize] == p.note || FLAT_NOTES[i as usize] == p.note {
                p.value = i;
                break;
            }
        }
        let pitch = p.value - p.accidental;
        p.just = pitch % 12;
        p.value = p.just + p.accidental;
        p
    }
}

impl std::fmt::Display for PitchClass {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.sharp {
            write!(f, "{}", SHARP_NOTES[self.value as usize])
        } else {
            write!(f, "{}", FLAT_NOTES[self.value as usize])
        }
    }
}

#[cfg(test)]
mod test {
    use super::PitchClass;

    #[test]
    fn test_pitch_1() {
        let p = PitchClass::from_note("D#".to_string());
        assert!(p.sharp, "D# is sharp? {}", true);
        assert_eq!(1, p.accidental);
    }

    #[test]
    fn test_pitch_2() {
        let p = PitchClass::from(4, Some(-1), None);
        assert_eq!(3, p.value);
        assert!(!p.sharp);
        assert_eq!("Eb", p.to_string(), "Note should be Eb");
    }

    #[test]
    fn test_pitch_3() {
        let p = PitchClass::from(4, Some(-1), Some(true));
        assert_eq!(3, p.value);
        assert_eq!("D#", p.to_string(), "Note should be D#");
    }

    #[test]
    fn test_pitch_4() {
        //let p = PitchClass::from(3, None, None);
        //TODO: assert_eq!("Eb", p.to_string(), "Note should be Eb"); //TODO: FIXME: error on the Python source
    }

    #[test]
    fn test_pitch_5() {
        let p = PitchClass::from(3, None, Some(true));
        assert_eq!("D#", p.to_string(), "Note should be D#");
    }
}
