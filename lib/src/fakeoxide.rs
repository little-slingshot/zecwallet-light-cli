#![allow(unused)]

pub mod crypto {
  fn hello_crypto(){
    println!("hello_crypto()");
  }


  pub mod secretbox {

    pub fn open(enc_seed:&[u8], nonce:&Nonce, key: &Key) -> Result<Vec<u8>, String> {
        // Err(String::from("Not implemented"))
        // return [1,2,3].to_vec()
        Ok([0x42;32].to_vec())
    }

    pub fn gen_nonce() -> Nonce {
        let my_nonce =  Nonce {
           name: String::from("Bob from get_nonce()"),
           age: 42,
       };

        my_nonce
    }

    // fn seal(seed_bytes: &[u8], nonce: &Nonce, key: &Key) -> cipher {
    pub fn seal(seed_bytes: &[u8], nonce: &Nonce, key: &Key) -> Vec<u8> {
        return [1,2,3].to_vec()
    }

    // =======================
    // Structs
    // =======================

    // -----------------------
    // Nonce
    // -----------------------
    pub struct Nonce {
      name: String,
      age: u32,
    }

    // impl std::vec::ToVec<Vec<u8>> for Nonce{
    //   fn to_vec(&self) -> Vec<u8> {
        
    //   }
    // }    

    // impl AsRef<[u8]> for Nonce {
    //   fn as_ref(&self) -> &[u8]{
    //     // return random slice? but this is clice on the heap?
    //     let a = [1,2,3];
    //     &a
    //   }
    // }

    impl AsRef<Nonce> for Nonce {
      fn as_ref(&self) -> &Nonce {
        // return random slice? but this is clice on the heap?
        self
      }
    }

    impl Nonce{
      pub fn from_slice(slice: &[u8]) -> Result<Nonce, String> {
          let my_nonce =  Nonce {
             name: String::from("Bob"),
             age: 42,
         };

          Ok(my_nonce)
      }

      pub fn to_vec(&self) -> Vec<u8> {
        // Vec::new()
        // vec![1,2,3]
        vec![0x20; 32]
      }      
    }

    // -----------------------
    // Key
    // -----------------------
    pub struct Key {
      name: String,
      age: u32,
    }

    impl Key {
      pub fn hello_key(&self){
        println!("Hello {} of age {}", self.name, self.age)
      }

      pub fn from_slice(slice: &[u8]) -> Result<Key, String>{
          Ok(Key {
              name: String::from("This is key"),
              age: 42,
          })
      }
    }

  }
}
