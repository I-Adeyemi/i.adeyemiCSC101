use std::io;

fn geopo_merger() {
    //arr1 = name_commissioner
    //arr2 = ministry
    //arr3 = geopolitical_zone
    let mut arr1:[&str;5] = ["Aigbogun Alamba Dauda", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    let mut arr2:[&str;5] = ["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let mut arr3:[&str;5] = ["south West", "North East", "South South", "South West", "South East"];
    

}
fn pub_service() {
    for i in 0..5 {   
    let mut public_servant = String::new();
    let mut office_admin = String::new();
    let mut academic = String::new();
    let mut lawyer = String::new();
    let mut teacher = String::new();
    
    //arr4 = public_servant
    //arr5 = office_admin
    //arr6 = lawyer
    //arr7 = teacher
    let mut arr4 = [&str;6] = ["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "ES 14"];
    let mut arr5 = [&str;6] = ["Intern","Administrator","Senior Administrator","Office Manager", "Director", "CEO"];
    let mut arr6 = [&str;6] = ["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let mut arr7 = [&str;6] = ["Placement", "Classroom Teacher", "Snr Teacher", "Leading Teacher", "Deputy Principal", "Principla"];
    }
}

fn main() {
    println!("Welcome to Admin to the Information Service Department of the Abuja cloud backup system");
    let mut input1 = String::new();

    println!("Which program would you like to run?");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    if program == 1 {
    println!("{:?}", geopo_merger);
    } else if program == 2 {
   
    let mut experience_input = String::new();
    println!("\nHow many years have you been working with our organization");
    io::stdin().read_line(&mut experience_input).expect("Not a valid input");
    let experience:i32 = experience_input.trim().parse().expect("Failed to read number"); 

    if experience <= 3 {
        println!("You're a {:?}", arr5);
    }
    
    else if experience <= 4 && >= 7{
    println!("You're {:?}", arr6);
    }
    
    else if experience <= 5 && >= 8 {
    println!("You're {:?}", arr6);
    }
    
    else if experience <= 9 && >= 12  {
    println!("You're {:?}", arr6);
    }
    else if experience <= 13 && >= 16  { 
    println!("You're {:?}", arr6);




    }
}
