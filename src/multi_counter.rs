pub struct MultiCounter {
    counter_maxes: Vec<usize>,
    require_simultaneous_completion: bool,
    _counter_values: Vec<usize>,
    _counter_complete: Vec<bool>,
}

impl MultiCounter {
    pub fn new(counter_maxes: Vec<usize>, require_simultaneous_completion: bool) -> MultiCounter {
        return MultiCounter {
            counter_maxes: counter_maxes.clone(),
            require_simultaneous_completion: require_simultaneous_completion,
            _counter_values: counter_maxes.iter().map(|_i| 0).collect(),
            _counter_complete: counter_maxes.iter().map(|_i| false).collect(),
        };
    }

    /// Returns the current values of the counters.
    pub fn get_values(&self) -> Vec<usize> {
        self._counter_values.clone()
    }

    /// Checks if all counters are complete based on the mode.
    pub fn check_complete(&self) -> bool {
        match self.require_simultaneous_completion {
            true => {
                self._counter_values.iter().all(|&value| value == 0)
                    && self._counter_complete.iter().all(|&complete| complete)
            }
            false => self._counter_complete.iter().all(|&complete| complete),
        }
    }

    /// Increments the counter values and returns the new state if not complete, otherwise None.
    pub fn increment(&mut self) {
        let values: Vec<usize> = self
            .get_values()
            .into_iter()
            .enumerate()
            .map(|(index, value)| {
                if value == self.counter_maxes[index] - 1 {
                    self._counter_complete[index] = true;
                    return 0;
                } else {
                    return value + 1;
                }
            })
            .collect();
        self._counter_values = values;
    }
}

impl Iterator for MultiCounter {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.check_complete() {
            true => return None,
            false => {
                let output = Some(self.get_values());
                self.increment();
                return output;
            }
        }
    }
}

#[cfg(test)]
mod tests_for_multicounter {
    use super::*;

    #[test]
    fn test_initialization() {
        let counter = MultiCounter::new(vec![3, 5], true);
        assert_eq!(counter.counter_maxes, vec![3, 5]);
        assert_eq!(counter.require_simultaneous_completion, true);
        assert_eq!(counter._counter_values, vec![0, 0]);
        assert_eq!(counter._counter_complete, vec![false, false]);

        let counter = MultiCounter::new(vec![2, 4, 6], false);
        assert_eq!(counter.counter_maxes, vec![2, 4, 6]);
        assert_eq!(counter.require_simultaneous_completion, false);
        assert_eq!(counter._counter_values, vec![0, 0, 0]);
        assert_eq!(counter._counter_complete, vec![false, false, false]);
    }

    #[test]
    fn test_increment() {
        let mut counter = MultiCounter::new(vec![2, 3], false);

        // check initial values
        assert_eq!(counter._counter_values, vec![0, 0]);
        assert_eq!(counter._counter_complete, vec![false, false]);
        // increment and then check values and completes
        counter.increment();
        assert_eq!(counter._counter_values, vec![1, 1]);
        assert_eq!(counter._counter_complete, vec![false, false]);
        counter.increment();
        assert_eq!(counter._counter_values, vec![0, 2]);
        assert_eq!(counter._counter_complete, vec![true, false]);
        counter.increment();
        assert_eq!(counter._counter_values, vec![1, 0]);
        assert_eq!(counter._counter_complete, vec![true, true]);
        counter.increment();
        assert_eq!(counter._counter_values, vec![0, 1]);
        assert_eq!(counter._counter_complete, vec![true, true]);
        counter.increment();
        assert_eq!(counter._counter_values, vec![1, 2]);
        assert_eq!(counter._counter_complete, vec![true, true]);
        counter.increment();
        assert_eq!(counter._counter_values, vec![0, 0]);
        assert_eq!(counter._counter_complete, vec![true, true]);
    }

    #[test]
    fn test_get_values() {
        let counter = MultiCounter::new(vec![4, 5], false);
        assert_eq!(counter.get_values(), vec![0, 0])
    }

    #[test]
    fn test_check_complete_when_requires_simultaneous_is_true() {
        let mut counter = MultiCounter::new(vec![2, 3], true);
        assert_eq!(counter.check_complete(), false);

        // [1 , 1] [false, false]
        counter.increment();
        assert_eq!(counter.check_complete(), false);

        // [0 , 2] [true, false]
        counter.increment();
        assert_eq!(counter.check_complete(), false);

        // [1 , 0] [true, true]
        counter.increment();
        assert_eq!(counter.check_complete(), false);

        // [0 , 1] [true, true]
        counter.increment();
        assert_eq!(counter.check_complete(), false);

        // [1 , 2] [true, true]
        counter.increment();
        assert_eq!(counter.check_complete(), false);

        // [0 , 0] [true, true]
        counter.increment();
        assert_eq!(counter.check_complete(), true);

        // [1 , 1] [true, true]
        counter.increment();
        assert_eq!(counter.check_complete(), false);
    }

    #[test]
    fn test_check_complete_when_requires_simultaneous_is_false() {
        let mut counter = MultiCounter::new(vec![2, 3], false);
        assert_eq!(counter.check_complete(), false);

        // [1 , 1] [false, false]
        counter.increment();
        assert_eq!(counter.check_complete(), false);

        // [0 , 2] [true, false]
        counter.increment();
        assert_eq!(counter.check_complete(), false);

        // [1 , 0] [true, true]
        counter.increment();
        assert_eq!(counter.check_complete(), true);

        // [0 , 1] [true, true]
        counter.increment();
        assert_eq!(counter.check_complete(), true);

        // [1 , 2] [true, true]
        counter.increment();
        assert_eq!(counter.check_complete(), true);

        // [0 , 0] [true, true]
        counter.increment();
        assert_eq!(counter.check_complete(), true);

        // [1 , 1] [true, true]
        counter.increment();
        assert_eq!(counter.check_complete(), true);
    }

    #[test]
    fn test_iterator_when_requires_simultaneous_is_false() {
        let counter = MultiCounter::new(vec![3, 5], false);
        let output: Vec<_> = counter.into_iter().collect();

        assert_eq!(output.len(), 5);
        assert_eq!(output[0], vec![0, 0]);
        assert_eq!(output[1], vec![1, 1]);
        assert_eq!(output[2], vec![2, 2]);
        assert_eq!(output[3], vec![0, 3]);
        assert_eq!(output[4], vec![1, 4]);
    }

    #[test]
    fn test_iterator_when_requires_simultaneous_is_true() {
        let counter = MultiCounter::new(vec![3, 5], true);
        let output: Vec<_> = counter.into_iter().collect();

        assert_eq!(output.len(), 15);
        assert_eq!(output[0], vec![0, 0]);
        assert_eq!(output[1], vec![1, 1]);
        assert_eq!(output[2], vec![2, 2]);
        assert_eq!(output[3], vec![0, 3]);
        assert_eq!(output[4], vec![1, 4]);
        assert_eq!(output[5], vec![2, 0]);
        assert_eq!(output[6], vec![0, 1]);
        assert_eq!(output[7], vec![1, 2]);
        assert_eq!(output[8], vec![2, 3]);
        assert_eq!(output[9], vec![0, 4]);
        assert_eq!(output[10], vec![1, 0]);
        assert_eq!(output[11], vec![2, 1]);
        assert_eq!(output[12], vec![0, 2]);
        assert_eq!(output[13], vec![1, 3]);
        assert_eq!(output[14], vec![2, 4]);
    }
}
