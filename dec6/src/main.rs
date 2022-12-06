fn main() {

    // Code by https://chat.openai.com/chat

    let buffer = include_str!("../input-full.txt");

    // Loop through the buffer, looking for the first start-of-packet marker
    let mut result = 0;
    for i in 0..buffer.len()-3 {
        let four_chars = &buffer[i..i+4];
        let mut chars = [false; 256];
        let mut marker = true;

        for c in four_chars.chars() {
            let index = c as usize;
            if chars[index] {
                marker = false;
                break;
            }
            chars[index] = true;
        }

        if marker {
            result = i + 4;
            break;
        }
    }

    // Print the result
    println!("{}", result);


    // part 2

      // Loop through the buffer, looking for the first start-of-message marker
      let mut result = 0;
      for i in 0..buffer.len()-13 {
          let fourteen_chars = &buffer[i..i+14];
          let mut chars = [false; 256];
          let mut marker = true;
  
          for c in fourteen_chars.chars() {
              let index = c as usize;
              if chars[index] {
                  marker = false;
                  break;
              }
              chars[index] = true;
          }
  
          if marker {
              result = i + 14;
              break;
          }
      }
  
      // Print the result
      println!("{}", result);
}