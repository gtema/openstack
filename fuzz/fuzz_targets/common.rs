//! Helpers shared between fuzz targets (not a target itself).

/// Largest array index we let through to `config-rs` path parsing.
const MAX_INDEX: usize = 10_000;

/// Return `true` when the input contains a `[<digits>]` subscript with an
/// index so large that `config-rs` tries to allocate a vector of that size
/// and runs out of memory (e.g. `key[999999999]:`).
///
/// This is a known upstream issue, see
/// <https://github.com/rust-cli/config-rs/issues/232>. Such inputs are
/// skipped so the fuzzer can keep looking for other problems instead of
/// reporting the same OOM over and over. Remove once fixed upstream.
#[allow(dead_code)]
pub fn has_huge_index(data: &str) -> bool {
    let bytes = data.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'[' {
            let start = i + 1;
            let mut end = start;
            while end < bytes.len() && bytes[end].is_ascii_digit() {
                end += 1;
            }
            if end > start
                && (end - start > 9
                    || data[start..end]
                        .parse::<usize>()
                        .is_ok_and(|n| n > MAX_INDEX))
            {
                return true;
            }
            i = end.max(i + 1);
        } else {
            i += 1;
        }
    }
    false
}
