
struct ServiceLevel {
    level: &'static str,
    office_administrator: &'static str,
    academic: &'static str,
    lawyer: &'static str,
    teacher: &'static str,
}

fn main() {
    
    let level_table: Vec<ServiceLevel> = vec![
        ServiceLevel {
            level: "APS 1-2",
            office_administrator: "Intern",
            academic: "-",
            lawyer: "Paralegal",
            teacher: "Placement",
        },
        ServiceLevel {
            level: "APS 3-5",
            office_administrator: "Administrator",
            academic: "Research Assistant",
            lawyer: "Junior Associate",
            teacher: "Classroom Teacher",
        },
        ServiceLevel {
            level: "APS 5-8",
            office_administrator: "Senior Administrator",
            academic: "PhD Candidate",
            lawyer: "Associate",
            teacher: "Snr Teacher",
        },
        ServiceLevel {
            level: "EL1 8-10",
            office_administrator: "Office Manager",
            academic: "Post-Doc Researcher",
            lawyer: "Senior Associate 1-2",
            teacher: "Leading Teacher",
        },
        ServiceLevel {
            level: "EL2 10-13",
            office_administrator: "Director",
            academic: "Senior Lecturer",
            lawyer: "Senior Associate 3-4",
            teacher: "Deputy Principal",
        },
        ServiceLevel {
            level: "SES",
            office_administrator: "CEO",
            academic: "Dean",
            lawyer: "Partner",
            teacher: "Principal",
        },
    ];

    
    let staff_role = "Associate";
    let staff_category = "Lawyer";

    
    validate_staff_level(&level_table, staff_role, staff_category);

    
    let staff_role_2 = "Director";
    let staff_category_2 = "Office Administrator";
    validate_staff_level(&level_table, staff_role_2, staff_category_2);
}


fn validate_staff_level(table: &Vec<ServiceLevel>, role: &str, category: &str) {
    println!("\n---");
    println!(" Validating Staff: **{}** in **{}** category.", role, category);

    
    let found_level = table.iter()
        .find(|entry| {
            // Check the staff role against the corresponding field in the struct entry
            match category {
                "Office Administrator" => entry.office_administrator == role,
                "Academic" => entry.academic == role,
                // Using .contains() to handle ranges like 'Senior Associate 1-2'
                "Lawyer" => entry.lawyer.contains(role),
                "Teacher" => entry.teacher == role,
                _ => false, // Default to false if the category is invalid
            }
        })
        
        .map(|entry| entry.level);

    
    match found_level {
        Some(level) => {
            println!("Validation successful: The staff member holds position **{}**.", level);
        }
        None => {
            println!("Validation failed: Role **'{}'** not found in the **{}** category.", role, category);
        }
    }
}