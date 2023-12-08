struct Ranges {
    destination_start_range: u64,
    source_start_range: u64,
    range_lengths: u64,
}

impl Ranges {
    fn try_build(input: &str) -> Option<Self> {
        todo!()
    }
}

struct Map {
    ranges: Vec<Ranges>,
}

impl Map {
    fn try_build(input: Vec<&str>) -> Option<Self> {
        todo!()
    }

    fn get_destination_from_source(&self, source: u64) -> u64 {
        todo!()
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
        todo!()
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

        let location = self
            .humidity_to_location_map
            .get_destination_from_source(humidity);

        location
    }
}

pub struct Almanac {
    seeds: Vec<u64>,
    maps: Maps,
}

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
                        println!("Cannot parse seeds, {}", seed_parsing_error);
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

    pub fn get_locations_from_seeds(&self) -> Vec<u64> {
        let mut locations = Vec::new();

        for seed in &self.seeds {
            locations.push(self.maps.get_location_from_seed(*seed));
        }

        locations
    }
}
