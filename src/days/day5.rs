use std::str::FromStr;

struct Mapping {
    in_start: i64,
    out_start: i64,
    len: i64,
}

impl Mapping {
    fn map(&self, input: i64) -> (i64, bool) {
        if input >= self.in_start && input < self.in_start + self.len {
            (input + (self.out_start - self.in_start), true)
        } else {
            (input, false)
        }
    }

    fn map_range(&self, start: i64, len: i64) -> Vec<(i64, i64, bool)> {
        let max = start + len;

        let overlap_start = i64::max(self.in_start, start);
        let overlap_end = i64::min(self.in_start + self.len, max);

        if overlap_end < overlap_start {
            vec![(start, len, false)]
        } else {
            let mut v = vec![];
            if start < overlap_start {
                v.push((start, overlap_start - start, false));
            }
            v.push((self.map(overlap_start).0, overlap_end - overlap_start, true));
            if max > overlap_end {
                v.push((overlap_end, max - overlap_end, false));
            }

            v
        }
    }

    fn map_all_range(mappings: &[Self], start: i64, len: i64) -> Vec<(i64, i64)> {
        if mappings.len() == 0 {
            return if len > 0 {
                vec![(start, len)]
            } else {
                vec![]
            };
        }

        let mut out = vec![];

        let mapped = mappings[0].map_range(start, len);
        for (s, l, m) in mapped {
            if m && l > 0 {
                out.push((s, l));
            } else {
                out.append(&mut Self::map_all_range(&mappings[1..], s, l));
            }
        }

        out
    }
}

fn get_seeds_maps(input: String) -> (Vec<i64>, Vec<Vec<Mapping>>) {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seeds: Vec<i64> = seeds
        .replace("seeds: ", "")
        .trim()
        .split(" ")
        .map(|s| i64::from_str(s.trim()).unwrap())
        .collect();

    let maps: Vec<Vec<Mapping>> = maps.split("\n\n").map(|map| {
        let mut lines: Vec<&str> = map.lines().collect();
        lines.remove(0);
        let mut mappings = vec![];
        for l in lines {
            let nums: Vec<i64> = l.split(" ").map(|s| i64::from_str(s.trim()).unwrap()).collect();
            mappings.push(Mapping {
                in_start: nums[1],
                out_start: nums[0],
                len: nums[2],
            });
        }
        mappings
    }).collect();

    (seeds, maps)
}

pub(crate) fn part1(input: String) {
    let (seeds, maps) = get_seeds_maps(input);

    let min = seeds.into_iter().map(|mut seed| {
        for map in &maps {
            for mapping in map {
                let (mapped, changed) = mapping.map(seed);
                seed = mapped;
                if changed {
                    break;
                }
            }
        }
        seed
    }).min().unwrap();

    println!("{min}");
}

pub(crate) fn part2(input: String) {
    let (seeds, maps) = get_seeds_maps(input);
    let seed_ranges: Vec<(i64, i64)> = seeds.chunks_exact(2).map(|chunk| (chunk[0], chunk[1])).collect();
    let mut ranges: Vec<Vec<(i64, i64)>> = vec![seed_ranges];

    for map in maps {
        let unmapped = ranges.last().unwrap();
        let mapped = unmapped.iter().map(|(s, l)| {
            Mapping::map_all_range(&map, *s, *l)
        }).flatten().collect();
        ranges.push(mapped);
    }

    let min = ranges.last().unwrap().iter().map(|(s, _)| *s).min().unwrap();

    println!("{min}");
}
