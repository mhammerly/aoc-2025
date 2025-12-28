use std::collections::BTreeMap;
use std::iter::FromIterator;

use util::range::{ParseRange, RangeError};

pub struct Kitchen {
    fresh_ranges: MultiRange,
}

impl Kitchen {
    pub fn import_fresh_ranges(lines: impl Iterator<Item = String>) -> anyhow::Result<Kitchen> {
        let fresh_ranges = lines
            .map(|s| <(u64, u64) as ParseRange>::parse_range(s.as_str()))
            .collect::<Result<MultiRange, RangeError>>()?;
        Ok(Kitchen { fresh_ranges })
    }

    pub fn is_fresh(&self, ingredient: u64) -> bool {
        self.fresh_ranges.check_point(ingredient)
    }

    pub fn fresh_ingredients(&self) -> impl Iterator<Item = u64> {
        self.fresh_ranges.iter()
    }
}

struct MultiRange {
    ranges: BTreeMap<u64, u64>,
    reverse_ranges: BTreeMap<u64, u64>,
}

impl MultiRange {
    pub fn new() -> MultiRange {
        MultiRange {
            ranges: BTreeMap::new(),
            reverse_ranges: BTreeMap::new(),
        }
    }

    fn overlapping(&self, (start, end): (u64, u64)) -> impl Iterator<Item = (u64, u64)> {
        let ends_after_start = self
            .reverse_ranges
            .range(start..=end)
            .map(|(end, start)| (*start, *end));
        let starts_before_end = self
            .ranges
            .range(start..=end)
            .map(|(start, end)| (*start, *end));

        // TODO: Filter out duplicates
        ends_after_start.chain(starts_before_end)
    }

    pub fn insert(&mut self, (start, end): (u64, u64)) {
        // All ranges that overlap with the current one can be flattened into a single range which
        // begins where the earliest overlapping range begins and ends where the latest overlapping
        // range ends. Find `min_start` and `max_end` accordingly.
        let ranges_to_merge: Vec<(u64, u64)> = self.overlapping((start, end)).collect();
        tracing::trace!(
            "({start}, {end}) has {} overlapping ranges: {ranges_to_merge:?}",
            ranges_to_merge.len()
        );
        let mut min_start = start;
        let mut max_end = end;
        for (start, end) in ranges_to_merge.iter() {
            min_start = std::cmp::min(*start, min_start);
            max_end = std::cmp::max(*end, max_end);
        }

        tracing::trace!(
            "Merging {} ranges into ({min_start}, {max_end})",
            ranges_to_merge.len()
        );

        // Insert our flattened range into our maps.
        self.ranges.insert(min_start, max_end);
        self.reverse_ranges.insert(max_end, min_start);

        // Remove the pre-flattened ranges.
        for (start, end) in ranges_to_merge.iter() {
            if *start != min_start {
                tracing::trace!("Removing ({start}, {end}) from forward range list");
                self.ranges.remove(start);
            }
            if *end != max_end {
                tracing::trace!("Removing ({start}, {end}) from reverse range list");
                self.reverse_ranges.remove(end);
            }
        }
    }

    pub fn check_point(&self, point: u64) -> bool {
        self.ranges
            .iter()
            .any(|(start, end)| point >= *start && point <= *end)
    }

    pub fn iter(&self) -> impl Iterator<Item = u64> {
        self.ranges.iter().flat_map(|(start, end)| *start..=*end)
    }
}

impl FromIterator<(u64, u64)> for MultiRange {
    fn from_iter<I: IntoIterator<Item = (u64, u64)>>(iter: I) -> Self {
        let mut multirange = MultiRange::new();

        for i in iter {
            multirange.insert(i)
        }

        multirange
    }
}
