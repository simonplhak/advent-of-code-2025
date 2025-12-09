use anyhow::Result;
use std::fmt::{self, Display};
use std::mem;
use std::{fs, path::Path};

pub fn read_lines(path: &Path) -> Result<Vec<String>> {
    let content = fs::read_to_string(path)?;
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    Ok(lines)
}

pub fn digit_count(n: usize) -> usize {
    if n == 0 {
        1
    } else {
        (n as f64).log10().floor() as usize + 1
    }
}

#[derive(Debug)]
pub struct Point2D {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct Point3D {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Point3D {
    pub fn l2(&self, b: &Point3D) -> f64 {
        (((self.x as isize - b.x as isize).pow(2)
            + (self.y as isize - b.y as isize).pow(2)
            + (self.z as isize - b.z as isize).pow(2)) as f64)
            .sqrt()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.start, self.end)
    }
}

impl Range {
    pub fn contains(&self, value: usize) -> bool {
        value >= self.start && value <= self.end
    }

    pub fn is_contained_in(&self, other: &Range) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end && self.end >= other.start
    }

    pub fn merge(&self, other: &Range) -> Range {
        Range {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

pub fn create_range(s: &str) -> Result<Range> {
    let split = s.split('-').collect::<Vec<&str>>();
    assert!(split.len() == 2);
    let start = split[0].parse::<usize>()?;
    let end = split[1].parse::<usize>()?;
    assert!(start <= end);
    Ok(Range { start, end })
}

pub fn make_ranges(lines: String, split_pattern: char) -> Result<Vec<Range>> {
    let ranges = lines
        .split(split_pattern)
        .map(create_range)
        .collect::<Result<Vec<Range>>>()?;
    Ok(ranges)
}
// Currently the implementation does not handle overlapping ranges correctly.
#[derive(Debug, PartialEq, Eq)]
pub struct RangedBinaryTree {
    pub value: Range,
    pub left: Option<Box<RangedBinaryTree>>,
    pub right: Option<Box<RangedBinaryTree>>,
}

impl Display for RangedBinaryTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_helper(f, "", true, "Root")
    }
}

impl RangedBinaryTree {
    pub fn new(value: Range) -> Self {
        RangedBinaryTree {
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, new_value: Range) {
        if new_value.is_contained_in(&self.value) {
            return;
        }
        if self.value.is_contained_in(&new_value) {
            self.value = new_value;
        } else if new_value.end <= self.value.start || new_value.start <= self.value.start {
            match &mut self.left {
                Some(node) => {
                    node.insert(new_value);
                }
                None => {
                    self.left = Some(Box::new(RangedBinaryTree::new(new_value)));
                }
            }
        } else if new_value.start >= self.value.end || new_value.start > self.value.start {
            match &mut self.right {
                Some(node) => {
                    node.insert(new_value);
                }
                None => {
                    self.right = Some(Box::new(RangedBinaryTree::new(new_value)));
                }
            }
        } else {
            panic!(
                "Overlapping ranges are not supported: {:?} vs {:?}",
                self.value, new_value
            );
        }
        self.normalize()
    }

    fn normalize(&mut self) {
        if let Some(left) = &self.left
            && left.value.end >= self.value.start
            && left.value.start <= self.value.end
        {
            let mut left = mem::take(&mut self.left).unwrap();
            if !left.right_is_none() {
                left.insert(self.value);
            }
            assert!(
                left.right_is_none(),
                "Left node expects to have no right children: {}",
                left
            );
            self.value.start = left.value.start;
            self.left = left.left;
        }
        if let Some(right) = &self.right
            && right.value.start <= self.value.end
            && right.value.end >= self.value.start
        {
            let mut right = mem::take(&mut self.right).unwrap();
            if !right.left_is_none() {
                right.insert(self.value);
            }
            assert!(
                right.left_is_none(),
                "Right node expects to have no left children: {}",
                right
            );
            self.value.end = right.value.end;
            self.right = right.right;
        }
    }

    pub fn left_is_none(&self) -> bool {
        self.left.is_none()
    }

    pub fn right_is_none(&self) -> bool {
        self.right.is_none()
    }

    pub fn total(&self) -> usize {
        assert!(self.value.end >= self.value.start);
        if let Some(left) = &self.left {
            assert!(left.value.end < self.value.start);
            assert!(left.value.start < self.value.start);
        }
        if let Some(right) = &self.right {
            assert!(right.value.start > self.value.end);
            assert!(right.value.start > self.value.start);
        }
        let children_total = self.left.as_ref().map_or(0, |left| left.total())
            + self.right.as_ref().map_or(0, |right| right.total());

        (self.value.end - self.value.start + 1) + children_total
    }

    pub fn search(&self, value: usize) -> bool {
        if self.value.contains(value) {
            return true;
        }

        if let Some(left) = &self.left
            && (value <= self.value.start || value <= self.value.end)
            && left.search(value)
        {
            return true;
        }

        if let Some(right) = &self.right
            && (value >= self.value.end || value >= self.value.start)
            && right.search(value)
        {
            return true;
        }

        false
    }

    pub fn from(new_values: &[Range]) -> Self {
        let (first, rest) = new_values.split_first().unwrap();
        let mut root = RangedBinaryTree::new(*first);

        for value in rest {
            root.insert(*value);
        }
        root
    }

    fn fmt_helper(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
        is_last: bool,
        node_name: &str,
    ) -> fmt::Result {
        let connector = if is_last { "└── " } else { "├── " };
        writeln!(f, "{}{}{}: {}", prefix, connector, node_name, self.value)?;

        // Prepare the prefix for the children
        // If this node is last, its children don't need a vertical bar from us.
        // If it isn't last, its children need a vertical bar to connect our siblings.
        let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

        // Collect children to determine who is strictly "last" for formatting purposes
        let has_right = self.right.is_some();

        if let Some(left) = &self.left {
            // Left is only "last" if there is no Right node
            left.fmt_helper(f, &child_prefix, !has_right, "L")?;
        }

        if let Some(right) = &self.right {
            // Right is always "last" in the order of printing
            right.fmt_helper(f, &child_prefix, true, "R")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_insert_and_search() {
        let mut tree = RangedBinaryTree::new(Range { start: 10, end: 20 });
        tree.insert(Range { start: 0, end: 5 }); // Left
        tree.insert(Range { start: 30, end: 40 }); // Right

        assert!(tree.search(15)); // In Root
        assert!(tree.search(2)); // In Left
        assert!(tree.search(35)); // In Right
        assert!(!tree.search(100)); // Not in tree
        assert!(!tree.search(8)); // Gap between Left and Root
        assert_eq!(tree.total(), 28); // 11 (10-20) + 6 (0-5) + 11 (30-40)
    }

    #[test]
    fn test_overlapping_search() {
        // Root: [10, 20]
        // Left: [0, 50] (Starts before 10, so goes Left, but extends way past Root)
        let mut tree = RangedBinaryTree::new(Range { start: 10, end: 20 });
        tree.insert(Range { start: 0, end: 50 });

        // Search for 45.
        // 45 > 10. Root doesn't have it.
        // Algorithm shouldn't just look Right (which is empty).
        // It must verify Left because Left could overlap.
        assert!(tree.search(45));
        assert_eq!(tree.total(), 51); // 51 (0-50)
    }

    #[test]
    fn test_boundary_conditions() {
        let tree = RangedBinaryTree::new(Range { start: 10, end: 20 });
        assert!(tree.search(10)); // Start inclusive
        assert!(tree.search(20)); // End inclusive
        assert!(!tree.search(9));
        assert!(!tree.search(21));
        assert_eq!(tree.total(), 11); // 10 through 20 inclusive
    }

    #[test]
    fn test_deep_tree() {
        let mut tree = RangedBinaryTree::new(Range { start: 50, end: 60 });
        // Zig Zag
        tree.insert(Range { start: 25, end: 30 });
        tree.insert(Range { start: 10, end: 15 });
        tree.insert(Range { start: 75, end: 80 });
        tree.insert(Range {
            start: 100,
            end: 110,
        });

        assert!(tree.search(12));
        assert!(tree.search(105));
        assert!(!tree.search(40));
        let expected = RangedBinaryTree {
            value: Range { start: 50, end: 60 },
            left: Some(Box::new(RangedBinaryTree {
                value: Range { start: 25, end: 30 },
                left: Some(Box::new(RangedBinaryTree {
                    value: Range { start: 10, end: 15 },
                    left: None,
                    right: None,
                })),
                right: None,
            })),
            right: Some(Box::new(RangedBinaryTree {
                value: Range { start: 75, end: 80 },
                left: None,
                right: Some(Box::new(RangedBinaryTree {
                    value: Range {
                        start: 100,
                        end: 110,
                    },
                    left: None,
                    right: None,
                })),
            })),
        };
        assert_eq!(tree, expected);
        assert_eq!(tree.total(), 40); // 11 + 6 + 6 + 6 + 11
    }

    #[test]
    fn test_merge_overlap_right() {
        // [10, 20] + [15, 25] -> [10, 25]
        let mut tree = RangedBinaryTree::new(Range { start: 10, end: 20 });
        tree.insert(Range { start: 15, end: 25 });

        let expected = RangedBinaryTree {
            value: Range { start: 10, end: 25 },
            left: None,
            right: None,
        };
        assert_eq!(
            tree, expected,
            "Tree failed to merge overlapping range on the right"
        );
        assert_eq!(tree.total(), 16);
    }

    #[test]
    fn test_merge_overlap_left() {
        // [10, 20] + [5, 15] -> [5, 20]
        // Note: Depending on implementation details, this might shift the root's value
        // or rebalance, but the resulting single node structure is what we care about here.
        let mut tree = RangedBinaryTree::new(Range { start: 10, end: 20 });
        tree.insert(Range { start: 5, end: 15 });

        let expected = RangedBinaryTree {
            value: Range { start: 5, end: 20 },
            left: None,
            right: None,
        };
        assert_eq!(
            tree, expected,
            "Tree failed to merge overlapping range on the left"
        );
        assert_eq!(tree.total(), 16);
    }

    #[test]
    fn test_merge_subset() {
        // [10, 30] + [15, 25] -> [10, 30]
        let mut tree = RangedBinaryTree::new(Range { start: 10, end: 30 });
        tree.insert(Range { start: 15, end: 25 });

        let expected = RangedBinaryTree {
            value: Range { start: 10, end: 30 },
            left: None,
            right: None,
        };
        assert_eq!(
            tree, expected,
            "Tree shouldn't change when inserting a subset range"
        );
        assert_eq!(tree.total(), 21);
    }

    #[test]
    fn test_merge_superset() {
        // [15, 25] + [10, 30] -> [10, 30]
        let mut tree = RangedBinaryTree::new(Range { start: 15, end: 25 });
        tree.insert(Range { start: 10, end: 30 });

        let expected = RangedBinaryTree {
            value: Range { start: 10, end: 30 },
            left: None,
            right: None,
        };
        assert_eq!(
            tree, expected,
            "Tree should update to the larger range when inserting a superset"
        );
        assert_eq!(tree.total(), 21);
    }

    #[test]
    fn test_merge_touching_right() {
        // [10, 20] + [20, 30] -> [10, 30]
        // Assuming inclusive ranges touch at 20 and should merge.
        let mut tree = RangedBinaryTree::new(Range { start: 10, end: 20 });
        tree.insert(Range { start: 20, end: 30 });

        let expected = RangedBinaryTree {
            value: Range { start: 10, end: 30 },
            left: None,
            right: None,
        };
        assert_eq!(tree, expected, "Tree should merge touching ranges");
        assert_eq!(tree.total(), 21);
    }

    #[test]
    fn test_merge_touching_left() {
        let mut tree = RangedBinaryTree::new(Range { start: 10, end: 20 });
        tree.insert(Range { start: 5, end: 10 });

        let expected = RangedBinaryTree {
            value: Range { start: 5, end: 20 },
            left: None,
            right: None,
        };
        assert_eq!(tree, expected, "Tree should merge touching ranges");
        assert_eq!(tree.total(), 16);
    }

    #[test]
    fn test_disjoint_ranges() {
        // [10, 20] and [30, 40] are distinct.
        let mut tree = RangedBinaryTree::new(Range { start: 10, end: 20 });
        tree.insert(Range { start: 30, end: 40 });

        let expected = RangedBinaryTree {
            value: Range { start: 10, end: 20 },
            left: None,
            right: Some(Box::new(RangedBinaryTree {
                value: Range { start: 30, end: 40 },
                left: None,
                right: None,
            })),
        };
        assert_eq!(
            tree, expected,
            "Disjoint ranges should remain separate nodes"
        );
        assert_eq!(tree.total(), 22);
    }

    #[test]
    fn test_bridging_gap() {
        // Setup: [0, 10] and [20, 30]
        // Insert: [5, 25] -> Bridges both into [0, 30]
        let mut tree = RangedBinaryTree::new(Range { start: 0, end: 10 });
        tree.insert(Range { start: 20, end: 30 });

        // This insert should merge the root and the right child
        tree.insert(Range { start: 5, end: 25 });

        let expected = RangedBinaryTree {
            value: Range { start: 0, end: 30 },
            left: None,
            right: None,
        };
        assert_eq!(
            tree, expected,
            "Inserting a bridge range should collapse multiple nodes"
        );
    }

    #[test]
    fn test_total_simple() {
        // [10, 20] contains 10, 11, ..., 20. Total = 11 numbers.
        let tree = RangedBinaryTree::new(Range { start: 10, end: 20 });
        assert_eq!(tree.total(), 11);
    }

    #[test]
    fn test_total_disjoint() {
        // [1, 5] (5 numbers) + [10, 12] (3 numbers) = 8
        let mut tree = RangedBinaryTree::new(Range { start: 1, end: 5 });
        tree.insert(Range { start: 10, end: 12 });
        assert_eq!(tree.total(), 8);
    }

    #[test]
    fn test_total_merged() {
        // [10, 20] (11) + [15, 25]
        // Overlap merge -> [10, 25] (16 numbers)
        // If they weren't merged, naive sum might be 11 + 11 = 22 (double counting 15..20).
        // Since implementation merges, total should be 16.
        let mut tree = RangedBinaryTree::new(Range { start: 10, end: 20 });
        tree.insert(Range { start: 15, end: 25 });
        assert_eq!(tree.total(), 16);
    }

    #[test]
    fn test_total_bridging() {
        // [0, 10] (11) + [20, 30] (11) = 22 disjoint
        let mut tree = RangedBinaryTree::new(Range { start: 0, end: 10 });
        tree.insert(Range { start: 20, end: 30 });

        // Bridge with [5, 25]. Result should be [0, 30] (31 numbers).
        tree.insert(Range { start: 5, end: 25 });
        assert_eq!(tree.total(), 31);
    }
    #[test]
    fn test_total_complex_structure() {
        // Create a tree with 15 disjoint ranges.
        // Each range has a length of 10 (e.g., [0, 9], [20, 29]...).
        // Since they are disjoint, the tree should maintain separate nodes for them.
        // Expected total unique numbers = 15 * 10 = 150.

        // Start with a middle value to act as root
        let mut tree = RangedBinaryTree::new(Range {
            start: 100,
            end: 109,
        });

        // Insert ranges to the left and right
        let ranges = vec![
            Range { start: 0, end: 9 },
            Range { start: 20, end: 29 },
            Range { start: 40, end: 49 },
            Range { start: 60, end: 69 },
            Range { start: 80, end: 89 },
            // 100-109 is root
            Range {
                start: 120,
                end: 129,
            },
            Range {
                start: 140,
                end: 149,
            },
            Range {
                start: 160,
                end: 169,
            },
            Range {
                start: 180,
                end: 189,
            },
            Range {
                start: 200,
                end: 209,
            },
            Range {
                start: 220,
                end: 229,
            },
            Range {
                start: 240,
                end: 249,
            },
            Range {
                start: 260,
                end: 269,
            },
            Range {
                start: 280,
                end: 289,
            },
            Range {
                start: 300,
                end: 309,
            },
        ];

        for r in ranges {
            tree.insert(r);
        }

        // 1 (root) + 14 (inserted) = 15 ranges.
        // Each range is inclusive with diff 9 (e.g., 0 to 9 is 10 numbers).
        // Total = 15 * 10 = 150.
        assert_eq!(tree.total(), 160);
    }

    #[test]
    fn test_total_overlap_merging_complex() {
        // [10, 20] (11 items)
        let mut tree = RangedBinaryTree::new(Range { start: 10, end: 20 });

        // Add ranges that overlap significantly

        // [15, 25] overlaps end -> Union should be [10, 25] (16 items)
        // If implementation is correct, tree contains one node [10, 25]
        tree.insert(Range { start: 15, end: 25 });
        assert_eq!(tree.total(), 16);

        // [5, 15] overlaps start -> Union should be [5, 25] (21 items)
        tree.insert(Range { start: 5, end: 15 });
        assert_eq!(tree.total(), 21);

        // Add a range that bridges to a new area but overlaps existing
        // [20, 40] overlaps [5, 25] -> Union should be [5, 40] (36 items)
        tree.insert(Range { start: 20, end: 40 });
        assert_eq!(tree.total(), 36);

        // Add a completely contained range (shouldn't change total)
        // [10, 30] is inside [5, 40]
        tree.insert(Range { start: 10, end: 30 });
        assert_eq!(tree.total(), 36);

        // Add a range that engulfs everything
        // [0, 50] covers [5, 40] -> Union should be [0, 50] (51 items)
        tree.insert(Range { start: 0, end: 50 });
        assert_eq!(tree.total(), 51);
    }

    #[test]
    fn test_complex_merge_bridging_gap() {
        // Construct a tree with multiple gaps and levels
        // Initial Structure:
        //       [50, 60]
        //      /        \
        //  [10, 20]    [90, 100]
        //     \          /
        //   [30, 40]  [70, 80]

        let mut tree = RangedBinaryTree::new(Range { start: 50, end: 60 });
        tree.insert(Range { start: 10, end: 20 });
        tree.insert(Range {
            start: 90,
            end: 100,
        });
        tree.insert(Range { start: 30, end: 40 });
        tree.insert(Range { start: 70, end: 80 });

        // Total currently: 5 disjoint ranges of 11 items each = 55
        assert_eq!(tree.total(), 55, "Initial disjoint tree total incorrect");

        // Insert a range that bridges ALL of them: [15, 95]
        // This overlaps with [10, 20] (extending it to 95)
        // It fully engulfs [30, 40], [50, 60], [70, 80]
        // It overlaps with [90, 100] (extending start to 15)
        // Ideally, this should collapse everything into one single range [10, 100]
        tree.insert(Range { start: 15, end: 95 });

        // [10, 100] contains 91 items (100 - 10 + 1)
        assert_eq!(
            tree.total(),
            91,
            "Tree failed to collapse multiple levels into single range: {tree}"
        );
    }

    #[test]
    fn test_complex_merge_bridging_gap_2() {
        // Construct a tree with multiple gaps and levels
        // Initial Structure:
        //       [50, 60]
        //      /        \
        //  [10, 20]    [90, 100]
        //     \          /
        //   [30, 40]  [70, 80]

        let mut tree = RangedBinaryTree::new(Range { start: 50, end: 60 });
        tree.insert(Range { start: 10, end: 20 });
        tree.insert(Range {
            start: 90,
            end: 100,
        });
        tree.insert(Range { start: 30, end: 40 });
        tree.insert(Range { start: 70, end: 80 });

        // Total currently: 5 disjoint ranges of 11 items each = 55
        assert_eq!(tree.total(), 55, "Initial disjoint tree total incorrect");

        tree.insert(Range { start: 15, end: 35 });
        // Structure:
        //       [50, 60]
        //      /        \
        //  [10, 40]    [90, 100]
        //                /
        //            [70, 80]
        assert_eq!(
            tree.total(),
            64,
            "Tree failed to collapse multiple levels into single range: {tree}"
        );

        // tree.insert(Range { start: 40, end: 70 });
        // // Structure:
        // //       [10, 70]
        // //               \
        // //              [90, 100]
        // //                /
        // //            [40, 80]
        // assert_eq!(
        //     tree.total(),
        //     64,
        //     "Tree failed to collapse multiple levels into single range: {tree}"
        // );
    }
}
