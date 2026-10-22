//! The rail card networks (Dragon, Phoenix, Thunderbird) — check digits
//! ported byte-for-byte from the kernel's `cn_check.c`.

/// The three rail networks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RailNetwork {
    Dragon,
    Phoenix,
    Thunderbird,
}

impl RailNetwork {
    pub const fn prefix(self) -> &'static str {
        match self {
            Self::Dragon => "8809",
            Self::Phoenix => "8829",
            Self::Thunderbird => "8849",
        }
    }
}

fn luhn_sum(digits: &[u8], double_last: bool) -> u32 {
    let mut sum = 0u32;
    let mut dbl = double_last;
    for i in (0..digits.len()).rev() {
        let mut v = (digits[i] - b'0') as u32;
        if dbl {
            v *= 2;
            if v > 9 {
                v -= 9;
            }
        }
        sum += v;
        dbl = !dbl;
    }
    sum % 10
}

fn mod1110_run(digits: &[u8]) -> u32 {
    let mut p = 10u32;
    for &d in digits {
        let mut s = (p + (d - b'0') as u32) % 10;
        if s == 0 {
            s = 10;
        }
        p = (2 * s) % 11;
    }
    p
}

// the Damm table, exactly as in cn_check.c
const DAMM: [[u8; 10]; 10] = [
    [0, 3, 1, 7, 5, 9, 8, 6, 4, 2], [7, 0, 9, 2, 1, 5, 4, 8, 6, 3],
    [4, 2, 0, 6, 8, 7, 1, 3, 5, 9], [1, 7, 5, 0, 9, 8, 3, 4, 2, 6],
    [6, 1, 2, 3, 0, 4, 5, 9, 7, 8], [3, 6, 7, 4, 2, 0, 9, 5, 8, 1],
    [5, 8, 6, 9, 7, 2, 0, 1, 3, 4], [8, 9, 4, 5, 3, 6, 2, 0, 1, 7],
    [9, 4, 3, 8, 6, 1, 7, 2, 0, 5], [2, 5, 8, 1, 4, 3, 6, 7, 9, 0],
];

fn damm_run(digits: &[u8]) -> usize {
    let mut interim = 0usize;
    for &d in digits {
        interim = DAMM[interim][(d - b'0') as usize] as usize;
    }
    interim
}

// the Verhoeff tables
const V_D: [[u8; 10]; 10] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], [1, 2, 3, 4, 0, 6, 7, 8, 9, 5],
    [2, 3, 4, 0, 1, 7, 8, 9, 5, 6], [3, 4, 0, 1, 2, 8, 9, 5, 6, 7],
    [4, 0, 1, 2, 3, 9, 5, 6, 7, 8], [5, 9, 8, 7, 6, 0, 4, 3, 2, 1],
    [6, 5, 9, 8, 7, 1, 0, 4, 3, 2], [7, 6, 5, 9, 8, 2, 1, 0, 4, 3],
    [8, 7, 6, 5, 9, 3, 2, 1, 0, 4], [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
];
const V_P: [[u8; 10]; 8] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], [1, 5, 7, 6, 2, 8, 3, 0, 9, 4],
    [5, 8, 0, 3, 7, 9, 6, 1, 4, 2], [8, 9, 1, 6, 0, 4, 3, 5, 2, 7],
    [9, 4, 5, 3, 1, 2, 6, 8, 7, 0], [4, 2, 8, 6, 5, 7, 3, 9, 0, 1],
    [2, 7, 9, 3, 8, 0, 6, 4, 1, 5], [7, 0, 4, 6, 9, 1, 3, 2, 5, 8],
];
const V_INV: [u8; 10] = [0, 4, 3, 2, 1, 5, 6, 7, 8, 9];

fn verhoeff_run(digits: &[u8], mode: usize) -> usize {
    let mut c = mode;
    for (i, &d) in digits.iter().rev().enumerate() {
        c = V_D[c][V_P[i % 8][(d - b'0') as usize] as usize] as usize;
    }
    c
}

/// The rail network a 20-digit number belongs to, if any. Matches the
/// kernel's `cn_rail_valid`: Luhn over the full 20, inner over the 19.
pub fn rail_network(number: &str) -> Option<RailNetwork> {
    let b = number.as_bytes();
    if b.len() != 20 || !b.iter().all(|c| c.is_ascii_digit()) || b[0] == b'0' {
        return None;
    }
    if !luhn_valid(b) {
        return None;
    }
    let inner19 = &b[..19];
    if number.starts_with("8809") && mod1110_valid(inner19) {
        Some(RailNetwork::Dragon)
    } else if number.starts_with("8829") && damm_run(inner19) == 0 {
        Some(RailNetwork::Phoenix)
    } else if number.starts_with("8849") && V_INV[verhoeff_run(inner19, 0)] == 0 {
        Some(RailNetwork::Thunderbird)
    } else {
        None
    }
}

/// True iff the number is a valid rail number for the given network.
pub fn rail_number_valid(number: &str, network: RailNetwork) -> bool {
    rail_network(number) == Some(network)
}

fn luhn_valid(digits: &[u8]) -> bool {
    digits.len() >= 2 && luhn_sum(digits, false) == 0
}

fn mod1110_valid(digits: &[u8]) -> bool {
    if digits.len() < 2 {
        return false;
    }
    let p = mod1110_run(&digits[..digits.len() - 1]);
    (p + (digits[digits.len() - 1] - b'0') as u32) % 10 == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_testnet_cards() {
        // the cards minted and verified on testnet
        assert_eq!(rail_network("88090016741821469134"), Some(RailNetwork::Dragon));
        assert_eq!(rail_network("88290014920879549736"), Some(RailNetwork::Phoenix));
        assert_eq!(rail_network("88490012596815542981"), Some(RailNetwork::Thunderbird));
    }

    #[test]
    fn rejects_bad_numbers() {
        assert_eq!(rail_network("12345678901234567890"), None);
        assert_eq!(rail_network("88090000000000000000"), None); // bad luhn
    }
}