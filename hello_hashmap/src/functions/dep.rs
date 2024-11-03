use std::io;
use std::collections::HashMap;


pub fn company_dep() -> () {

    let mut company = HashMap::new();

    let mut employee: String = String::from("");
    let mut department: String = String::from("");

    while employee != "exit" && department != "exit" {
        employee = String::new();
        department = String::new();

        println!("Insert the name of the employee: ");
        io::stdin()
            .read_line(&mut employee)
            .expect("Failed to read name");
        employee.pop();
        println!("Insert the name of the department in which {employee} works: ");

        io::stdin()
            .read_line(&mut department)
            .expect("Failed to read the department!");
        department.pop();
        
        if employee != "exit" && department != "exit" {
            company.insert(employee.clone(), department.clone());
        }
    }

    for (key, value) in &company {
        println!("{key} is working in {value}");
    }
}