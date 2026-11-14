#[derive(Clone, Copy, PartialEq)]
pub enum InstructionSet {
    _Chip8,
    Chip8D6800,
    _Chip8E,
    _Chip8I,
    _Chip8X,
    _Chip10,
    Chip48,
    _Megachip,
    Schip1point0,
    Schip1point1,
    SchipModern,
    _SchipC,
    _XoChip,
}

impl InstructionSet {
    pub fn from_title(title: &str) -> Self {
        match title {
            "CHIP-8" => InstructionSet::_Chip8,
            "CHIP-8-D6800" => InstructionSet::Chip8D6800,
            "CHIP-8E" => InstructionSet::_Chip8E,
            "CHIP-8I" => InstructionSet::_Chip8I,
            "CHIP-8X" => InstructionSet::_Chip8X,
            "CHIP-10" => InstructionSet::_Chip10,
            "CHIP-48" => InstructionSet::Chip48,
            "MEGACHIP" => InstructionSet::_Megachip,
            "SCHIP-1.0" => InstructionSet::Schip1point0,
            "SCHIP-1.1" => InstructionSet::Schip1point1,
            "SCHIP-MODERN" => InstructionSet::SchipModern,
            "SCHIPC" => InstructionSet::_SchipC,
            "XO-CHIP" => InstructionSet::_XoChip,
            _ => InstructionSet::_Chip8,
        }
    }

    pub fn get_title(&self) -> &str {
        match self {
            InstructionSet::_Chip8 => "CHIP-8",
            InstructionSet::Chip8D6800 => "CHIP-8-D6800",
            InstructionSet::_Chip8E => "CHIP-8E",
            InstructionSet::_Chip8I => "CHIP-8I",
            InstructionSet::_Chip8X => "CHIP-8X",
            InstructionSet::_Chip10 => "CHIP-10",
            InstructionSet::Chip48 => "CHIP-48",
            InstructionSet::_Megachip => "MEGACHIP",
            InstructionSet::Schip1point0 => "SCHIP-1.0",
            InstructionSet::Schip1point1 => "SCHIP-1.1",
            InstructionSet::SchipModern => "SCHIP-MODERN",
            InstructionSet::_SchipC => "SCHIPC",
            InstructionSet::_XoChip => "XO-CHIP",
        }
    }

    pub fn has_hires_mode(&self) -> bool {
        if *self == InstructionSet::_Chip10
            || *self == InstructionSet::Schip1point0
            || *self == InstructionSet::Schip1point1
            || *self == InstructionSet::_Megachip
            || *self == InstructionSet::SchipModern
            || *self == InstructionSet::_SchipC
            || *self == InstructionSet::_XoChip
        {
            true
        } else {
            false
        }
    }

    pub fn is_cosmac(&self) -> bool {
        match self {
            InstructionSet::_Chip8
            | InstructionSet::Chip8D6800
            | InstructionSet::_Chip8E
            | InstructionSet::_Chip8I
            | InstructionSet::_Chip8X
            | InstructionSet::_Chip10
            | InstructionSet::Chip48 => true,
            _ => false,
        }
    }

    pub fn is_hp48(&self) -> bool {
        match self {
            InstructionSet::Chip48
            | InstructionSet::Schip1point0
            | InstructionSet::Schip1point1
            | InstructionSet::_SchipC => true,
            _ => false,
        }
    }

    pub fn is_schip(&self) -> bool {
        match self {
            InstructionSet::Schip1point0
            | InstructionSet::Schip1point1
            | InstructionSet::_SchipC
            | InstructionSet::SchipModern => true,
            _ => false,
        }
    }
}
