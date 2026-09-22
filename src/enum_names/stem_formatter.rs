pub struct StemFormatter {
    num_digits: usize,
    next_num: usize,
}

impl StemFormatter {
    pub fn new(num_stems: usize) -> StemFormatter
    {
        StemFormatter {
            num_digits: ((num_stems + 1) as f64).log10().ceil() as usize,
            next_num: 1,
        }
    }

    pub fn format(&mut self) -> String
    {
        let ret_val = format!("{:0width$}", self.next_num, width=self.num_digits);
        self.next_num += 1;
        ret_val
    }
}