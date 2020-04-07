use rstd::prelude::*;
use support::{decl_module, decl_storage, StorageValue};

pub trait Trait: timestamp::Trait {
}

const MAX_TIMESTAMP_SAMPLES: usize = 11;

decl_storage! {
    trait Store for Module<T: Trait> as MTP {
        pub MedianTimePast get(median_time_past): T::Moment;
        pub SampleTimestamps get(sample_timestamps): Vec<T::Moment>;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn on_finalize(_n: T::BlockNumber) {
            let now = <timestamp::Module<T>>::get();
            Self::calculate_mtp(now);
        }
    }
}

impl<T: Trait> Module<T> {
    fn calculate_mtp(time: T::Moment) {
        let mut samples = <SampleTimestamps<T>>::get();
        match samples.len() {
            MAX_TIMESTAMP_SAMPLES => {
                samples.remove(0);
                samples.push(time);
            }
            _ => samples = [time; MAX_TIMESTAMP_SAMPLES].to_vec(),
        }
        <SampleTimestamps<T>>::put(&sample);

        samples.sort();
        <MedianTimePast<T>>::put(samples[MAX_TIMESTAMP_SAMPLES/2]);
    }
}