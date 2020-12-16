use std::{collections::HashSet, usize};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Default, Debug)]
struct Either<T> {
    a: T,
    b: T,
}

#[derive(Default, Debug)]
struct Departure {
    location: Either<(u32, u32)>,
    station: Either<(u32, u32)>,
    platform: Either<(u32, u32)>,
    track: Either<(u32, u32)>,
    date: Either<(u32, u32)>,
    time: Either<(u32, u32)>,
}

#[derive(Default, Debug)]
struct Arrival {
    location: Either<(u32, u32)>,
    station: Either<(u32, u32)>,
    platform: Either<(u32, u32)>,
    track: Either<(u32, u32)>,
}

enum FieldType {
    DLocation,
    DStation,
    DPlatform,
    DTrack,
    DDate,
    DTime,

    ALocation,
    AStation,
    APlatform,
    ATrack,

    Class,
    Duration,
    Price,
    Route,
    Row,
    Seat,
    Train,
    Ttype,
    Wagon,
    Zone,
}

#[derive(Default, Debug)]
struct General {
    class: Either<(u32, u32)>,
    duration: Either<(u32, u32)>,
    price: Either<(u32, u32)>,
    route: Either<(u32, u32)>,
    row: Either<(u32, u32)>,
    seat: Either<(u32, u32)>,
    train: Either<(u32, u32)>,
    ttype: Either<(u32, u32)>,
    wagon: Either<(u32, u32)>,
    zone: Either<(u32, u32)>,
}

struct FieldRanges {
    general: General,
    departure: Departure,
    arrival: Arrival,
}

#[derive(Default, Debug)]
struct Ticket {
    fields: Vec<u32>,
}

fn parse_either(input: &str) -> Either<(u32, u32)> {
    let (a, b) = input.split_once(" or ").unwrap();

    let (a0, a1) = a.split_once("-").unwrap();
    let (b0, b1) = b.split_once("-").unwrap();

    Either {
        a: (a0.parse().unwrap(), a1.parse().unwrap()),
        b: (b0.parse().unwrap(), b1.parse().unwrap()),
    }
}

fn parse_ticket(input: &str) -> Ticket {
    Ticket {
        fields: input.split(',').map(|v| v.parse().unwrap()).collect(),
    }
}

pub fn aoc_16(reader: BufReader<File>) -> String {
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let departure = Departure {
        location: parse_either(lines[0].split(": ").nth(1).unwrap()),
        station: parse_either(lines[1].split(": ").nth(1).unwrap()),
        platform: parse_either(lines[2].split(": ").nth(1).unwrap()),
        track: parse_either(lines[3].split(": ").nth(1).unwrap()),
        date: parse_either(lines[4].split(": ").nth(1).unwrap()),
        time: parse_either(lines[5].split(": ").nth(1).unwrap()),
    };

    let arrival = Arrival {
        location: parse_either(lines[6].split(": ").nth(1).unwrap()),
        station: parse_either(lines[7].split(": ").nth(1).unwrap()),
        platform: parse_either(lines[8].split(": ").nth(1).unwrap()),
        track: parse_either(lines[9].split(": ").nth(1).unwrap()),
    };

    let general = General {
        class: parse_either(lines[10].split(": ").nth(1).unwrap()),
        duration: parse_either(lines[11].split(": ").nth(1).unwrap()),
        price: parse_either(lines[12].split(": ").nth(1).unwrap()),
        route: parse_either(lines[13].split(": ").nth(1).unwrap()),
        row: parse_either(lines[14].split(": ").nth(1).unwrap()),
        seat: parse_either(lines[15].split(": ").nth(1).unwrap()),
        train: parse_either(lines[16].split(": ").nth(1).unwrap()),
        ttype: parse_either(lines[17].split(": ").nth(1).unwrap()),
        wagon: parse_either(lines[18].split(": ").nth(1).unwrap()),
        zone: parse_either(lines[19].split(": ").nth(1).unwrap()),
    };

    let my_ticket = parse_ticket(&lines[22]);

    let mut other_tickets = Vec::new();
    for i in 25..lines.len() {
        other_tickets.push(parse_ticket(&lines[i]));
    }

    let field_ranges = FieldRanges {
        general: general,
        arrival: arrival,
        departure: departure,
    };

    let part1 = part_1(&other_tickets, &field_ranges);
    assert_eq!(26009, part1);

    let part2 = part_2(other_tickets, &field_ranges, &my_ticket);
    assert_ne!(18560365686799, part2);
    assert_ne!(8911854359477, part2);
    assert_ne!(1023837308309, part2);
    assert_ne!(275448615941, part2);
    assert_ne!(12177583059479, part2);

    format!("Part 1: {}\n\tPart 2: {}", part1, part2)
}

// !59329

impl Either<(u32, u32)> {
    pub fn pick(&self, num: u32) -> Option<(u32, u32)> {
        if num >= self.a.0 && num <= self.a.1 {
            Some(self.a)
        } else if num >= self.b.0 && num <= self.b.1 {
            Some(self.b)
        } else {
            None
        }
    }
}

fn validate_ticket(ticket: &Ticket, field_ranges: &FieldRanges) -> u32 {
    ticket.fields.iter().fold(0, |acc, f| {
        if field_ranges.general.class.pick(*f).is_some()
            || field_ranges.general.duration.pick(*f).is_some()
            || field_ranges.general.price.pick(*f).is_some()
            || field_ranges.general.route.pick(*f).is_some()
            || field_ranges.general.row.pick(*f).is_some()
            || field_ranges.general.seat.pick(*f).is_some()
            || field_ranges.general.train.pick(*f).is_some()
            || field_ranges.general.ttype.pick(*f).is_some()
            || field_ranges.general.wagon.pick(*f).is_some()
            || field_ranges.general.zone.pick(*f).is_some()
            || field_ranges.departure.location.pick(*f).is_some()
            || field_ranges.departure.platform.pick(*f).is_some()
            || field_ranges.departure.station.pick(*f).is_some()
            || field_ranges.departure.time.pick(*f).is_some()
            || field_ranges.departure.track.pick(*f).is_some()
            || field_ranges.departure.date.pick(*f).is_some()
            || field_ranges.arrival.track.pick(*f).is_some()
            || field_ranges.arrival.station.pick(*f).is_some()
            || field_ranges.arrival.platform.pick(*f).is_some()
            || field_ranges.arrival.location.pick(*f).is_some()
        {
            acc
        } else {
            acc + f
        }
    })
}

fn part_1(tickets: &Vec<Ticket>, field_ranges: &FieldRanges) -> u32 {
    tickets
        .iter()
        .map(|t| validate_ticket(t, field_ranges))
        .sum::<u32>()
}

fn part_2(tickets: Vec<Ticket>, field_ranges: &FieldRanges, my_ticket: &Ticket) -> usize {
    let valid_tickets: Vec<&Ticket> = tickets
        .iter()
        .filter(|t| validate_ticket(t, &field_ranges) == 0)
        .collect();

    let mut skips = HashSet::new();

    let general_class_index =
        get_field_index(&valid_tickets, &field_ranges.general.class.a, &mut skips);
    let general_duration_index =
        get_field_index(&valid_tickets, &field_ranges.general.duration.a, &mut skips);
    let general_price_index =
        get_field_index(&valid_tickets, &field_ranges.general.price.a, &mut skips);
    let general_route_index =
        get_field_index(&valid_tickets, &field_ranges.general.route.a, &mut skips);
    let general_row_index =
        get_field_index(&valid_tickets, &field_ranges.general.row.a, &mut skips);
    let general_seat_index =
        get_field_index(&valid_tickets, &field_ranges.general.seat.a, &mut skips);
    let general_train_index =
        get_field_index(&valid_tickets, &field_ranges.general.train.a, &mut skips);
    let general_type_index =
        get_field_index(&valid_tickets, &field_ranges.general.ttype.a, &mut skips);
    let general_wagon_index =
        get_field_index(&valid_tickets, &field_ranges.general.wagon.a, &mut skips);
    let general_zone_index =
        get_field_index(&valid_tickets, &field_ranges.general.zone.a, &mut skips);

    let arrival_location_index =
        get_field_index(&valid_tickets, &field_ranges.arrival.location.a, &mut skips);
    let arrival_station_index =
        get_field_index(&valid_tickets, &field_ranges.arrival.station.a, &mut skips);
    let arrival_platform_index =
        get_field_index(&valid_tickets, &field_ranges.arrival.platform.a, &mut skips);
    let arrival_track_index =
        get_field_index(&valid_tickets, &field_ranges.arrival.track.a, &mut skips);

    let departure_location_index = get_field_index(
        &valid_tickets,
        &field_ranges.departure.location.b,
        &mut skips,
    );
    let departure_station_index = get_field_index(
        &valid_tickets,
        &field_ranges.departure.station.b,
        &mut skips,
    );
    let departure_platform_index = get_field_index(
        &valid_tickets,
        &field_ranges.departure.platform.b,
        &mut skips,
    );
    let departure_track_index =
        get_field_index(&valid_tickets, &field_ranges.departure.track.b, &mut skips);
    let departure_date_index =
        get_field_index(&valid_tickets, &field_ranges.departure.date.b, &mut skips);
    let departure_time_index =
        get_field_index(&valid_tickets, &field_ranges.departure.time.b, &mut skips);

    let mut skips = skips.iter().clone().collect::<Vec<&usize>>();
    skips.sort();
    println!("{:?}", skips);

    println!(
        "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
        departure_location_index,
        departure_station_index,
        departure_platform_index,
        departure_track_index,
        departure_date_index,
        departure_time_index,
        general_class_index,
        general_duration_index,
        general_price_index,
        general_route_index,
        general_row_index,
        general_seat_index,
        general_train_index,
        general_type_index,
        general_wagon_index,
        general_zone_index,
        arrival_location_index,
        arrival_station_index,
        arrival_platform_index,
        arrival_track_index,
    );

    my_ticket.fields[departure_location_index] as usize
        * my_ticket.fields[departure_station_index] as usize
        * my_ticket.fields[departure_platform_index] as usize
        * my_ticket.fields[departure_track_index] as usize
        * my_ticket.fields[departure_date_index] as usize
        * my_ticket.fields[departure_time_index] as usize
}

fn get_field_index(
    tickets: &Vec<&Ticket>,
    range: &(u32, u32),
    skips: &mut HashSet<usize>,
) -> usize {
    let mut last_index = 0;
    for i in 0..tickets[0].fields.len() {
        if skips.contains(&i) {
            continue;
        }

        let mut valid = true;
        for j in 0..tickets.len() {
            let f = tickets[j].fields[i];

            if f < range.0 || f > range.1 {
                valid = false;
                break;
            }
        }

        last_index = i;
        if valid {
            break;
        }
    }

    skips.insert(last_index);
    last_index
}
