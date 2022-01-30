fn main() {
  // assert_eq!(2 + 2, 4);

  let signal_freq = 100.;
  let sample_freq = 1000.;
  let mut g = Goertzel::new(signal_freq, sample_freq);
  
  for n in 0..100 {
    let sample =  (n as f32 * 1./sample_freq).sin();
    println!("{:?}", g.filter(sample));
  }
}