//! The chance of at least one success must match the table on p. 59
//! (first column). The pushed column fits no simple model and is only
//! shown, not tested (docs/rules/02-dice-skills-panic.md §3.15).

use xenoforge_engine::dice::{Dice, throw};

/// A small, fixed pseudo-random source (SplitMix64), so the test is
/// reproducible.
struct SplitMix(u64);

impl Dice for SplitMix {
    fn d6(&mut self) -> u8 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^= z >> 31;
        u8::try_from(z % 6).unwrap() + 1
    }
}

#[test]
fn chance_of_success_matches_page_59() {
    // Dice 1–10: 17 %, 31 %, 42 %, 52 %, 60 %, 67 %, 72 %, 77 %, 81 %, 84 %.
    let table = [17.0, 31.0, 42.0, 52.0, 60.0, 67.0, 72.0, 77.0, 81.0, 84.0];
    let mut dice = SplitMix(2019);
    let rolls = 40_000;
    for (n, expected) in (1u8..=10).zip(table) {
        let exact = 100.0 * (1.0 - (5.0f64 / 6.0).powi(i32::from(n)));
        assert!(
            (exact - expected).abs() <= 0.5,
            "{n} dice: {exact:.1} vs {expected}"
        );
        let hits = (0..rolls)
            .filter(|_| throw(&mut dice, n, 0).iter().any(|d| d.success()))
            .count();
        let measured = 100.0 * hits as f64 / f64::from(rolls);
        assert!(
            (measured - exact).abs() < 1.0,
            "{n} dice: measured {measured:.1} %, exact {exact:.1} %"
        );
    }
}
