use crate::utils::FixedWidthFormatter;

pub struct StemFormatter {
    num_formatter: FixedWidthFormatter,
    next_num: usize,
}

impl StemFormatter {
    pub fn new(num_stems: usize) -> StemFormatter
    {
        StemFormatter {
            num_formatter: FixedWidthFormatter::new_for_num(num_stems),
            next_num: 1,
        }
    }

    pub fn format(&mut self) -> String
    {
        let ret_val = self.num_formatter.num_0(self.next_num);
        self.next_num += 1;
        ret_val
    }
}