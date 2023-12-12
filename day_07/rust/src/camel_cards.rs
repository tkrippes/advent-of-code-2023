use super::hand::Hand;

#[derive(Clone, PartialEq, Eq, Ord)]
struct HandBid {
    hand: Hand,
    bid: u64,
}

impl HandBid {
    fn build(hand: Hand, bid: u64) -> Self {
        HandBid { hand, bid }
    }
}

impl PartialOrd for HandBid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}

pub struct CamelCards {
    hand_bids: Vec<HandBid>,
}

impl CamelCards {
    pub fn try_build(inputs: Vec<&str>, consider_jokers: bool) -> Option<Self> {
        let mut hand_bids = Vec::new();

        for input in inputs {
            let camel_cards_input = input.split_whitespace().collect::<Vec<&str>>();

            if let (Some(hand_input), Some(bid_input)) =
                (camel_cards_input.first(), camel_cards_input.last())
            {
                if let (Some(hand), Some(bid)) = (
                    Hand::try_build(hand_input, consider_jokers),
                    Self::try_build_bid(bid_input),
                ) {
                    hand_bids.push(HandBid::build(hand, bid));
                } else {
                    println!("Cannot parse camel cards");
                    return None;
                }
            } else {
                println!(
                    "Cannot parse camel cards, did not find cards or bid inputs in {}",
                    input
                );
                return None;
            }
        }

        Some(CamelCards { hand_bids })
    }

    fn try_build_bid(bid_input: &str) -> Option<u64> {
        match bid_input.parse::<u64>() {
            Ok(bid) => Some(bid),
            Err(bid_parsing_error) => {
                println!("Cannot parse bid, {}", bid_parsing_error);
                None
            }
        }
    }

    pub fn get_ranked_bids(&self) -> Vec<u64> {
        let mut ranked_bids = Vec::new();

        let mut hand_bids = self.hand_bids.clone();
        hand_bids.sort();
        hand_bids.reverse();

        for hand_bid in &hand_bids {
            ranked_bids.push(hand_bid.bid);
        }

        ranked_bids
    }
}
