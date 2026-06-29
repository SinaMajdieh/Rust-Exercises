use std::{collections::HashMap, io::{self, Write}};

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "exit" {
            break;
        }

        handle_command(input, &mut company);
    }
}


fn handle_command(input: &str, company: &mut HashMap<String, Vec<String>>) {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() == 4 && parts[0] == "Add" && parts[2] == "to" {
        let name = parts[1].to_string();
        let department = parts[3].to_string();
        company
            .entry(department)
            .or_insert(Vec::new())
            .push(name);

        return;
    }

    if parts.len() == 2 && parts[0] == "List" {
        let target = parts[1];

        if target == "Company" {
            list_company(company);
        } else {
            list_department(company, target);
        }
    }
}


fn list_department(company: &HashMap<String, Vec<String>>, department: &str) {
    if let Some(employees) = company.get(department) {
        let mut sorted = employees.clone();
        sorted.sort();

        println!("{department} : {}", sorted.join(", "));
        return;
    }

    println!("{department}: (empty)");
}


fn list_company(company: &HashMap<String, Vec<String>>) {
    let mut departments: Vec<_> = company.keys().collect();
    departments.sort();
    for dept in departments {
        let mut employees = company.get(dept).unwrap().clone();
        employees.sort();

        println!("{dept}: {}", employees.join(", "));
    }
}