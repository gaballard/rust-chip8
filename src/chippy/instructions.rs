#[derive(Clone, Copy, PartialEq)]
pub enum InstructionSet {
    Chip8,
    _Chip8D6800,
    _Chip8E,
    _Chip8I,
    _Chip8X,
    _Chip10,
    _Chip48,
    _Megachip,
    _Schip1point0,
    Schip1point1,
    _SchipModern,
    _SchipC,
    _XoChip,
}

impl InstructionSet {
    pub fn get_title(&self) -> &str {
        match self {
            InstructionSet::Chip8 => "CHIP-8",
            InstructionSet::_Chip8D6800 => "CHIP-8-D6800",
            InstructionSet::_Chip8E => "CHIP-8E",
            InstructionSet::_Chip8I => "CHIP-8I",
            InstructionSet::_Chip8X => "CHIP-8X",
            InstructionSet::_Chip10 => "CHIP-10",
            InstructionSet::_Chip48 => "CHIP-48",
            InstructionSet::_Megachip => "MEGACHIP",
            InstructionSet::_Schip1point0 => "SCHIP-1.0",
            InstructionSet::Schip1point1 => "SCHIP-1.1",
            InstructionSet::_SchipModern => "SCHIP-MODERN",
            InstructionSet::_SchipC => "SCHIPC",
            InstructionSet::_XoChip => "XO-CHIP",
        }
    }

    pub fn has_hires_mode(&self) -> bool {
        if *self == InstructionSet::_Chip10
            || *self == InstructionSet::_Schip1point0
            || *self == InstructionSet::Schip1point1
            || *self == InstructionSet::_Megachip
            || *self == InstructionSet::_SchipModern
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
            InstructionSet::Chip8
            | InstructionSet::_Chip8D6800
            | InstructionSet::_Chip8E
            | InstructionSet::_Chip8I
            | InstructionSet::_Chip8X
            | InstructionSet::_Chip10
            | InstructionSet::_Chip48 => true,
            _ => false,
        }
    }

    pub fn is_schip(&self) -> bool {
        match self {
            InstructionSet::_Schip1point0
            | InstructionSet::Schip1point1
            | InstructionSet::_SchipC
            | InstructionSet::_SchipModern => true,
            _ => false,
        }
    }
}
