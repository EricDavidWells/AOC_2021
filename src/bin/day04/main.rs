use std::borrow::Borrow;
use libaoc::libaoc;
use std::collections::HashMap;
use std::hash::Hash;


#[derive(Clone)]
struct Card
{
    num_to_loc_map: HashMap<u8, Loc>,
    loc_to_entry_map: HashMap<Loc, Entry>,
    status: bool,
    rows: u8,
    cols: u8
}

impl Default for Card
{
    fn default() -> Card
    {
        Card{
            num_to_loc_map: HashMap::new(),
            loc_to_entry_map: HashMap::new(),
            status: false,
            rows: 0,
            cols: 0}
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Entry
{
    val: u8,
    status: bool
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Loc
{
    row: u8,
    col: u8,
}

fn parse_input(filename: &str) -> (Vec<u8>, Vec<Card>)
{
    let mut draw_order: Vec<u8> = Vec::new();
    let mut cards: Vec<Card> = Vec::new();

    let input_lines: Vec<String> = libaoc::parse_file(&filename);

    for val in input_lines[0].split(',')
    {
        draw_order.push(val.parse::<u8>().unwrap());
    }

    let mut n = 2;
    let mut row = 0;
    let mut col = 0;

    let mut card: Card = Card::default();

    while n < input_lines.len()
    {
        if input_lines[n] == ""
        {
            card.cols = col;
            card.rows = row;

            cards.push(card.clone());
            card.num_to_loc_map.clear();
            card.loc_to_entry_map.clear();

            row = 0;
        }
        else
        {
            col = 0;
            let nums = input_lines[n].split(" ");
            for num in nums
            {
                if num == "" {continue;}
                let loc = Loc{row: row, col: col};
                let entry = Entry {val: num.parse::<u8>().unwrap(), status: false};

                card.num_to_loc_map.insert(num.parse::<u8>().unwrap(), loc);
                card.loc_to_entry_map.insert(loc, entry);
                col += 1;
            }
            row += 1;
        }
        n += 1;
    }

    card.cols = col;
    card.rows = row;
    cards.push(card.clone());

    card.num_to_loc_map.clear();
    card.loc_to_entry_map.clear();

    (draw_order, cards)

}

fn play_game(draw_order: &Vec<u8>, cards: &mut Vec<Card>) -> u32
{
    let mut ret = 0;

    for draw in draw_order
    {
        for i in (0..cards.len())
        {
            let mut card = &mut cards[i];

            if card.num_to_loc_map.contains_key(draw)
            {
                let loc = card.num_to_loc_map.get(draw).unwrap();
                card.loc_to_entry_map.get_mut(loc).unwrap().status = true;
            }
            check_card_complete(&mut card);

            if (card.status)
            {
                ret = sum_unmarked(card) * (*draw as u32);
                return ret
            }
        }
    }

    ret
}

fn play_game_2(draw_order: &Vec<u8>, cards: &mut Vec<Card>) -> u32
{
    let mut ret = 0;
    let mut ind = 0;
    let mut done = false;

    for draw in draw_order
    {
        for i in (0..cards.len())
        {
            let mut card = &mut cards[i];

            if card.num_to_loc_map.contains_key(draw)
            {
                let loc = card.num_to_loc_map.get(draw).unwrap();
                card.loc_to_entry_map.get_mut(loc).unwrap().status = true;
            }
            check_card_complete(&mut card);

            {
                done = count_incompletes(cards);
            }
            if (done)
            {
                ind = i;
                break;
            }
        }

        if (done)
        {
            return sum_unmarked(&cards[ind]) * (*draw as u32);
        }
    }

    ret
}

fn count_incompletes(cards: & Vec<Card>) -> bool
{
    let mut incompletes = 0;
    let mut ind = 0;

    for card in cards
    {
        ind += 1;
        if card.status == false{incompletes += 1}
    }
    ind -= 1;

    incompletes == 0
}

fn check_card_complete(card: &mut Card)
{
    for i in 0..card.rows
    {
        let mut check = true;

        for j in 0..card.cols
        {
            if card.loc_to_entry_map.get(&Loc{row: i, col: j}).unwrap().status == false
            {
                check = false;
                break;
            }
        }
        if check {card.status = true}
    }

    for j in 0..card.cols
    {
        let mut check = true;

        for i in 0..card.rows
        {
            if card.loc_to_entry_map.get(&Loc{row: i, col: j}).unwrap().status == false
            {
                check = false;
                break;
            }
        }
        if check {card.status = true}
    }
}

fn sum_unmarked(card: &Card) -> u32
{
    let mut sum: u32 = 0;

    for i in 0..card.rows
    {
        for j in 0..card.cols
        {
            if (card.loc_to_entry_map.get(&Loc{row: i, col: j}).unwrap().status == false)
            {
                sum += card.loc_to_entry_map.get(&Loc{row: i, col: j}).unwrap().val as u32;
            }
        }
    }

    sum
}

fn main()
{
    let filename = "src/bin/day04/input.txt";
    // let filename = "src/bin/day04/example_input.txt";

    let (draw_order, mut cards) = parse_input(&filename);

    let result = play_game(&draw_order, &mut cards);
    println!("{}", result);

    let result = play_game_2(&draw_order, &mut cards);
    println!("{}", result);

}