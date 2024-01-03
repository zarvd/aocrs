#[derive(Debug, Default)]
struct Input {
    seeds: Vec<i64>,
    seed_to_soil: RangeMap,
    soil_to_fertilizer: RangeMap,
    fertilizer_to_water: RangeMap,
    water_to_light: RangeMap,
    light_to_temperature: RangeMap,
    temperature_to_humidity: RangeMap,
    humidity_to_location: RangeMap,
}

#[derive(Debug, Default)]
struct RangeMap {
    data: Vec<(i64, i64, i64)>,
}

impl RangeMap {
    pub fn new(mut data: Vec<(i64, i64, i64)>) -> Self {
        data.sort_by_key(|v| v.0);
        Self { data }
    }

    pub fn parse(s: &str) -> Self {
        let data = s
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split('\n')
            .map(|line| line.trim().split(' ').collect::<Vec<_>>())
            .map(|args| {
                let (dst, src, len) = (
                    args[0].parse::<i64>().unwrap(),
                    args[1].parse::<i64>().unwrap(),
                    args[2].parse::<i64>().unwrap(),
                );
                (src, dst, len)
            })
            .collect::<Vec<_>>();
        Self::new(data)
    }

    pub fn get(&self, src: i64) -> i64 {
        // TODO: binary search for bigger map
        for &(s, d, len) in &self.data {
            if src < self.data[0].0 {
                return src;
            }
            if s <= src && src < s + len {
                return d + (src - s);
            }
        }
        src
    }

    pub fn range(&self, (mut start, end): (i64, i64)) -> Vec<(i64, i64)> {
        let mut rv = vec![];
        if start < self.data[0].0 {
            let size = (end - start + 1).min(self.data[0].0 - start);
            rv.push((start, start + size - 1));
            start += size;
        }
        if start > end {
            return rv;
        }
        for &(s, d, len) in &self.data {
            if end < s || start > end {
                break;
            }

            if s <= start && start <= s + len - 1 {
                let offset = start - s;
                let size = end.min(s + len - 1) - start + 1;
                rv.push((d + offset, d + offset + size - 1));
                start += size;
            }
        }
        if start <= end {
            rv.push((start, end));
        }

        rv
    }
}

pub fn solve_part1(input: String) -> String {
    part1(normalize_input(input)).to_string()
}

pub fn solve_part2(input: String) -> String {
    part2(normalize_input(input)).to_string()
}

fn normalize_input(s: String) -> Input {
    let parts = s.split("\n\n").collect::<Vec<_>>();

    let mut input = Input::default();
    input.seeds = parts[0]
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(' ')
        .map(|v| v.trim())
        .map(|v| v.parse::<i64>().unwrap())
        .collect();
    input.seed_to_soil = RangeMap::parse(parts[1]);
    input.soil_to_fertilizer = RangeMap::parse(parts[2]);
    input.fertilizer_to_water = RangeMap::parse(parts[3]);
    input.water_to_light = RangeMap::parse(parts[4]);
    input.light_to_temperature = RangeMap::parse(parts[5]);
    input.temperature_to_humidity = RangeMap::parse(parts[6]);
    input.humidity_to_location = RangeMap::parse(parts[7]);

    input
}

fn part1(input: Input) -> i64 {
    let mut rv = i64::MAX;
    for seed in input.seeds {
        let soil = input.seed_to_soil.get(seed);
        let fertilizer = input.soil_to_fertilizer.get(soil);
        let water = input.fertilizer_to_water.get(fertilizer);
        let light = input.water_to_light.get(water);
        let temperature = input.light_to_temperature.get(light);
        let humidity = input.temperature_to_humidity.get(temperature);
        let location = input.humidity_to_location.get(humidity);
        rv = rv.min(location);
    }

    rv
}

fn part2(input: Input) -> i64 {
    let mut rv = i64::MAX;
    for range in input.seeds.chunks(2) {
        let init = range[0];
        let end = init + range[1] - 1;

        let location = input
            .seed_to_soil
            .range((init, end))
            .into_iter()
            .fold(Vec::new(), |mut acc, range| {
                acc.extend(input.soil_to_fertilizer.range(range));
                acc
            })
            .into_iter()
            .fold(Vec::new(), |mut acc, range| {
                acc.extend(input.fertilizer_to_water.range(range));
                acc
            })
            .into_iter()
            .fold(Vec::new(), |mut acc, range| {
                acc.extend(input.water_to_light.range(range));
                acc
            })
            .into_iter()
            .fold(Vec::new(), |mut acc, range| {
                acc.extend(input.light_to_temperature.range(range));
                acc
            })
            .into_iter()
            .fold(Vec::new(), |mut acc, range| {
                acc.extend(input.temperature_to_humidity.range(range));
                acc
            })
            .into_iter()
            .fold(Vec::new(), |mut acc, range| {
                acc.extend(input.humidity_to_location.range(range));
                acc
            })
            .into_iter()
            .min()
            .unwrap()
            .0;

        rv = rv.min(location);
    }

    rv
}
