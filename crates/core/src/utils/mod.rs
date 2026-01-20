pub mod orthonormal_basis;
pub mod perlin;

pub use orthonormal_basis::OrthonormalBasis;
pub use perlin::Perlin;

#[cfg(not(target_arch = "wasm32"))]
pub fn to_absolute(path: &str) -> std::io::Result<std::path::PathBuf> {
    use std::env;
    use std::path::Path;

    let path = Path::new(path);

    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(env::current_dir()?.join(path))
    }
}

/// Returns the line number (0-indexed) and column position (0-indexed)
/// at the given byte offset in the string.
///
/// # Arguments
///
/// * `s` - The string to search in
/// * `offset` - The byte offset into the string
///
/// # Returns
///
/// Returns `Some((line, column))` where:
/// - `line` is 0-indexed (first line is 0)
/// - `column` is 0-indexed (first character in a line is 0)
///
/// Returns `None` if the offset is out of bounds.
///
/// # Examples
///
/// ```
/// use caustic_core::utils::line_and_column_at_offset;
/// let text = "Hello\nWorld";
/// assert_eq!(line_and_column_at_offset(text, 0), Some((0, 0)));  // 'H'
/// assert_eq!(line_and_column_at_offset(text, 5), Some((0, 5)));  // '\n'
/// assert_eq!(line_and_column_at_offset(text, 6), Some((1, 0)));  // 'W'
/// assert_eq!(line_and_column_at_offset(text, 100), None);        // out of bounds
/// ```
pub fn line_and_column_at_offset(s: &str, offset: usize) -> Option<(usize, usize)> {
    if offset > s.len() {
        return None;
    }

    let mut line_num = 0;
    let mut col = 0;

    for (i, c) in s.char_indices() {
        if i >= offset {
            break;
        }

        if c == '\n' {
            line_num += 1;
            col = 0;
        } else {
            col += 1;
        }
    }

    Some((line_num, col))
}

pub fn line_offset_to_position(s: &str, line: usize, offset: usize) -> Option<usize> {
    let mut current_line = 0;
    let mut line_start = 0;

    for (i, ch) in s.char_indices() {
        if current_line == line {
            // We're on the target line, now find the offset
            let mut current_offset = 0;
            for (j, _) in s[line_start..].char_indices() {
                if current_offset == offset {
                    return Some(line_start + j);
                }
                current_offset += 1;
            }
            // If offset is at the end of the line
            if current_offset == offset {
                return Some(s.len());
            }
            return None; // offset out of bounds
        }

        if ch == '\n' {
            current_line += 1;
            line_start = i + 1;
        }
    }

    // Check if we're on the last line
    if current_line == line {
        let mut current_offset = 0;
        for (j, _) in s[line_start..].char_indices() {
            if current_offset == offset {
                return Some(line_start + j);
            }
            current_offset += 1;
        }
        if current_offset == offset {
            return Some(s.len());
        }
    }

    None // line out of bounds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_line() {
        let text = "Hello, world!";
        assert_eq!(line_and_column_at_offset(text, 0), Some((0, 0)));
        assert_eq!(line_and_column_at_offset(text, 5), Some((0, 5)));
        assert_eq!(line_and_column_at_offset(text, 7), Some((0, 7)));
    }

    #[test]
    fn test_multiple_lines() {
        let text = "Line 1\nLine 2\nLine 3";
        assert_eq!(line_and_column_at_offset(text, 0), Some((0, 0))); // 'L' in Line 1
        assert_eq!(line_and_column_at_offset(text, 6), Some((0, 6))); // '\n' after Line 1
        assert_eq!(line_and_column_at_offset(text, 7), Some((1, 0))); // 'L' in Line 2
        assert_eq!(line_and_column_at_offset(text, 10), Some((1, 3))); // 'e' in Line 2
        assert_eq!(line_and_column_at_offset(text, 13), Some((1, 6))); // '\n' after Line 2
        assert_eq!(line_and_column_at_offset(text, 14), Some((2, 0))); // 'L' in Line 3
        assert_eq!(line_and_column_at_offset(text, 18), Some((2, 4))); // '3' in Line 3
    }

    #[test]
    fn test_out_of_bounds() {
        let text = "Hello";
        assert_eq!(line_and_column_at_offset(text, 100), None);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(line_and_column_at_offset("", 0), Some((0, 0)));
        assert_eq!(line_and_column_at_offset("", 1), None);
    }

    #[test]
    fn test_unicode() {
        let text = "Hello 🦀\nRust 🚀";
        assert_eq!(line_and_column_at_offset(text, 0), Some((0, 0))); // 'H'
        assert_eq!(line_and_column_at_offset(text, 6), Some((0, 6))); // '🦀'
        assert_eq!(line_and_column_at_offset(text, 11), Some((1, 0))); // 'R' on line 2
        assert_eq!(line_and_column_at_offset(text, 16), Some((1, 5))); // '🚀'
    }

    #[test]
    fn test_at_end_of_string() {
        let text = "Hello\nWorld";
        assert_eq!(line_and_column_at_offset(text, text.len()), Some((1, 5)));
    }
}
