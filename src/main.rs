use std::env;

const BLOCK_SIZE: usize = 8; // Size of each block in bytes
const HASH_SIZE: usize = 8; // Size of the hash code in bytes
const INITIAL_STATE: [u8; HASH_SIZE] = [255, 100, 211, 37, 112, 167, 41, 37]; // randomly generated initial state

struct EvanHash {
    state: [u8; HASH_SIZE],
    block: [u8; BLOCK_SIZE],
    length: usize,
    data: Vec<u8>,
    debug: bool,
}

impl EvanHash {
    fn new(data: &[u8], debug: bool) -> Self {
      let hasher = EvanHash {
        state: INITIAL_STATE,
        block: [0u8; BLOCK_SIZE],
        length: 0,
        data: Vec::from(data),
        debug: debug,
      };
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

        debug_print(self.debug, format!("block{:<7} {:?}", format!("{}:", self.length / BLOCK_SIZE), self.block));
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
    
    fn hash(data: &[u8], debug: bool) -> [u8; HASH_SIZE] {
      let mut hasher = EvanHash::new(&data, debug);
      debug_print(debug, format!("{:<12} {:?}", "state:", hasher.state));
      while hasher.length <= hasher.data.len() {
        hasher.update();
        debug_print(debug, format!("{:<12} {:?}", "state:", hasher.state));
      }
      hasher.finalize()
    }
}

fn debug_print(debug: bool, message: String) {
  if debug {
      println!("{}", message);
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
      eprintln!("Usage: <input> [--debug]");
      std::process::exit(1);
  }

  let input = &args[1];
  let debug = args.iter().any(|arg| arg == "--debug");

  debug_print(debug, format!("{:<12} \"{}\"", "input:", input));
  let input = input.as_bytes();
  debug_print(debug, format!("{:<12} {:?}", "bytes input:", input));
  let hash_output = EvanHash::hash(input, debug);
  debug_print(debug, format!("{:<12} {:?}", "hash output:", hash_output));
  println!("{}", hex::encode(hash_output));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_empty_input() {
        let data = b"";
        let expected_hash = INITIAL_STATE;
        let hash_output = EvanHash::hash(data);
        assert_eq!(hash_output, expected_hash, "Hash of empty input does not match expected output");
    }

    #[test]
    fn test_hash_single_block() {
        let data = b"ABCDEFG"; // Exactly one block, including length byte
        let expected_hash = [72, 29, 111, 3, 59, 135, 57, 163];
        let hash_output = EvanHash::hash(data);
        assert_eq!(hash_output, expected_hash, "Hash of single block input does not match expected output");
    }

    #[test]
    fn test_hash_multi_block() {
        let data = b"The quick brown fox jumps over the lazy dog"; // Longer than one block
        let expected_hash = [194, 19, 66, 50, 112, 155, 159, 251];
        let hash_output = EvanHash::hash(data);
        assert_eq!(hash_output, expected_hash, "Hash of multi-block input does not match expected output");
    }

    #[test]
    fn test_hash_with_padding() {
        let data = b"Short"; // Requires padding to fill a block
        let expected_hash = [130, 173, 146, 187, 206, 95, 78, 196];
        let hash_output = EvanHash::hash(data);
        assert_eq!(hash_output, expected_hash, "Hash with padding does not match expected output");
    }

    #[test]
    fn test_hash_identical_inputs() {
        let data1 = b"Identical";
        let data2 = b"Identical";
        let hash_output1 = EvanHash::hash(data1);
        let hash_output2 = EvanHash::hash(data2);
        assert_eq!(hash_output1, hash_output2, "Hashes of identical inputs do not match");
    }

    #[test]
    fn test_hash_different_inputs() {
        let data1 = b"Different";
        let data2 = b"Inputs";
        let hash_output1 = EvanHash::hash(data1);
        let hash_output2 = EvanHash::hash(data2);
        assert_ne!(hash_output1, hash_output2, "Hashes of different inputs should not match");
    }
}