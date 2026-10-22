//! The four-valued paraconsistent gate (the kernel's semantics).

/// The four foundation states.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LogicState {
    Neither = 0x00,
    True = 0x01,
    False = 0x02,
    Both = 0x03,
}

impl LogicState {
    /// The raw byte value (the kernel's wire format).
    #[inline]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// Decode from a raw byte (masked to 0..3).
    #[inline]
    pub const fn from_u8(v: u8) -> Self {
        match v & 0x03 {
            0x00 => Self::Neither,
            0x01 => Self::True,
            0x02 => Self::False,
            _ => Self::Both,
        }
    }

    /// The settlement invariant: **only TRUE is settleable.**
    #[inline]
    pub const fn is_settleable(self) -> bool {
        matches!(self, Self::True)
    }
}

/// The gate: the kernel's four-valued operations, matching `lpres.c`.
pub struct Gate;

impl Gate {
    /// Knowledge join: `kjoin(a, b) = (a | b) & 3`. Note: the DEPLOYED
    /// gate's op-2 exposes this and does NOT range-check — use
    /// `conjoin` for settlement evidence (see the docs).
    #[inline]
    pub const fn kjoin(a: LogicState, b: LogicState) -> LogicState {
        LogicState::from_u8((a.as_u8() | b.as_u8()) & 0x03)
    }

    /// The kernel's `lpres_conjoin` table (from `lpres.c`), copied
    /// rule-for-rule: FALSE dominates; TRUE is identity; NEITHER
    /// dominates BOTH (a gap + a contradiction never grants authority).
    #[inline]
    pub const fn conjoin(a: LogicState, b: LogicState) -> LogicState {
        let (x, y) = (a.as_u8(), b.as_u8());
        if x == 0x02 || y == 0x02 {
            return LogicState::False;
        }
        if x == 0x01 {
            return b;
        }
        if y == 0x01 {
            return a;
        }
        if x == 0x00 || y == 0x00 {
            return LogicState::Neither;
        }
        LogicState::Both
    }

    /// The kernel's `lpres_disjoin` table (existential; never settlement).
    #[inline]
    pub const fn disjoin(a: LogicState, b: LogicState) -> LogicState {
        let (x, y) = (a.as_u8(), b.as_u8());
        if x == 0x01 || y == 0x01 {
            return LogicState::True;
        }
        if x == 0x02 {
            return b;
        }
        if y == 0x02 {
            return a;
        }
        if x == 0x03 || y == 0x03 {
            return LogicState::Both;
        }
        LogicState::Neither
    }
}

/// Fold evidence with the kernel's conjoin. The fold starts from the
/// FIRST evidence (a NEITHER accumulator would poison BOTH). Returns
/// `Err` when the verdict is not settleable (the guard pattern).
pub fn settle_or_hold(states: &[LogicState]) -> Result<LogicState, LogicState> {
    if states.is_empty() {
        return Err(LogicState::Neither);
    }
    let mut acc = states[0];
    for &s in &states[1..] {
        acc = Gate::conjoin(acc, s);
    }
    if acc.is_settleable() {
        Ok(acc)
    } else {
        Err(acc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conjoin_matches_kernel() {
        assert_eq!(Gate::conjoin(LogicState::True, LogicState::True), LogicState::True);
        assert_eq!(Gate::conjoin(LogicState::True, LogicState::False), LogicState::False);
        assert_eq!(Gate::conjoin(LogicState::Neither, LogicState::True), LogicState::Neither);
        assert_eq!(Gate::conjoin(LogicState::Neither, LogicState::Both), LogicState::Neither);
        assert_eq!(Gate::conjoin(LogicState::Both, LogicState::Both), LogicState::Both);
    }

    #[test]
    fn only_true_settles() {
        assert!(LogicState::True.is_settleable());
        assert!(!LogicState::Neither.is_settleable());
        assert!(!LogicState::False.is_settleable());
        assert!(!LogicState::Both.is_settleable());
    }

    #[test]
    fn settle_or_hold_holds_contradiction() {
        assert_eq!(settle_or_hold(&[LogicState::True, LogicState::False]), Err(LogicState::False));
        assert_eq!(settle_or_hold(&[LogicState::True, LogicState::True]), Ok(LogicState::True));
    }
}