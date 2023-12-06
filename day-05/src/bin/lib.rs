pub const MAP_NAMES: [&str; 7] = [
    "seed-to-soil map",
    "soil-to-fertilizer map",
    "fertilizer-to-water map",
    "water-to-light map",
    "light-to-temperature map",
    "temperature-to-humidity map",
    "humidity-to-location map",
];

pub fn get_map(key: &str, input_str: &str) -> Vec<RangeMap> {
    let mut destination_to_source: Vec<RangeMap> = Vec::new();
    let mut in_destination_to_source_map = false;
    for line in input_str.lines().skip(1) {
        if line.contains(key) {
            in_destination_to_source_map = true;
            continue;
        }
        if line == "" {
            in_destination_to_source_map = false;
            continue;
        }
        if in_destination_to_source_map {
            let (destination, source, range) = {
                let mut iter = line.split_whitespace().map(|x| x.parse::<i128>().unwrap());
                (
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                )
            };
            destination_to_source.push(RangeMap::new(source, destination, range));
        }
    }

    destination_to_source.sort_by_key(|k| k.source.start);
    destination_to_source
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub start: i128,
    pub end: i128,
}

impl Line {
    pub fn intersects(&self, other: &Line) -> bool {
        !((self.end < other.start) || (other.end < self.start))
    }

    pub fn overlap(&self, other: &Line) -> Line {
        Line {
            start: self.start.max(other.start),
            end: self.end.min(other.end),
        }
    }

    pub fn non_overlapping_sections(&self, line2: Line) -> Vec<Line> {
        let mut sections = Vec::new();

        let (first_line, second_line) = if self.start < line2.start {
            (*self, line2)
        } else {
            (line2, *self)
        };

        if first_line.end <= second_line.start {
            sections.push(first_line);
            sections.push(second_line);
        } else {
            if first_line.start < second_line.start {
                sections.push(Line {
                    start: first_line.start,
                    end: second_line.start,
                });
            }
            if first_line.end < second_line.end {
                sections.push(Line {
                    start: first_line.end,
                    end: second_line.end,
                });
            }
        }

        sections
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RangeMap {
    pub source: Line,
    pub destination: Line,
    pub diff: i128,
}

impl RangeMap {
    pub fn new(source_start: i128, destination_start: i128, range: i128) -> RangeMap {
        RangeMap {
            source: Line {
                start: source_start,
                end: source_start + range,
            },
            destination: Line {
                start: destination_start,
                end: destination_start + range,
            },
            diff: destination_start - source_start,
        }
    }
}
