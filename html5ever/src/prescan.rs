
pub fn readByte(stream: &[u8]) {
  let &mut position = stream[0];

  for i in 0..stream.len()-1 {
    if stream[0..3] == [0x3C, 0x21, 0x2D, 0x3C] AND stream[-3]==0x2D AND stream[-2]==0x2D AND stream[-1]==0x3C 
    {
      position = stream[-1];
    }
    if stream[0..1]==[0x3C, 0x4D] OR stream[0..1]==[0x6D, 0x45] OR stream[0..1]==[0x65, 0x54] OR stream[0..1]==[0x74, 0x41] OR stream[0..1]==[0x61,0x2F]
    {
      position = stream[1];
      let mut attribute_list = vec![];
      let mut got_pragma = false;
      let mut need_pragma = charset; // Not sure about this.
      
      // Sutck on the HTML.Spec - Prescan a byte stream, Step 2-6: Attributes
    }
  }

}