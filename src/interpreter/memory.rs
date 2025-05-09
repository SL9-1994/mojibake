#[derive(Debug)]
pub struct Memory {
    tape: Vec<u8>,
    pointer: usize,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            tape: vec![0; 30_000], // Brainfuck仕様に倣って30,000セル
            pointer: 0,
        }
    }

    pub fn increment(&mut self) {
        self.tape[self.pointer] = self.tape[self.pointer].wrapping_add(1);
    }

    pub fn decrement(&mut self) {
        self.tape[self.pointer] = self.tape[self.pointer].wrapping_sub(1);
    }

    pub fn move_right(&mut self) {
        self.pointer += 1;
        if self.pointer >= self.tape.len() {
            self.tape.push(0);
        }
    }

    pub fn move_left(&mut self) {
        if self.pointer == 0 {
            // 仕様に応じて panic かエラー返す
            panic!("Memory pointer moved left out of bounds");
        }
        self.pointer -= 1;
    }

    pub fn get(&self) -> u8 {
        self.tape[self.pointer]
    }

    pub fn set(&mut self, value: u8) {
        self.tape[self.pointer] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_operations() {
        let mut mem = Memory::new();
        assert_eq!(mem.get(), 0);

        mem.increment();
        assert_eq!(mem.get(), 1);

        mem.decrement();
        assert_eq!(mem.get(), 0);

        mem.set(42);
        assert_eq!(mem.get(), 42);

        mem.move_right();
        assert_eq!(mem.get(), 0);
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_move_left_panic() {
        let mut mem = Memory::new();
        mem.move_left(); // 左端超え → panic
    }
}
