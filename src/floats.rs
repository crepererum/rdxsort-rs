use super::RdxSort;

use std::mem;

impl RdxSort for Vec<f64> {
    fn rdxsort(&mut self) {
        let n = self.len();
        let mut bucket_inf_negative: Vec<f64> = Vec::new();
        let mut bucket_negative: Vec<u64> = Vec::with_capacity(n);
        let mut bucket_zero_negative: Vec<f64> = Vec::new();
        let mut bucket_zero_positive: Vec<f64> = Vec::new();
        let mut bucket_positive: Vec<u64> = Vec::with_capacity(n);
        let mut bucket_inf_positive: Vec<f64> = Vec::new();
        for x in self.iter().cloned() {
            if x.is_normal() {
                if x > 0f64 {
                    bucket_positive.push(unsafe { mem::transmute::<f64, u64>(x) });
                } else {
                    bucket_negative.push(unsafe { mem::transmute::<f64, u64>(x) });
                }
            } else if x == 0f64 {
                bucket_zero_positive.push(x);
            } else if x == -0f64 {
                bucket_zero_negative.push(x);
            } else if x.is_infinite() {
                if x > 0f64 {
                    bucket_inf_positive.push(x);
                } else {
                    bucket_inf_negative.push(x);
                }
            } else {
                panic!("Sorting of NaNs and subnormals is not implemented!");
            }
        }
        bucket_negative.rdxsort();
        bucket_positive.rdxsort();
        self.clear();
        self.append(&mut bucket_inf_negative);
        self.extend(bucket_negative.iter()
                                   .rev()
                                   .cloned()
                                   .map(|x| unsafe { mem::transmute::<u64, f64>(x) }));
        self.append(&mut bucket_zero_negative);
        self.append(&mut bucket_zero_positive);
        self.extend(bucket_positive.iter()
                                   .cloned()
                                   .map(|x| unsafe { mem::transmute::<u64, f64>(x) }));
        self.append(&mut bucket_inf_positive);
    }
}
