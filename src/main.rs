mod data;
mod storage;

use data::Habit;

fn main() {
    let mut app_data = storage::load_data().expect("Failed to load data");

    println!("heat");
    println!("Data path: {:?}", storage::get_data_path());
    println!("Loaded {} habits", app_data.habits.len());

    if app_data.habits.is_empty() {
        let test_habit = Habit::new("Test Habit".to_string());
        println!("Creating test habit: {}", test_habit.name);
        app_data.habits.push(test_habit);
        storage::save_data(&app_data).expect("Failed to save data");
        println!("Saved data successfully");
    } else {
        println!("Existing habits:");
        for habit in &app_data.habits {
            println!("  - {} (created: {})", habit.name, habit.created_at);
        }
    }
}
