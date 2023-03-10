fn demonstrate_heap_stack_difference() {
    let greeting: &str = "Hello";
    let mut greeting_on_heap: String = String::from("Hello");

    greeting_on_heap.push_str(", world!");
    let greeting: String = format!("{}{}", greeting, ", world!");

    println!("String on heap: {greeting_on_heap}");
    println!("String on stack: {greeting}");
}

fn demonstrate_heap_cloning() {
    let name: String = String::from("Jamil Bousquet");
    let name_clone: String = name.clone();

    println!("\nComparing {name} to {name_clone}");
}

fn claim_and_give_back_string_ownership(received_string: String) -> String {
    let returned_string = received_string;
    returned_string
}

fn demonstrate_function_ownership_transfer() {
    let string_to_give: String = String::from("I am being passed around.");
    let string_received = claim_and_give_back_string_ownership(string_to_give);

    println!("\nString received: {string_received}");
}

fn receive_string_reference_return_size(string_inputted: &String) -> usize {
    string_inputted.len()
}

fn demonstrate_rust_references() {
    let string_to_be_referenced: String = String::from("Size me up bro");
    let size_of_string: usize = receive_string_reference_return_size(&string_to_be_referenced);
    println!("\nString referenced: {string_to_be_referenced}");
    println!("Size of referenced string: {size_of_string}");
}

fn receive_string_reference_and_mutate_it(string_inputted: &mut String) {
    string_inputted.push_str("completed");
}

fn demonstrate_mutable_rust_references() {
    let mut string_to_be_mutated: String = String::from("I am ");

    receive_string_reference_and_mutate_it(&mut string_to_be_mutated);
    println!("\nJust mutated a string: {string_to_be_mutated}");
}

fn demonstrate_data_race_prevention() {
    let mut mutable_string: String = String::from("I am mutable ");
    {
        let first_copy: &mut String = &mut mutable_string;
        first_copy.push_str("here.");
        println!("\n{first_copy}");
    }
    let second_copy: &mut String = &mut mutable_string;
    second_copy.push_str(" And here as well");
    println!("{second_copy}");
}

fn demonstrate_multiple_immutable_reference_use() {
    let immutable_string: String = String::from("I am immutable, but borrowable");
    let first_reference: &String = &immutable_string;
    let second_reference: &String = &immutable_string;

    println!("\nFirst immutable reference: {first_reference}");
    println!("Second immutable reference: {second_reference}");
}

fn demonstrate_string_slices() {
    let my_opinion: String = String::from("Your tune is jarring");
    let riddle_answer_start_index: usize;
    match my_opinion.find("jar") {
        Some(index) => {
            riddle_answer_start_index = index;
        }
        None => {
            println!("\nError: no riddle answer found in text.");
            return;
        }
    }
    let riddle_answer_end_index: usize = riddle_answer_start_index + 3;
    let riddle_answer: &str = &my_opinion[riddle_answer_start_index..riddle_answer_end_index];
    println!("\nWhen is a door not a door?\n\tWhen it is a{riddle_answer}");
}

fn slice_a_string(string_to_slice: &str, slice_indexes: [usize; 2]) -> &str {
    // using &str in the parameter list matches &String and &str
    &string_to_slice[slice_indexes[0]..slice_indexes[1]]
}

fn demonstrate_data_consistencies() {
    let string_to_slice: String = String::from("I'm about to be sliced D:");
    let slice_start_index: usize;
    let slice_end_index: usize;
    match string_to_slice.find("sliced") {
        Some(index) => {
            slice_start_index = index;
            slice_end_index = index + "sliced".len();
        }
        None => {
            println!("Error: someting is off with `string_to_slice`");
            return;
        }
    }

    let sliced_word: &str = slice_a_string(&string_to_slice, [slice_start_index, slice_end_index]);
    println!("\n{string_to_slice}\n\tI just got {sliced_word}...");
    // at this point on, `string_to_slice` can not be mutated
    // my String methods
    // e.g. `string_to_slice.push_str("this will error out")` will panic
}

fn main() {
    demonstrate_heap_stack_difference();
    demonstrate_heap_cloning();
    demonstrate_function_ownership_transfer();
    demonstrate_rust_references();
    demonstrate_mutable_rust_references();
    demonstrate_data_race_prevention();
    demonstrate_multiple_immutable_reference_use();
    demonstrate_string_slices();
    demonstrate_data_consistencies();
}
