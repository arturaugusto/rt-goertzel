#[allow(unused_imports)]
#[allow(dead_code)]

mod goertzel;

#[cfg(test)]
mod tests {
  use crate::goertzel::filter::*;
  #[test]
  fn exploration() {
    // assert_eq!(2 + 2, 4);

    let sig_freq = 9.;
    let sample_freq = 100.;
    let dt = 1./sample_freq;
    let dc_offset = 10.;
    
    let mut g = Goertzel::new(10., sample_freq);
    
    for n in 0..100 {
      let sample =  (2. * 3.1416 * sig_freq * (n as f32) * dt).sin() * 10. + dc_offset;
      // note: run "cargo watch -cx 'test -- --nocapture'" to get output
      println!("{:?}", g.filter(sample));
      // println!("{:?}", sample);
    }
    // println!("---second tone---");
    // for n in 0..100 {
    //   let sample =  (2. * 3.1416 * (sig_freq * 2.) * (n as f32) * dt).sin() * 10.;
    //   // note: run "cargo watch -cx 'test -- --nocapture'" to get output
    //   println!("{:?}", g.filter(sample));
    //   // println!("{:?}", sample);
    // }




  }
}
