mod test_main;

use std::fs::File;
use std::io::{BufReader, BufRead};
use log::{debug, info};

#[derive(Debug)]
struct Field {
    identifier: String,
    lower_range: (i32, i32),
    higher_range: (i32, i32)
}

fn is_val_valid(val: i32, field: &Field) -> bool {
    let low_valid: bool = field.lower_range.0 <= val && val <= field.lower_range.1;
    let high_valid: bool = field.higher_range.0 <= val && val <= field.higher_range.1;
    return low_valid || high_valid;
}

fn is_val_valid_for_any_field(val: i32, fields: &Vec<Field>) -> bool {
    for field in fields {
        if is_val_valid(val, field) {
            return true;
        }
    }
    return false;
}

fn is_ticket_valid(ticket: &Vec<i32>, fields: &Vec<Field>) -> bool {
    for &val in ticket {
        if !is_val_valid_for_any_field(val, fields) {
            return false;
        }
    }
    return true;
}

fn is_field_valid_for_all_vals(vals:& Vec<i32>, field: &Field) -> bool {
    for &val in vals {
        if !is_val_valid(val, field) {
            return false;
        }
    }
    return true;
}

fn build_field(identifier: String, lower_range: String, higher_range: String) -> Field {
    debug!("{:?}, {:?}", lower_range, higher_range);
    let lower_split = lower_range.split("-");
    let lower_tuple: Vec<i32> = lower_split.map(|s| s.trim().parse::<i32>().unwrap()).collect();
    let higher_split = higher_range.split("-");
    let higher_tuple: Vec<i32> = higher_split.map(|s| s.trim().parse::<i32>().unwrap()).collect();
    return Field {identifier,
        lower_range: (lower_tuple[0], lower_tuple[1]),
        higher_range: (higher_tuple[0], higher_tuple[1])}
}

fn read_input_data(file_name: &str) -> (Vec<Field>, Vec<i32>, Vec<Vec<i32>>) {
    let f = File::open(file_name).unwrap();
    let mut f = BufReader::new(f);

    let mut current_line: String = String::new();
    let mut field_list: Vec<Field> = Vec::new();
    let mut identifier: String;
    let mut outer_data: Vec<String>;
    let mut ranges_list: Vec<String>;
    let mut tickets: Vec<Vec<i32>> = Vec::new();
    f.read_line(&mut current_line).unwrap();
    while current_line.trim() != "" {
        outer_data = current_line.trim().split(":").map(|s| String::from(s.trim())).collect();
        identifier = outer_data.remove(0);
        ranges_list = outer_data[0].trim().split("or").map(|s| String::from(s.trim())).collect();
        debug!("{:?}", ranges_list);
        field_list.push(build_field(identifier, ranges_list.remove(0), ranges_list.remove(0)));
        current_line.clear();
        f.read_line(&mut current_line).unwrap();
    }
    debug!("{:?}", current_line);
    current_line.clear();
    f.read_line(&mut current_line).unwrap();
    debug!("{:?}", current_line);
    current_line.clear();
    f.read_line(&mut current_line).unwrap();
    let my_ticket: Vec<i32> = current_line.trim().split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    debug!("{:?}", current_line);
    current_line.clear();
    f.read_line(&mut current_line).unwrap();
    debug!("{:?}", current_line);
    current_line.clear();
    f.read_line(&mut current_line).unwrap();
    debug!("{:?}", current_line);
    current_line.clear();
    f.read_line(&mut current_line).unwrap();
    while current_line.trim() != "" {
        debug!("{:?}", current_line);
        tickets.push(current_line.trim().split(",").map(|s| s.parse::<i32>().unwrap()).collect());
        current_line.clear();
        f.read_line(&mut current_line).unwrap();
    }
    return (field_list, my_ticket, tickets)
}

fn solution_part_1(file_name: &str) -> i32 {
    let (fields, _my_tickets, other_tickets) = read_input_data(file_name);
    let mut scanning_error = 0;
    for ticket in other_tickets {
        for val in ticket {
            if !is_val_valid_for_any_field(val, &fields) {
                debug!("{:?}", val);
                scanning_error += val;
            }
        }
    }
    return scanning_error;
}

fn transpose_tickets(tickets: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut current_vals: Vec<i32>;
    let mut transposed_tickets: Vec<Vec<i32>> = Vec::new();
    for i in 0..tickets[0].len() {
        current_vals = Vec::new();
        for j in 0..tickets.len() {
            current_vals.push(tickets[j][i]);
        }
        transposed_tickets.push(current_vals);
    }
    return transposed_tickets;
}

fn solution_part_2(file_name: &str) -> i64 {
    let (fields, my_ticket, mut other_tickets) = read_input_data(file_name);
    let mut valid_ticket_list: Vec<Vec<i32>> = Vec::new();
    while other_tickets.len() > 0 {
        let current_ticket = other_tickets.remove(0);
        if is_ticket_valid(&current_ticket, &fields) {
            valid_ticket_list.push(current_ticket);
        }
    }
    debug!("{:?}", valid_ticket_list);
    let transposed_valid_tickets = transpose_tickets(valid_ticket_list);
    let mut valid_fields_indexed: Vec<(usize, Vec<String>)> = Vec::new();
    let mut current_fields: Vec<String>;
    for (i, vals) in transposed_valid_tickets.iter().enumerate() {
        current_fields = Vec::new();
        for field in &fields {
            if is_field_valid_for_all_vals(&vals, field) {
                current_fields.push(field.identifier.clone())
            }
        }
        valid_fields_indexed.push((i, current_fields));
    }
    valid_fields_indexed.sort_by_key( |fields| fields.1.len());
    debug!("{:?}", valid_fields_indexed);
    let mut index_list: Vec<usize> = Vec::new();
    let mut valid_fields: Vec<Vec<String>> = Vec::new();
    for (i, field_list) in valid_fields_indexed {
        index_list.push(i);
        valid_fields.push(field_list);
    }
    let mut dense_valid_list: Vec<String> = Vec::new();
    for field_list in valid_fields {
        for field in field_list {
            debug!("{:?}", field);
            if !dense_valid_list.contains(&field) {
                dense_valid_list.push(field);
                break;
            }
        }
    }
    debug!("{:?}", dense_valid_list);
    let mut field_val: i64 = 1;
    for (i, field_name) in dense_valid_list.iter().enumerate() {
        if field_name.contains("departure") {
            field_val = field_val * (my_ticket[index_list[i]] as i64);
        }
    }
    return field_val;
}


fn main() {
    env_logger::init();
    info!("Scanning Error: {}", solution_part_1("inputData.txt"));
    info!("Multiplied field values: {}", solution_part_2("inputData.txt"));
}
