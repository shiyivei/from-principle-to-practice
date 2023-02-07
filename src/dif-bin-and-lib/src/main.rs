fn main() {
    use core::ops::Add;
    // 类型不同，行为相同，通过trait实现
    trait KnobControl<T: Add + Add<Output = T> + Copy> {
        fn set_position(&mut self, value: T);
        fn get_value(&self, p: T) -> T;
    }

    struct LinearKnob<T: Add + Add<Output = T> + Copy> {
        position: T,
    }

    struct LogarithmicKnob<T: Add + Add<Output = T> + Copy> {
        position: T,
    }

    impl<T: Add + Add<Output = T> + Copy> KnobControl<T> for LinearKnob<T> {
        fn set_position(&mut self, value: T) {
            self.position = value
        }
        fn get_value(&self, p: T) -> T {
            self.position
        }
    }

    impl<T: Add + Add<Output = T> + Copy> KnobControl<T> for LogarithmicKnob<T> {
        fn set_position(&mut self, value: T) {
            self.position = value
        }

        fn get_value(&self, p: T) -> T {
            self.position + p
        }
    }

    // 通过enum实现
    // 将类型抽象到枚举体中

    enum Knob<T: Add + Add<Output = T> + Copy> {
        Linear(LinearKnob<T>),
        Logarithmic(LogarithmicKnob<T>),
    }

    impl<T: Add + Add<Output = T> + Copy> KnobControl<T> for Knob<T> {
        fn set_position(&mut self, value: T) {
            match self {
                Knob::Linear(inner_knob) => inner_knob.set_position(value),
                Knob::Logarithmic(inner_knob) => inner_knob.set_position(value),
            }
        }

        fn get_value(&self, value: T) -> T {
            match self {
                Knob::Linear(inner_knob) => inner_knob.get_value(value),
                Knob::Logarithmic(inner_knob) => inner_knob.get_value(value),
            }
        }
    }
}
