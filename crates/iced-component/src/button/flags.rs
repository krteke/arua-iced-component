#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct ButtonFlags(u8);

impl ButtonFlags {
    pub(super) const HOVERED: Self = Self(1 << 0);
    pub(super) const PRESSED: Self = Self(1 << 1);
    pub(super) const FOCUSED: Self = Self(1 << 2);
    pub(super) const DISABLED: Self = Self(1 << 3);

    pub(super) const fn empty() -> Self {
        Self(0)
    }

    pub(super) const fn contains(self, flags: Self) -> bool {
        self.0 & flags.0 == flags.0
    }

    pub(super) fn insert(&mut self, flags: Self) {
        self.0 |= flags.0;
    }

    pub(super) fn remove(&mut self, flags: Self) {
        self.0 &= !flags.0;
    }
}

impl core::ops::BitOr for ButtonFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
