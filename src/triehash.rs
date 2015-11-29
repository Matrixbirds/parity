//! Generete trie root

//use std::collections::BTreeMap;
//use std::cmp;
//use hash::*;
//use sha3::*;
//use rlp;
//use rlp::RlpStream;
//use vector::SharedPrefix;

///// Hex-prefix Notation. First nibble has flags: oddness = 2^0 & termination = 2^1.
/////
///// The "termination marker" and "leaf-node" specifier are completely equivalent.
///// 
///// Input values are in range `[0, 0xf]`.
///// 
///// ```markdown
/////  [0,0,1,2,3,4,5]   0x10012345 // 7 > 4
/////  [0,1,2,3,4,5]     0x00012345 // 6 > 4
/////  [1,2,3,4,5]       0x112345   // 5 > 3
/////  [0,0,1,2,3,4]     0x00001234 // 6 > 3
/////  [0,1,2,3,4]       0x101234   // 5 > 3
/////  [1,2,3,4]         0x001234   // 4 > 3
/////  [0,0,1,2,3,4,5,T] 0x30012345 // 7 > 4
/////  [0,0,1,2,3,4,T]   0x20001234 // 6 > 4
/////  [0,1,2,3,4,5,T]   0x20012345 // 6 > 4
/////  [1,2,3,4,5,T]     0x312345   // 5 > 3
/////  [1,2,3,4,T]       0x201234   // 4 > 3
///// ``` 
/////  
///// ```rust
/////	extern crate ethcore_util as util;
/////	use util::triehash::*;
/////  
/////	fn main() {
/////		let v = vec![0, 0, 1, 2, 3, 4, 5];
/////		let e = vec![0x10, 0x01, 0x23, 0x45];
/////		let h = hex_prefix_encode(&v, false);
/////		assert_eq!(h, e);
/////		
/////		let v = vec![0, 1, 2, 3, 4, 5];
/////		let e = vec![0x00, 0x01, 0x23, 0x45];
/////		let h = hex_prefix_encode(&v, false);
/////		assert_eq!(h, e);
/////		
/////		let v = vec![0, 1, 2, 3, 4, 5];
/////		let e = vec![0x20, 0x01, 0x23, 0x45];
/////		let h = hex_prefix_encode(&v, true);
/////		assert_eq!(h, e);
/////		
/////		let v = vec![1, 2, 3, 4, 5];
/////		let e = vec![0x31, 0x23, 0x45];
/////		let h = hex_prefix_encode(&v, true);
/////		assert_eq!(h, e);
/////		
/////		let v = vec![1, 2, 3, 4];
/////		let e = vec![0x00, 0x12, 0x34];
/////		let h = hex_prefix_encode(&v, false);
/////		assert_eq!(h, e);
/////		
/////		let v = vec![4, 1];
/////		let e = vec![0x20, 0x41];
/////		let h = hex_prefix_encode(&v, true);
/////		assert_eq!(h, e);
/////	}
///// ```
/////  
//pub fn hex_prefix_encode(nibbles: &[u8], leaf: bool) -> Vec<u8> {
	//let inlen = nibbles.len();
	//let oddness_factor = inlen % 2;
	//// next even number divided by two
	//let reslen = (inlen + 2) >> 1;
	//let mut res = vec![];
	//res.reserve(reslen);

	//let first_byte = {
		//let mut bits = ((inlen as u8 & 1) + (2 * leaf as u8)) << 4;
		//if oddness_factor == 1 {
			//bits += nibbles[0];
		//}
		//bits
	//};

	//res.push(first_byte);

	//let mut offset = oddness_factor;	
	//while offset < inlen {
		//let byte = (nibbles[offset] << 4) + nibbles[offset + 1];
		//res.push(byte);
		//offset += 2;
	//}

	//res
//}

///// Converts slice of bytes to nibbles.
///// 
///// ```rust
/////	extern crate ethcore_util as util;
/////	use util::triehash::*;
/////
/////	fn main () {
/////		let v = vec![0x31, 0x23, 0x45];
/////		let e = vec![3, 1, 2, 3, 4, 5];
/////		assert_eq!(as_nibbles(&v), e);
/////		
/////		// A => 65 => 0x41 => [4, 1]
/////		let v: Vec<u8> = From::from("A");
/////		let e = vec![4, 1]; 
/////		assert_eq!(as_nibbles(&v), e);
/////	}
///// ```
//pub fn as_nibbles(bytes: &[u8]) -> Vec<u8> {
	//let mut res = vec![];
	//res.reserve(bytes.len() * 2);
	//for i in 0..bytes.len() {
		//res.push(bytes[i] >> 4);
		//res.push((bytes[i] << 4) >> 4);
	//}
	//res
//}

//#[derive(Debug)]
//pub struct NibblePair {
	//nibble: Vec<u8>,
	//data: Vec<u8>
//}

//impl NibblePair {
	//pub fn new(nibble: Vec<u8>, data: Vec<u8>) -> NibblePair {
		//NibblePair {
			//nibble: nibble,
			//data: data
		//}
	//}

	//pub fn new_raw(to_nibble: Vec<u8>, data: Vec<u8>) -> NibblePair {
		//NibblePair::new(as_nibbles(&to_nibble), data)
	//}
//}

//pub fn ordered_trie_root(data: Vec<Vec<u8>>) -> H256 {
	//let vec: Vec<NibblePair> = data
		//// first put elements into btree to sort them by nibbles
		//// optimize it later
		//.into_iter()
		//.fold(BTreeMap::new(), | mut acc, vec | {
			//let len = acc.len();
			//acc.insert(as_nibbles(&rlp::encode(&len)), vec);
			//acc
		//})
		//// then move them to a vector
		//.into_iter()
		//.map(|(k, v)| NibblePair::new(k, v) )
		//.collect();

	//hash256(&vec)
//}

//pub fn hash256(vec: &[NibblePair]) -> H256 {
	//let out = match vec.len() {
		//0 => rlp::encode(&""),
		//_ => {
			//let mut stream = RlpStream::new();
			//hash256rlp(&vec, 0, &mut stream);
			//stream.out().unwrap()
		//}
	//};
	
	//println!("out: {:?}", out);
	//out.sha3()
//}

//fn hash256rlp(vec: &[NibblePair], pre_len: usize, stream: &mut RlpStream) {
	//match vec.len() {
		//0 => { 
			//stream.append(&""); 
		//},
		//1 => {
			//stream.append_list(2);
			//stream.append(&hex_prefix_encode(&vec[0].nibble[pre_len..], true));
			//stream.append(&vec[0].data);
		//},
		//_ => {
			//let shared_prefix = vec.iter()
				//// skip first element
				//.skip(1)
				//// get minimum number of shared nibbles between first and each successive
				//.fold(usize::max_value(), | acc, pair | { 
					//cmp::min(vec[0].nibble.shared_prefix_len(&pair.nibble), acc)
				//});

			//match shared_prefix > pre_len {
				//true => {
					//stream.append_list(2);
					//stream.append(&hex_prefix_encode(&vec[0].nibble[pre_len..shared_prefix], false));
					//hash256aux(vec, shared_prefix, stream);
				//},
				//false => {
					//stream.append_list(17);

					//// every nibble is longer then previous one
					//let iter = vec.iter()
						//// move to first element with len different then pre_len
						//.take_while(| pair | { pair.nibble.len() == pre_len });

					//let mut begin = iter.count();

					//for i in 0..16 {
						//// cout how many successive elements have same next nibble
						//let len = vec[begin..].iter()
							//.map(| pair | pair.nibble[pre_len] )
							//.take_while(|&q| q == i).count();
							
						//match len {
							//0 => { stream.append(&""); },
							//_ => hash256aux(&vec[begin..begin + len], pre_len + 1, stream)
						//}
						//begin += len;
					//}

					//match pre_len == vec[0].nibble.len() {
						//true => stream.append(&vec[0].data),
						//false => stream.append(&"")
					//};
				//}
			//}
		//}
	//};
//}

//fn hash256aux(vec: &[NibblePair], pre_len: usize, stream: &mut RlpStream) {
	//let mut s = RlpStream::new();
	//hash256rlp(vec, pre_len, &mut s);
	//let out = s.out().unwrap();
	//match out.len() {
		//0...31 => stream.append_raw(&out, 1),
		//_ => stream.append(&out.sha3())
	//};
//}

//#[cfg(test)]
//mod tests {
	//use std::str::FromStr;
	//use rustc_serialize::hex::FromHex;
	//use hash::*;
	//use triehash::*;

	//#[test]
	//fn empty_trie_root() {
		//assert_eq!(hash256(&vec![]), H256::from_str("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421").unwrap());
	//}

	//#[test]
	//fn single_trie_item() {

		//let v = vec![
			//NibblePair::new_raw(From::from("A"),
								//From::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"))
		//];
		//println!("{:?}", v);
		//assert_eq!(hash256(&v), H256::from_str("d23786fb4a010da3ce639d66d5e904a11dbc02746d1ce25029e53290cabf28ab").unwrap());
	//}

	//#[test]
	//fn test_trie_root() {
		//let v = vec![
			//NibblePair::new_raw("0000000000000000000000000000000000000000000000000000000000000045".from_hex().unwrap(),
								//"22b224a1420a802ab51d326e29fa98e34c4f24ea".from_hex().unwrap()),

			//NibblePair::new_raw("0000000000000000000000000000000000000000000000000000000000000046".from_hex().unwrap(),
								//"67706c2076330000000000000000000000000000000000000000000000000000".from_hex().unwrap()),

			//NibblePair::new_raw("000000000000000000000000697c7b8c961b56f675d570498424ac8de1a918f6".from_hex().unwrap(),
								//"6f6f6f6820736f2067726561742c207265616c6c6c793f000000000000000000".from_hex().unwrap()),

			//NibblePair::new_raw("0000000000000000000000007ef9e639e2733cb34e4dfc576d4b23f72db776b2".from_hex().unwrap(),
								//"4655474156000000000000000000000000000000000000000000000000000000".from_hex().unwrap()),

			//NibblePair::new_raw("000000000000000000000000ec4f34c97e43fbb2816cfd95e388353c7181dab1".from_hex().unwrap(),
								//"4e616d6552656700000000000000000000000000000000000000000000000000".from_hex().unwrap()),

			//NibblePair::new_raw("4655474156000000000000000000000000000000000000000000000000000000".from_hex().unwrap(),
								//"7ef9e639e2733cb34e4dfc576d4b23f72db776b2".from_hex().unwrap()),

			//NibblePair::new_raw("4e616d6552656700000000000000000000000000000000000000000000000000".from_hex().unwrap(),
								//"ec4f34c97e43fbb2816cfd95e388353c7181dab1".from_hex().unwrap()),

			//NibblePair::new_raw("6f6f6f6820736f2067726561742c207265616c6c6c793f000000000000000000".from_hex().unwrap(),
								//"697c7b8c961b56f675d570498424ac8de1a918f6".from_hex().unwrap())
		//];

		//let root = H256::from_str("9f6221ebb8efe7cff60a716ecb886e67dd042014be444669f0159d8e68b42100").unwrap();

		//let res = hash256(&v);

		//println!("{:?}", res);
		////assert!(false);
	//}
//}
