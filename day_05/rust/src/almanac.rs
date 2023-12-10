struct Ranges {
    destination_start_range: u64,
    source_start_range: u64,
    range_lengths: u64,
}

impl Ranges {
    fn try_build(input: &str) -> Option<Self> {
        let range_input = input.split_whitespace().collect::<Vec<&str>>();

        if let (Some(destination_start_range), Some(source_start_range), Some(range_lengths)) =
            (range_input.first(), range_input.get(1), range_input.get(2))
        {
            if let (Ok(destination_start_range), Ok(source_start_range), Ok(range_lengths)) = (
                destination_start_range.parse::<u64>(),
                source_start_range.parse::<u64>(),
                range_lengths.parse::<u64>(),
            ) {
                Some(Ranges {
                    destination_start_range,
                    source_start_range,
                    range_lengths,
                })
            } else {
                println!(
                    "Cannot parse ranges, at least on of the input could not parse into a number in '{}'", input
                );
                None
            }
        } else {
            println!(
                "Cannot parse ranges, did not find 3 ranges inputs in '{}'",
                input
            );
            None
        }
    }

    fn get_destination_from_source(&self, source: u64) -> Option<u64> {
        if self.is_source_in_range(source) {
            Some(source + self.destination_start_range - self.source_start_range)
        } else {
            None
        }
    }

    fn is_source_in_range(&self, source: u64) -> bool {
        let range = self.source_start_range..self.source_start_range + self.range_lengths;
        range.contains(&source)
    }
}

struct Map {
    ranges: Vec<Ranges>,
}

impl Map {
    fn try_build(input: Vec<&str>) -> Option<Self> {
        let mut ranges = Vec::new();

        for input_line in input {
            if let Some(range) = Ranges::try_build(input_line) {
                ranges.push(range);
            } else {
                println!("Cannot parse map");
                return None;
            }
        }

        Some(Map { ranges })
    }

    fn get_destination_from_source(&self, source: u64) -> u64 {
        for range in &self.ranges {
            if let Some(destination) = range.get_destination_from_source(source) {
                return destination;
            }
        }

        source
    }
}

struct Maps {
    seed_to_soil_map: Map,
    soil_to_fertilizer_map: Map,
    fertilizer_to_water_map: Map,
    water_to_light_map: Map,
    light_to_temperature_map: Map,
    temperature_to_humidity_map: Map,
    humidity_to_location_map: Map,
}

impl Maps {
    fn try_build(input: Vec<&str>) -> Option<Self> {
        let mut seed_to_soil_map: Option<Map> = None;
        let mut soil_to_fertilizer_map: Option<Map> = None;
        let mut fertilizer_to_water_map: Option<Map> = None;
        let mut water_to_light_map: Option<Map> = None;
        let mut light_to_temperature_map: Option<Map> = None;
        let mut temperature_to_humidity_map: Option<Map> = None;
        let mut humidity_to_location_map: Option<Map> = None;

        for (line_index, input_line) in input.iter().enumerate() {
            match *input_line {
                "seed-to-soil map:" => {
                    seed_to_soil_map = Map::try_build(Self::get_map_input(&input, line_index + 1))
                }
                "soil-to-fertilizer map:" => {
                    soil_to_fertilizer_map =
                        Map::try_build(Self::get_map_input(&input, line_index + 1))
                }
                "fertilizer-to-water map:" => {
                    fertilizer_to_water_map =
                        Map::try_build(Self::get_map_input(&input, line_index + 1))
                }
                "water-to-light map:" => {
                    water_to_light_map = Map::try_build(Self::get_map_input(&input, line_index + 1))
                }
                "light-to-temperature map:" => {
                    light_to_temperature_map =
                        Map::try_build(Self::get_map_input(&input, line_index + 1))
                }
                "temperature-to-humidity map:" => {
                    temperature_to_humidity_map =
                        Map::try_build(Self::get_map_input(&input, line_index + 1))
                }
                "humidity-to-location map:" => {
                    humidity_to_location_map =
                        Map::try_build(Self::get_map_input(&input, line_index + 1))
                }
                _ => (),
            };
        }

        if let (
            Some(seed_to_soil_map),
            Some(soil_to_fertilizer_map),
            Some(fertilizer_to_water_map),
            Some(water_to_light_map),
            Some(light_to_temperature_map),
            Some(temperature_to_humidity_map),
            Some(humidity_to_location_map),
        ) = (
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map,
        ) {
            Some(Maps {
                seed_to_soil_map,
                soil_to_fertilizer_map,
                fertilizer_to_water_map,
                water_to_light_map,
                light_to_temperature_map,
                temperature_to_humidity_map,
                humidity_to_location_map,
            })
        } else {
            println!("Cannot maps, at least one of the maps is none");
            None
        }
    }

    fn get_map_input<'a>(input: &'a [&str], start_index: usize) -> Vec<&'a str> {
        let mut map_input = Vec::new();
        let mut index = start_index;

        while let Some(input_line) = input.get(index) {
            if input_line.is_empty() {
                break;
            }

            map_input.push(*input_line);
            index += 1;
        }

        map_input
    }

    fn get_location_from_seed(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil_map.get_destination_from_source(seed);

        let fertilizer = self
            .soil_to_fertilizer_map
            .get_destination_from_source(soil);

        let water = self
            .fertilizer_to_water_map
            .get_destination_from_source(fertilizer);

        let light = self.water_to_light_map.get_destination_from_source(water);

        let temperature = self
            .light_to_temperature_map
            .get_destination_from_source(light);

        let humidity = self
            .temperature_to_humidity_map
            .get_destination_from_source(temperature);

        self.humidity_to_location_map
            .get_destination_from_source(humidity)
    }
}

pub struct Almanac {
    seeds: Vec<u64>,
    maps: Maps,
}

// TODO improve performance for part 2
impl Almanac {
    pub fn try_build(input: Vec<&str>) -> Option<Self> {
        let mut seeds: Option<Vec<u64>> = None;

        if let Some(seeds_input) = input.first() {
            seeds = Almanac::try_build_seeds(seeds_input);
        }

        let maps = Maps::try_build(input[2..].to_vec());

        if let (Some(seeds), Some(maps)) = (seeds, maps) {
            Some(Almanac { seeds, maps })
        } else {
            println!("Cannot parse almanac");
            None
        }
    }

    fn try_build_seeds(input: &str) -> Option<Vec<u64>> {
        let mut seeds = Vec::new();

        if let Some(seeds_input) = input.split(':').last() {
            for seed in seeds_input.split_whitespace() {
                match seed.parse::<u64>() {
                    Ok(seed) => seeds.push(seed),
                    Err(seed_parsing_error) => {
                        println!("Cannot parse seed '{}', {}", seed, seed_parsing_error);
                        return None;
                    }
                }
            }
        } else {
            println!("Cannot find seeds input");
            return None;
        }

        Some(seeds)
    }

    pub fn get_locations_from_seeds(&self, consider_seed_range: bool) -> Vec<u64> {
        let mut locations = Vec::new();

        if let Some(seeds) = self.get_seeds(consider_seed_range) {
            for seed in seeds {
                locations.push(self.maps.get_location_from_seed(seed));
            }
        } else {
            println!("Cannot get locations from seeds");
        }

        locations
    }

    fn get_seeds(&self, consider_seed_range: bool) -> Option<Vec<u64>> {
        if consider_seed_range {
            if self.seeds.len() % 2 == 0 {
                let mut seeds = Vec::new();

                for i in (0..self.seeds.len()).step_by(2) {
                    let start_seed = *self.seeds.get(i).unwrap();
                    let length = *self.seeds.get(i + 1).unwrap();

                    for j in 0..length {
                        let seed = start_seed + j;
                        if !seeds.contains(&seed) {
                            seeds.push(seed);
                        }
                    }
                }

                Some(seeds)
            } else {
                println!("Could not get seed range, number of seed input is odd");
                None
            }
        } else {
            Some(self.seeds.clone())
        }
    }
}
