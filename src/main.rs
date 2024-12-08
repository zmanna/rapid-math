use eframe::egui;
use rand::Rng;
use std::time::{Duration, Instant};

struct MathQuizApp {
    question: String,
    answer: i32,
    user_input: String,
    score: i32,
    correct_answers: i32,
    wrong_answers: i32,
    remaining_time: Duration,
    start_time: Option<Instant>,
    feedback: String,
    game_over: bool,
    is_pemdas: bool,
}

impl Default for MathQuizApp {
    fn default() -> Self {
        let (question, answer, is_pemdas) = generate_problem(0);
        Self {
            question,
            answer,
            user_input: String::new(),
            score: 0,
            correct_answers: 0,
            wrong_answers: 0,
            remaining_time: Duration::new(30, 0),
            start_time: None,
            feedback: String::from("Press Start to begin!"),
            game_over: false,
            is_pemdas,
        }
    }
}

impl eframe::App for MathQuizApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed();
            if elapsed >= self.remaining_time {
                self.game_over = true;
                self.start_time = None; // Stop the timer
            } else {
                self.remaining_time -= elapsed;
                self.start_time = Some(Instant::now());
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.game_over {
                self.display_game_over(ui);
            } else {
                self.display_game(ui, ctx);
            }
        });

        ctx.request_repaint(); // Ensure continuous UI updates
    }
}

impl MathQuizApp {
    fn display_game(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.vertical_centered(|ui| {
            ui.heading("Math Quiz");
            ui.add_space(20.0);

            // Timer and Score
            ui.label(format!("Time Remaining: {} seconds", self.remaining_time.as_secs()));
            ui.label(format!("Score: {}", self.score));

            // Question and Input
            ui.add_space(30.0);
            ui.heading(&self.question);
            ui.add_space(10.0);

            let input_response = ui.add(
                egui::TextEdit::singleline(&mut self.user_input)
                    .hint_text("Enter your answer")
                    .font(egui::FontId::proportional(40.0))
                    .frame(true),
            );

            // Automatically focus on the input box
            if self.start_time.is_some() && !input_response.has_focus() {
                ui.memory_mut(|mem| mem.request_focus(input_response.id));
            }

            // Detect Enter Key Submission
            if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.process_input();
            }

            ui.add_space(20.0);
            ui.label(&self.feedback);

            // Start Button
            if self.start_time.is_none() && !self.game_over {
                if ui.button("Start").clicked() {
                    self.start_time = Some(Instant::now());
                    self.feedback = "Solve the problems!".to_string();
                }
            }
        });
    }

    fn display_game_over(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Game Over");
            ui.add_space(20.0);
            ui.label(format!("Final Score: {}", self.score));
            ui.label(format!("Correct Answers: {}", self.correct_answers));
            ui.label(format!("Wrong Answers: {}", self.wrong_answers));

            ui.add_space(20.0);
            if ui.button("Restart").clicked() {
                *self = MathQuizApp::default(); // Reset the game state
            }
        });
    }

    fn process_input(&mut self) {
        if let Ok(user_answer) = self.user_input.trim().parse::<i32>() {
            if user_answer == self.answer {
                self.correct_answers += 1;

                // Adjust score and timer based on question type
                if self.is_pemdas {
                    self.score += 2; // PEMDAS questions count as 2 points
                } else {
                    self.score += 1;
                }

                self.remaining_time += Duration::new(1, 0); // Add 1 second for correct answer
                self.feedback = "Correct!".to_string();
            } else {
                self.wrong_answers += 1;
                self.remaining_time = self
                    .remaining_time
                    .checked_sub(Duration::new(2, 0))
                    .unwrap_or(Duration::new(0, 0)); // Subtract 2 seconds for wrong answer
                self.feedback = format!("Wrong! The correct answer was {}.", self.answer);
            }

            // Generate new question
            let (new_question, new_answer, is_pemdas) = generate_problem(self.score);
            self.question = new_question;
            self.answer = new_answer;
            self.is_pemdas = is_pemdas;
        } else {
            self.wrong_answers += 1;
            self.remaining_time = self
                .remaining_time
                .checked_sub(Duration::new(2, 0))
                .unwrap_or(Duration::new(0, 0)); // Subtract 2 seconds for invalid input
            self.feedback = "Invalid input. Try again!".to_string();
        }

        // Clear user input
        self.user_input.clear();
    }
}

fn generate_problem(score: i32) -> (String, i32, bool) {
    let mut rng = rand::thread_rng();

    // Adjust difficulty based on score
    let (min, max, include_complex_ops) = if score < 5 {
        (1, 10, false) // Easy: Numbers 1–10, no PEMDAS
    } else if score < 10 {
        (1, 20, true) // Medium: Numbers 1–20, occasional PEMDAS
    } else {
        (1, 50, true) // Hard: Numbers 1–50, frequent PEMDAS
    };

    let num1 = rng.gen_range(min..=max);
    let num2 = rng.gen_range(min..=max);
    let num3 = rng.gen_range(min..=max);

    if include_complex_ops && rng.gen_bool(0.3) {
        // 30% chance to generate PEMDAS question
        let operator = rng.gen_range(0..2); // 0: *, 1: /
        match operator {
            0 => (
                format!("{} * ({} + {})", num1, num2, num3),
                num1 * (num2 + num3),
                true, // PEMDAS question
            ),
            1 => (
                format!("({} - {}) / {}", num1 + num3, num2, num3),
                if num3 != 0 {
                    (num1 + num3 - num2) / num3
                } else {
                    0
                },
                true,
            ),
            _ => unreachable!(),
        }
    } else {
        // Simple operations
        let operator = rng.gen_range(0..4); // 0: +, 1: -, 2: *, 3: /
        match operator {
            0 => (format!("{} + {}", num1, num2), num1 + num2, false),
            1 => (format!("{} - {}", num1, num2), num1 - num2, false),
            2 => (format!("{} * {}", num1, num2), num1 * num2, false),
            3 => {
                if num2 != 0 {
                    (format!("{} / {}", num1 * num2, num2), num1, false)
                } else {
                    (format!("{} + {}", num1, 1), num1 + 1, false)
                }
            }
            _ => unreachable!(),
        }
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Math Quiz",
        options,
        Box::new(|_cc| Box::new(MathQuizApp::default())),
    )
}
