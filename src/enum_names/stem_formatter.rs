use crate::utils::FixedWidthFormatter;

pub struct StemNumFormatter {
    num_formatter: FixedWidthFormatter,
    next_num: usize,
}

impl StemNumFormatter {
    pub fn new(start_num: usize, max_stem_number: usize, min_width: usize) -> StemNumFormatter
    {
        StemNumFormatter {
            num_formatter: FixedWidthFormatter::new_for_num(max_stem_number, min_width),
            next_num: start_num,
        }
    }

    pub fn format(&mut self) -> String
    {
        let ret_val = self.num_formatter.num_0(self.next_num);
        self.next_num += 1;
        ret_val
    }
}