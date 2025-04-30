pub mod consts;
pub use consts::{ROUNDS, TOTAL_ROUNDS};

/// # Example
/// ```
/// use sure25::Hasher;
///
/// let mut hasher = Hasher::new();
/// hasher.update(b"hello world");
/// assert_eq!(
///     hasher.finalize(),
///     vec![
///         0xE8, 0x1C, 0xCA, 0x57, 0xF2, 0xD5, 0xDB, 0xCA, 0xEE, 0x19, 0x6A, 0xAC, 0xDD, 0x04,
///         0xC3, 0x7E, 0x61, 0x62, 0x58, 0x84, 0x70, 0xDA, 0x70, 0xAF, 0x3F
///     ]
/// );
/// ```
#[derive(Clone, Debug, Default)]
pub struct Hasher {
    state: [u8; 25],
    round: usize,
    index: usize,
}

impl Hasher {
    pub fn new() -> Hasher {
        let mut hasher = Hasher::default();
        hasher.initialize();
        hasher
    }

    fn initialize(&mut self) {
        self.state = [0u8; 25];
        self.tick();
    }

    fn tick(&mut self) {
        for index in 0..25 {
            self.index += 1;
            let flip = if index > 0 && index % 2 == 1 {
                (self.index + self.round) as u8 % 0x25
            } else {
                (self.index + self.round) as u8 % u8::MAX
            };
            let rotate = if index > 0 && index % 2 == 1 {
                (self.index + self.round) as u32 % 0x25
            } else {
                (self.index + self.round) as u32 % u32::MAX
            };
            let rindex = if index == 0 { 0 } else { index % 20 };
            self.state[index] = self.state[index]
                ^ ROUNDS[self.round][rindex]
                ^ self.state[index].rotate_right(rotate)
                ^ flip;
            self.next_round()
        }
    }

    fn next_round(&mut self) {
        self.round = if self.round > TOTAL_ROUNDS {
            self.round % TOTAL_ROUNDS
        } else {
            self.round + 1
        };
    }

    pub fn finalize(&self) -> Vec<u8> {
        self.state.to_vec()
    }

    pub fn update(&mut self, bytes: &[u8]) {
        if bytes.is_empty() {
            return;
        }
        for (index, (state_byte, incoming_byte)) in
            bytes.iter().zip(self.state.clone().iter()).enumerate()
        {
            let rindex = if index == 0 { 0 } else { index % 20 };
            self.state[index] = state_byte ^ ROUNDS[self.round][rindex] ^ incoming_byte;
            self.next_round()
        }
        self.tick()
    }
}

#[test]
fn test_hasher_initial_state() {
    let mut hasher = Hasher::new();
    assert_eq!(
        hex::encode(hasher.finalize()),
        "ad9b133fd2618159c6b4478d7154625a35ce275c554075d0fe"
    );
    hasher.update(&[0u8; 0]);
    assert_eq!(
        hex::encode(hasher.finalize()),
        "ad9b133fd2618159c6b4478d7154625a35ce275c554075d0fe"
    );
    hasher.update(&[0u8; 1]);
    assert_eq!(
        hex::encode(hasher.finalize()),
        "f1b509eff4ac6e7af9218ed1c760d58257a801511178f62578"
    );
    hasher.update(&[0u8; 25]);
    assert_eq!(
        hex::encode(hasher.finalize()),
        "b98a9f1c3fbcfd6d275aaf25a83d824c5153cd16fa6353b59b"
    );
}

#[test]
fn test_hello_world() {
    let mut hasher = Hasher::new();
    hasher.update(b"hello world");
    assert_eq!(
        hex::encode(hasher.finalize()),
        "e81cca57f2d5dbcaee196aacdd04c37e6162588470da70af3f"
    );
}

#[test]
fn test_hasher_consts() {
    let mut hasher = Hasher::new();
    hasher.update(&std::fs::read("sure25/consts.rs").unwrap());
    assert_eq!(
        hex::encode(hasher.finalize()),
        "6db8546cafca40caef0a4d73aec595878cc5e598794fa5acbd"
    );
}
