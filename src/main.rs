use {
	colored::Colorize,
	guessing_game::{
		input,
		respond,
		Guess,
	},
	rand::{
		thread_rng,
		Rng,
	},
	std::{
		io::{
			stdin,
			stdout,
		},
		ops::RangeInclusive,
	},
};

// Range of valid guesses and the correct answer.
const GUESS_RANGE: RangeInclusive<i32> = 0_i32..=1024_i32;

// How many times does the user get to guess?
const ATTEMPTS_ALLOWED: i32 = 10_i32;

fn main()
{
	// Greeting/header.
	println!(
		"{}",
		format!("\n\nI'm thinking of a number somwhere from {} through {}. Guess it! 😈", GUESS_RANGE.start(), GUESS_RANGE.end()).green()
	);

	// Generate random Guess.
	let correct = Guess::new(thread_rng().gen_range(GUESS_RANGE)).expect("Error generating random correct value.");

	// For each attempt.
	for i in (1..=ATTEMPTS_ALLOWED).rev()
	{
		// Respond to prompted input
		if respond(
			input::<GUESS_RANGE>(format!("You have {i} attempts remaining. Guess: ").yellow(), &mut stdin(), &mut stdout()),
			correct,
			&mut stdout(),
		)
		.is_break()
		{
			// Win condition: Correct guess should be end of program.
			return
		}
	}

	// Lose Condition: No attempts remaining.
	println!("{}", "\nYou're out of guesses! Game over. 😢\n\n".red());
}
