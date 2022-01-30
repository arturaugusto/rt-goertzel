pub mod filter {
    pub struct Goertzel {
        s_prev: f32,
        s_prev2: f32,
        totalpower: f32,
        n: i32,
        mean: f32,
        mean_prev: f32,

        freq: f32,
        samplef: f32
    }

    impl Goertzel {
        pub fn new(freq: f32, samplef: f32) -> Self {
            Self {
                s_prev: 0.,
                s_prev2: 0.,
                totalpower: 0.,
                n: 0,
                mean: 0.,
                mean_prev: 0.,

                freq: freq,
                samplef: samplef
            }
        }
        
        pub fn filter (&mut self, sample: f32) -> f32 {
            // real time mean
            // https://dsp.stackexchange.com/questions/811/determining-the-mean-and-standard-deviation-in-real-time
            self.mean_prev = self.mean;
            self.n = self.n + 1;
            self.mean = self.mean + (sample-self.mean)/self.n as f32;
            let x: f32 = sample - self.mean;

            let normalizedfreq: f32 = self.freq/self.samplef;
            let coeff: f32 = 2.*(2.*3.1416*normalizedfreq).cos();
            let s: f32 = x + coeff * self.s_prev - self.s_prev2;
            self.s_prev2 = self.s_prev;
            self.s_prev = s;

            let power: f32 = self.s_prev2*self.s_prev2+self.s_prev*self.s_prev-coeff*self.s_prev*self.s_prev2;
            self.totalpower = self.totalpower + x*x;
            if self.totalpower < 0. {
                self.totalpower = 1.
            }
            return power / self.totalpower / self.n as f32
        }
    }
}
