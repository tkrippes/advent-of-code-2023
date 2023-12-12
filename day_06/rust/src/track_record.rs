use super::boat::Boat;

struct TrackRecord {
    time_in_ms: u32,
    distance_in_mm: u32,
}

impl TrackRecord {
    fn build(time_in_ms: u32, distance_in_mm: u32) -> Self {
        TrackRecord {
            time_in_ms,
            distance_in_mm,
        }
    }

    fn get_number_of_ways_to_beat_track_record(&self, acceleration_rate: u32) -> u32 {
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
        let mut track_records = Vec::new();

        if let (Some(record_times_input), Some(record_distances_input)) =
            (input.first(), input.last())
        {
            if let (Some(record_times), Some(record_distances)) = (
                Self::get_record_times(record_times_input),
                Self::get_record_distances(record_distances_input),
            ) {
                if record_times.len() == record_distances.len() {
                    for (index, record_time) in record_times.iter().enumerate() {
                        if let Some(record_distance) = record_distances.get(index) {
                            track_records.push(TrackRecord::build(*record_time, *record_distance));
                        } else {
                            println!(
                                "Cannot parse track records, cannot find record distance for {}",
                                index
                            );
                            return None;
                        }
                    }

                    Some(TrackRecords { track_records })
                } else {
                    println!("Cannot parse track records, times and distances do not have the same length");
                    None
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

    fn get_record_times(record_times_input: &str) -> Option<Vec<u32>> {
        let mut record_times = Vec::new();

        if let Some(record_times_input) = record_times_input.split(':').last() {
            let record_times_input = record_times_input.split_whitespace().collect::<Vec<&str>>();

            for record_time in record_times_input {
                match record_time.parse::<u32>() {
                    Ok(record_time) => record_times.push(record_time),
                    Err(record_time_parsing_error) => {
                        println!(
                            "Cannot get record times, cannot parse record time, {}",
                            record_time_parsing_error
                        );
                        return None;
                    }
                }
            }

            Some(record_times)
        } else {
            println!(
                "Cannot get record times, could not find record times input in {}",
                record_times_input
            );
            None
        }
    }

    fn get_record_distances(record_distances_input: &str) -> Option<Vec<u32>> {
        let mut record_distances = Vec::new();

        if let Some(record_distances_input) = record_distances_input.split(':').last() {
            let record_distances_input = record_distances_input
                .split_whitespace()
                .collect::<Vec<&str>>();

            for record_distance in record_distances_input {
                match record_distance.parse::<u32>() {
                    Ok(record_distance) => record_distances.push(record_distance),
                    Err(record_distance_parsing_error) => {
                        println!(
                            "Cannot get record distance, cannot parse record distance, {}",
                            record_distance_parsing_error
                        );
                        return None;
                    }
                }
            }

            Some(record_distances)
        } else {
            println!(
                "Cannot get record distances, could not find record distances input in {}",
                record_distances_input
            );
            None
        }
    }

    pub fn get_number_of_ways_to_beat_track_records(&self, acceleration_rate: u32) -> Vec<u32> {
        let mut number_of_ways_to_beat_track_records = Vec::new();

        for track_record in &self.track_records {
            number_of_ways_to_beat_track_records
                .push(track_record.get_number_of_ways_to_beat_track_record(acceleration_rate));
        }

        number_of_ways_to_beat_track_records
    }
}
