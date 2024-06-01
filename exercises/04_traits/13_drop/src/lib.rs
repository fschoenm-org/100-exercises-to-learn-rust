pub struct DropBomb {
    is_defused: bool,
}

impl DropBomb {
    pub fn new() -> Self {
        DropBomb { is_defused: false }
    }

    pub fn defuse(&mut self) {
        self.is_defused = true;
    }
}

impl Default for DropBomb {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if !self.is_defused {
            panic!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let _bomb = DropBomb::new();
        // The bomb should panic when dropped
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused
    }
}
