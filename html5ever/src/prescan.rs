
enum AbortReason
{
  not enough bytes,
  no encoding detected within the first 1024 bytes
}
pub fn readByte(stream: vec!&[u8]) -> Result<&'static Encoding, AbortReason>{
  let &mut position = stream[0];


  for i in 0..stream.iter() {

    match stream {
      [0x3C, 0x21, 0x2D, 0x3C, .., 0x2D, 0x2D, 0x3C] => {
        //Adding position
        pub fn for_label(label: &[u8]) -> Option<&'static Encoding>{

        }
      },
      [0x3C, 0x4D | 0x6D, 0x45 | 0x65, 0x54 | 0x74, 0x41 | 0x61 ,0x20] => {
        // will include another Match here for attribute
        pub fn for_label(label: &[u8]) -> Option<&'static Encoding>{

        }
      }
      // implementing the AbortReason
    }
  }

}