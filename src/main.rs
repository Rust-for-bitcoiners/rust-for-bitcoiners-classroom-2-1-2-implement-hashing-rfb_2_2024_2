const BLOCK_SIZE: usize = 8; // Size of each block in bytes
const HASH_SIZE: usize = 8; // Size of the hash code in bytes

struct EvanHash {
    state: [u8; HASH_SIZE],
    block: [u8; BLOCK_SIZE],
    length: usize,
    data: Vec<u8>,
}

impl EvanHash {
    fn new(data: &[u8]) -> Self {
      let hasher = EvanHash {
        state: [0u8; HASH_SIZE],
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

        // TODO update state:
        // loop 13 times
        // if i == 0, nand with 0xff
        // else if i % 2 == 0, right shift by 1
        // if i % 3 == 0, left shift by 2
        // if i % 4 == 0, swap first half with second half
        // if i % 5 == 0, reverse
        // if i % 6 == 0, xor with 0xff
        // if i % 7 == 0, right shift by 2
        // if i % 8 == 0, left shift by 1
        // if i % 9 == 0, xor with 0x0f
        // if i % 11 == 0, right shift by 3
        // if i % 13 == 0, left shift by 3
        // set state = NAND all iterations together with initial state
    }

    fn finalize(self) -> [u8; HASH_SIZE] {
        self.state
    }

    fn process_block(&mut self) {
        for i in 0..HASH_SIZE {
          // right shift by 1
          self.state[i] ^= self.block[i];
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


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_pad_function() {
    //     let data = b"Hello World!";
    //     let padded_data = EvanHash::pad(data);
    //     let expected_data = vec![72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12];
    //     assert_eq!(padded_data, expected_data, "Padded data does not match expected output");
    // }
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
