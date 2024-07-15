const BLOCK_SIZE: usize = 8; // Size of each block in bytes
const HASH_SIZE: usize = 8; // Size of the hash code in bytes
const INITIAL_STATE: [u8; HASH_SIZE] = [255, 100, 211, 37, 112, 167, 41, 37]; // randomly generated initial state

struct EvanHash {
    state: [u8; HASH_SIZE],
    block: [u8; BLOCK_SIZE],
    length: usize,
    data: Vec<u8>,
}

impl EvanHash {
    fn new(data: &[u8]) -> Self {
      let hasher = EvanHash {
        state: INITIAL_STATE,
        block: [0u8; BLOCK_SIZE],
        length: 0,
        data: Vec::from(data),
      };
      println!("{:<12} {:?}", "state:", hasher.state);
      hasher
    }

    fn update(&mut self) {
        self.block = [0u8; BLOCK_SIZE];
        let remaining = self.data.len() - self.length;
        let end = self.length + remaining.min(BLOCK_SIZE);

        // fill the block from input data
        self.block[..remaining.min(BLOCK_SIZE)].copy_from_slice(&self.data[self.length..end]);

        // if this is the last block append data.len()
        if remaining < BLOCK_SIZE {
            self.block[remaining] = self.data.len() as u8;
        }

        println!("block{:<7} {:?}", format!("{}:", self.length / BLOCK_SIZE), self.block);
        self.process_block();

        self.length += BLOCK_SIZE;
    }

    fn finalize(self) -> [u8; HASH_SIZE] {
        self.state
    }

    fn process_block(&mut self) {
      let mut temp_states: Vec<[u8; BLOCK_SIZE]> = Vec::new();
      temp_states.push(self.state);
  
      for i in 0..13 {
          let mut temp_block = self.block;
          let block_as_u64: u64 = u64::from_be_bytes(temp_block.try_into().unwrap()); // Convert block to u64 for bitwise operations
  
          let modified_block_as_u64 = match i {
              0 => !block_as_u64, // invert bits
              1 => block_as_u64.rotate_right(17),
              2 => block_as_u64.rotate_right(23),
              3 => block_as_u64.rotate_right(32),
              4 => block_as_u64.reverse_bits(),
              5 => block_as_u64 ^ 0xAAAAAAAAAAAAAAAA, // Toggle every other bit starting with the first bit
              6 => block_as_u64.rotate_right(43),
              7 => block_as_u64 ^ 0x5555555555555555, // Toggle every other bit skipping the first bit
              8 => block_as_u64.rotate_right(51),
              9 => block_as_u64.rotate_right(13),
              10 => block_as_u64.rotate_right(37),
              11 => block_as_u64.rotate_right(19),
              12 => block_as_u64.rotate_right(7),
              _ => block_as_u64,
          };
  
          temp_block = modified_block_as_u64.to_be_bytes(); // Convert back to byte array
          temp_states.push(temp_block);
      }
  
      // XOR all iterations together
      for i in 0..BLOCK_SIZE {
          self.state[i] = !temp_states.iter().fold(0xff, |acc, x| acc ^ x[i]);
      }
    }
    
    fn hash(data: &[u8]) -> [u8; HASH_SIZE] {
      let mut hasher = EvanHash::new(&data);
      while hasher.length <= hasher.data.len() {
        hasher.update();
      }
      hasher.finalize()
    }
}

fn main() {
  let input = "Hello, World!";
  println!("{:<12} \"{}\"", "input:", input);
  let input = input.as_bytes();
  println!("{:<12} {:?}", "bytes input:", input);
  let hash_output = EvanHash::hash(input);
  println!("{:<12} {:?}", "hash output:", hash_output);
  let hex_output = hex::encode(hash_output);
  println!("{:<12} {:?}", "hex output:", hex_output);
}