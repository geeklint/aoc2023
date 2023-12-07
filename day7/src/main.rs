fn main1() {
    let mut card_priority = [
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];
    card_priority.reverse();
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum Kind {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }
    let input = std::fs::read_to_string("input").unwrap();
    let mut hands = vec![];
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let hand = words.next().unwrap();
        let bid = words.next().unwrap();
        let hand = hand
            .chars()
            .map(|c| card_priority.into_iter().position(|c2| c == c2).unwrap())
            .collect::<Vec<_>>();
        let mut occur = [0; 13];
        for card in hand.iter().copied() {
            occur[card] += 1;
        }
        occur.sort();
        let hand_kind = match occur {
            [.., 5] => Kind::FiveOfAKind,
            [.., 4] => Kind::FourOfAKind,
            [.., 2, 3] => Kind::FullHouse,
            [.., 3] => Kind::ThreeOfAKind,
            [.., 2, 2] => Kind::TwoPair,
            [.., 2] => Kind::OnePair,
            [..] => Kind::HighCard,
        };
        hands.push((hand_kind, hand, bid));
    }
    hands.sort();
    let result: usize = hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, _, bid))| (i + 1) * bid.parse::<usize>().unwrap())
        .sum();
    dbg!(result);
}

fn main() {
    let mut card_priority = [
        'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
    ];
    card_priority.reverse();
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum Kind {
        HighCard,
        OnePair,
        TwoPair,
        ThreeOfAKind,
        FullHouse,
        FourOfAKind,
        FiveOfAKind,
    }
    let input = std::fs::read_to_string("input").unwrap();
    let mut hands = vec![];
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let hand = words.next().unwrap();
        let bid = words.next().unwrap();
        let hand = hand
            .chars()
            .map(|c| card_priority.into_iter().position(|c2| c == c2).unwrap())
            .collect::<Vec<_>>();
        let hand_kind = (1..card_priority.len())
            .map(|jsub| {
                let mut occur = [0; 13];
                for card in hand.iter().copied() {
                    let card = if card == 0 { jsub } else { card };
                    occur[card] += 1;
                }
                occur.sort();
                match occur {
                    [.., 5] => Kind::FiveOfAKind,
                    [.., 4] => Kind::FourOfAKind,
                    [.., 2, 3] => Kind::FullHouse,
                    [.., 3] => Kind::ThreeOfAKind,
                    [.., 2, 2] => Kind::TwoPair,
                    [.., 2] => Kind::OnePair,
                    [..] => Kind::HighCard,
                }
            })
            .max();
        hands.push((hand_kind, hand, bid));
    }
    hands.sort();
    let result: usize = hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, _, bid))| (i + 1) * bid.parse::<usize>().unwrap())
        .sum();
    dbg!(result);
}
