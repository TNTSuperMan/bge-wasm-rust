pub struct MusicNote{
    pub length: u8,
    pub scale: u8
}
impl MusicNote{
    pub fn new(length: u8, scale: u8) -> MusicNote{
        MusicNote {
            length,
            scale
        }
    }
}
pub struct RestNote{
    pub length: u8
}
impl RestNote{
    pub fn new(length: u8) -> RestNote{
        RestNote {
            length
        }
    }
}

pub enum Note{
    Mus(MusicNote),
    Rest(RestNote)
}
impl Note {
    pub fn new(bin: u8, nbin: u8) -> Note{
        if (bin & 0b10000000) == 0 {
            return Note::Mus(MusicNote::new(bin, nbin));
        }else{
            return Note::Rest(RestNote::new(bin & 0b111111));
        }
    }
}

pub struct Music {
    pub sq1: Vec<Note>,
    pub sq2: Vec<Note>,
    pub tri: Vec<Note>,
    pub nse: Vec<Note>,
    pub tempo: u8,
    pub is_loop: bool
}
impl Music {
    pub fn new(d: u8) -> Music{
        Music {
            sq1: Vec::new(),
            sq2: Vec::new(),
            tri: Vec::new(),
            nse: Vec::new(),
            tempo: d & 0b01111111,
            is_loop: (d & 0b10000000) != 0
        }
    }
}

pub fn tokenize(data: &[u8]) -> Result<Vec<Music>, String>{
    let mut state: u8 = 0;
    let mut before: u8 = 0xff;
    let mut music: Music = Music::new(0);
    let mut musics: Vec<Music> = Vec::new();
    for d in data {
        match state {
            0 => { // Flag
                state = 1;
                music = Music::new(*d);
            },
            5 => { // End
                state = 0;
                musics.push(music);
                music = Music::new(0);
            },
            _ => {
                if ((before & 0b11000000) >> 6) == 0 {
                    let note = Note::new(before, *d);
                    before = 0xff;
                    match state {
                        1 => { music.sq1.push(note) },
                        2 => { music.sq2.push(note) },
                        3 => { music.tri.push(note) },
                        4 => { music.nse.push(note) },
                        _ => { }
                    }
                }else{
                    match (d & 0b11000000) >> 6 {
                        0 => {
                            before = *d;
                        },
                        1 => {
                            let note = Note::new(*d, 0);
                            match state {
                                1 => { music.sq1.push(note) },
                                2 => { music.sq2.push(note) },
                                3 => { music.tri.push(note) },
                                4 => { music.nse.push(note) },
                                _ => { }
                            }
                        },
                        3 => {
                            state += 1;
                        },
                        _ => {}
                    }
                }
            }
        }
    }
    return Err(String::from(""));
}
