fn pomodoro_timer(work_duration: u32, break_duration: u32, rounds: u32) {
    for round in 1..=rounds {
        println!("🍅 Round {round}: Focus for {work_duration} minutes!");
        countdown(work_duration * 60);

        println!("🛋️ Time for a {break_duration}-minute break!");
        countdown(break_duration * 60);

        if round < rounds {
            println!("\x07🔔 Break over! Get ready for the next round.");
        }
    }

    println!("\x07✅ Pomodoro complete! Great work!");
}

fn countdown(seconds: u32) {
    for i in (1..=seconds).rev() {
        println!("⏳ {} seconds remaining", i);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
fn main() {
    let work_duration = 25; // 25 minutes
    let break_duration = 5; // 5 minutes
    let rounds = 4; // 4 rounds

    println!("🍅 Starting Pomodoro Timer!");
    pomodoro_timer(work_duration, break_duration, rounds);
}
