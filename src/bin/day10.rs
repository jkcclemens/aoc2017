extern crate hex;

const INPUT: &[u8] = &[94, 84, 0, 79, 2, 27, 81, 1, 123, 93, 218, 23, 103, 255, 254, 243];
const INPUT_P2: &str = "94,84,0,79,2,27,81,1,123,93,218,23,103,255,254,243";
const LIST: [u8; 256] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235,236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255];

fn main() {
  println!("{}", p1());
  println!("{}", p2());
}

fn p1() -> usize {
  let mut hasher = Hasher::new(LIST.to_vec(), INPUT.to_vec());
  hasher.do_all_lengths();
  hasher.list[0] as usize * hasher.list[1] as usize
}

fn p2() -> String {
  Hasher::from_str(INPUT_P2).hash()
}

struct Hasher {
  list: Vec<u8>,
  pos: usize,
  skip: usize,
  lengths: Vec<u8>
}

impl Hasher {
  fn new(list: Vec<u8>, lengths: Vec<u8>) -> Hasher {
    Hasher {
      list,
      pos: 0,
      skip: 0,
      lengths
    }
  }

  fn from_str(input: &str) -> Hasher {
    Hasher::new(LIST.to_vec(), input.as_bytes().to_vec())
  }

  /// Calculate the hash of the given input.
  fn hash(mut self) -> String {
    const SUFFIX: &[u8] = &[17, 31, 73, 47, 23];

    // add the suffix on to the current lengths
    self.lengths.extend(SUFFIX.to_vec());

    // do 64 rounds
    for _ in 0..64 {
      self.do_all_lengths();
    }

    // calculate the dense hash
    let dense: Vec<u8> = self.list
      .chunks(16)
      .map(|c| c.into_iter().fold(0, |acc, x| acc ^ x))
      .collect();

    // encode the dense hash into a hex string
    hex::encode(dense)
  }

  fn do_all_lengths(&mut self) {
    for length in self.lengths.clone() {
      self.do_one_length(length);
    }
  }

  fn do_one_length(&mut self, length: u8) {
    let length = length as usize;
    let list_size = self.list.len();
    let bytes: Vec<u8> = self.list.iter()
      .cycle() // make the iterator repeat itself forever
      .skip(self.pos) // skip to current pos
      .take(length) // take this length
      .cloned()
      .collect();
    // write each byte to the list in reverse, wrapping if necessary
    for (i, b) in bytes.into_iter().enumerate() {
      let mut pos = self.pos + (length - i) - 1;
      if pos >= list_size {
        pos %= list_size;
      }
      self.list[pos] = b;
    }
    // update the position, wrapping if necessary
    self.pos += length + self.skip;
    if self.pos >= list_size {
      self.pos %= list_size;
    }
    // increment the skip
    self.skip += 1;
  }
}

#[cfg(test)]
mod test {
  use {p1, p2, Hasher};

  #[test]
  fn solutions() {
    assert_eq!(23715, p1());
    assert_eq!("541dc3180fd4b72881e39cf925a50253", p2());
  }

  #[test]
  fn examples() {
    let mut hasher = Hasher::new(vec![0, 1, 2, 3, 4], vec![3, 4, 1, 5]);
    assert_eq!(0, hasher.pos);
    assert_eq!(0, hasher.skip);

    hasher.do_one_length(3);
    assert_eq!(vec![2, 1, 0, 3, 4], hasher.list);
    assert_eq!(3, hasher.pos);
    assert_eq!(1, hasher.skip);

    hasher.do_one_length(4);
    assert_eq!(vec![4, 3, 0, 1, 2], hasher.list);
    assert_eq!(3, hasher.pos);
    assert_eq!(2, hasher.skip);

    hasher.do_one_length(1);
    assert_eq!(vec![4, 3, 0, 1, 2], hasher.list);
    assert_eq!(1, hasher.pos);
    assert_eq!(3, hasher.skip);

    hasher.do_one_length(5);
    assert_eq!(vec![3, 4, 2, 1, 0], hasher.list);
    assert_eq!(4, hasher.pos);
    assert_eq!(4, hasher.skip);
  }

  #[test]
  fn hashes() {
    assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", Hasher::from_str("").hash());
    assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", Hasher::from_str("AoC 2017").hash());
    assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", Hasher::from_str("1,2,3").hash());
    assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", Hasher::from_str("1,2,4").hash());
  }
}
