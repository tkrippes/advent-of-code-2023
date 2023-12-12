use super::boat::Boat;

struct TrackRecord {
    time_in_ms: u64,
    distance_in_mm: u64,
}

impl TrackRecord {
    fn build(time_in_ms: u64, distance_in_mm: u64) -> Self {
        TrackRecord {
            time_in_ms,
            distance_in_mm,
        }
    }

    fn get_number_of_ways_to_beat_track_record(&self, acceleration_rate: u64) -> u64 {
        let mut number_of_ways_to_beat_track_record = 0;

        let mut boat = Boat::build(acceleration_rate);

        for charging_time in 1..self.time_in_ms {
            boat.charge(charging_time);

            let moving_time = self.time_in_ms - charging_time;
            if boat.distance_covered(moving_time) > self.distance_in_mm {
                number_of_ways_to_beat_track_record += 1;
            }
        }

        number_of_ways_to_beat_track_record
    }
}

pub struct TrackRecords {
    track_records: Vec<TrackRecord>,
}

impl TrackRecords {
    pub fn try_build(input: Vec<&str>) -> Option<Self> {
        if let (Some(record_times_input), Some(record_distances_input)) =
            (input.first(), input.last())
        {
            if let (Some(record_times), Some(record_distances)) = (
                Self::try_get_record_times(record_times_input),
                Self::try_get_record_distances(record_distances_input),
            ) {
                match Self::try_get_track_records(record_times, record_distances) {
                    Some(track_records) => Some(TrackRecords { track_records }),
                    None => None,
                }
            } else {
                println!(
                    "Cannot parse track records, either time or distances could not be parsed"
                );
                None
            }
        } else {
            println!("Cannot parse track records, no times or distances given");
            None
        }
    }

    fn try_get_track_records(
        record_times: Vec<u64>,
        record_distances: Vec<u64>,
    ) -> Option<Vec<TrackRecord>> {
        let mut track_records = Vec::new();

        if record_times.len() == record_distances.len() {
            for (index, record_time) in record_times.iter().enumerate() {
                match record_distances.get(index) {
                    Some(record_distance) => {
                        track_records.push(TrackRecord::build(*record_time, *record_distance))
                    }
                    None => {
                        println!(
                            "Cannot parse track records, cannot find record distance for {}",
                            index
                        );
                        return None;
                    }
                }
            }

            Some(track_records)
        } else {
            println!("Cannot parse track records, times and distances do not have the same length");
            None
        }
    }

    fn try_get_record_times(record_times_input: &str) -> Option<Vec<u64>> {
        if let Some(record_times_input) = record_times_input.split(':').last() {
            match Self::try_get_numbers(record_times_input) {
                Some(record_times) => Some(record_times),
                None => {
                    println!("Cannot get record times");
                    None
                }
            }
        } else {
            println!(
                "Cannot get record times, could not find record times input in {}",
                record_times_input
            );
            None
        }
    }

    fn try_get_record_distances(record_distances_input: &str) -> Option<Vec<u64>> {
        match record_distances_input.split(':').last() {
            Some(record_distances_input) => match Self::try_get_numbers(record_distances_input) {
                Some(record_distances) => Some(record_distances),
                None => {
                    println!("Cannot get record distances");
                    None
                }
            },
            None => {
                println!(
                    "Cannot get record distances, could not find record distances input in {}",
                    record_distances_input
                );
                None
            }
        }
    }

    fn try_get_numbers(numbers_input: &str) -> Option<Vec<u64>> {
        let mut numbers = Vec::new();

        let numbers_input = numbers_input.split_whitespace().collect::<Vec<&str>>();

        for number in numbers_input {
            match number.parse::<u64>() {
                Ok(number) => numbers.push(number),
                Err(number_parsing_error) => {
                    println!("Cannot parse number, {}", number_parsing_error);
                    return None;
                }
            }
        }

        Some(numbers)
    }

    pub fn get_number_of_ways_to_beat_track_records(&self, acceleration_rate: u64) -> Vec<u64> {
        let mut number_of_ways_to_beat_track_records = Vec::new();

        for track_record in &self.track_records {
            number_of_ways_to_beat_track_records
                .push(track_record.get_number_of_ways_to_beat_track_record(acceleration_rate));
        }

        number_of_ways_to_beat_track_records
    }

    pub fn get_number_of_ways_to_beat_single_track_record(&self, acceleration_rate: u64) -> u64 {
        match self.try_get_single_track_record() {
            Some(single_track_record) => {
                single_track_record.get_number_of_ways_to_beat_track_record(acceleration_rate)
            }
            None => {
                println!("Cannot get number of ways to beat single track record");
                0
            }
        }
    }

    fn try_get_single_track_record(&self) -> Option<TrackRecord> {
        let mut record_times = Vec::new();
        let mut record_distances = Vec::new();

        for track_record in &self.track_records {
            record_times.push(track_record.time_in_ms);
            record_distances.push(track_record.distance_in_mm);
        }

        if let (Some(single_record_time), Some(single_record_distance)) = (
            Self::try_get_single_number(record_times),
            Self::try_get_single_number(record_distances),
        ) {
            Some(TrackRecord {
                time_in_ms: single_record_time,
                distance_in_mm: single_record_distance,
            })
        } else {
            println!("Cannot get single track record, either record time or distance could not be parsed");
            None
        }
    }

    fn try_get_single_number(numbers: Vec<u64>) -> Option<u64> {
        let single_number = numbers.iter().map(|number| number.to_string()).fold(
            String::new(),
            |mut sum, number| {
                sum.push_str(&number);
                sum
            },
        );

        match single_number.parse::<u64>() {
            Ok(single_number) => Some(single_number),
            Err(single_number_parsing_error) => {
                println!("Cannot get single number, {}", single_number_parsing_error);
                None
            }
        }
    }
}
