#![feature(try_trait_v2)]
#![feature(let_chains)]
#![feature(adt_const_params)]
#![allow(incomplete_features)]

use {
	colored::{
		ColoredString,
		Colorize,
	},
	std::{
		cmp::Ordering,
		fmt::{
			Display,
			Formatter,
		},
		io::{
			Read,
			Write,
		},
		ops::{
			ControlFlow,
			RangeInclusive,
		},
		result::Result,
	},
};

/// Gets user input until it is valid and returns it as a Guess. Accepts a
/// colored string to prompt the user for input.
///
/// This will block the program while waiting for input from stdin. It will ask
/// for input until the input is an integer withing the range of valid values.
/// If the input is invalid, it will display an error before repeating from the
/// prompt.
///
/// # Panics
/// Panics if writing to [io::stdout] fails or if a formatting trait
/// implementation returns an error. This indicates an incorrect implementation
/// since fmt::Write for String never returns an error itself.
///
/// # Examples
///
/// Prompting the user for a guess and storing it.
///
/// ```
/// // use {
/// // 	colored::Colorize,
/// // 	guessing_game::input,
/// // 	std::io::{
/// // 		stdin,
/// // 		stdout,
/// // 	},
/// //};
/// // let input = input::<{ 0..=100000 }>(format!("Guess a number: ").yellow(), &mut stdin(), &mut stdout());
/// ```
pub fn input<const RANGE: RangeInclusive<i32>>(
	prompt: ColoredString,
	input: &mut impl Read,
	output: &mut impl Write,
) -> Guess<RANGE>
{
	// Avoids counting invalid guesses as used attempts.
	loop
	{
		print!("{}", prompt);
		let mut guess_input = String::new();
		// If no issue prompting.
		if let Ok(_) = output.flush()
		// Read input.
			&& let Ok(_) = input.read_to_string(&mut guess_input)
		// Trim and parse to integer.
			&& let Ok(parsed) = guess_input.trim().parse::<i32>()
		// Validate input.
			&& let Ok(guess) = Guess::new(parsed)
		{
			// Stop looping if everything checks out.
			break guess;
		}
		else
		{
			// Show helpful error when user input is invalid.
			output.write_all(format!("\n{}\n{}",
				"Invalid guess. 🤕".red(),
				format!("Guesses must be an integer from {} through {}.", RANGE.start(), RANGE.end()).yellow()).as_bytes())
			.expect("Error erroring...");
		}
	}
}

/// Respond to a user's input with some console output. Returns the correct
/// action to take.
///
/// The output will explain whether the guess was too high, too low, or if they
/// are equal, that the user wins. The return value will be
/// std::ops::ControlFlow::Continue(()) unless the user wins, in which case it
/// will return std::ops::ControlFlow::Break(()).
///
/// # Panics
/// Panics if writing to [io::stdout] fails.
///
/// # Examples
///
/// Demonstrating a guess that is too high:
///
/// ```
/// // use std::{
/// // 	io::stdout,
/// // 	ops::ControlFlow,
/// //};
/// // let example_guess = Guess::<{ 0..=100000 }>::new(42069_i32).expect("");
/// // let correct_guess = Guess::<{ 0..=100000 }>::new(1660_i32).expect("");
/// // let action: ControlFlow<()> = respond(example_guess, correct_guess, &mut stdout());
/// // assert!(action.is_continue()));
/// ```
pub fn respond<const RANGE: RangeInclusive<i32>>(
	guess: Guess<RANGE>,
	correct: Guess<RANGE>,
	output: &mut impl Write,
) -> ControlFlow<()>
{
	output.write_all(
		match guess.cmp(&correct)
		{
			Ordering::Greater => "\n{guess} is too high! 🥵".magenta(),
			Ordering::Less => "\n{guess} is too low! 🥶".cyan(),
			Ordering::Equal => "\nYou win! 😊🏖".green().bold(),
		}
		.as_bytes(),
	)
	.expect("Error outputting response.");
	if let Ordering::Equal = correct.cmp(&guess)
	{
		ControlFlow::Break(())
	}
	else
	{
		ControlFlow::Continue(())
	}
}

/// Tuple struct to represent a guess. A guess is a type-safe way to represent
/// the integer that was input by the user, and the correct value to compare
/// against.
///
/// Guess's constructor offers input validation in the constructor itself rather
/// than relying on other methods to obey the assumption that the guess is in a
/// valid state. This struct must be created using Guess::new().
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub struct Guess<const RANGE: RangeInclusive<i32>>
{
	value: i32,
}

/// Constructor for creating a Guess.
///
/// The constructor validates that the guess' value lies within RANGE. If the
/// value provided is valid, it returns an Ok() containing a new instance of
/// Guess. If the value provided is invalid, the constructor returns an error.
///
///
/// # Examples
///
/// Demonstrating a guess that is too high:
///
/// ```
/// // use guessing_game::{
/// // 	input,
/// // 	Guess,
/// //};
/// // let example_guess = Guess::<{ 0..=100000 }>::new(42069_i32);
/// ```
impl<const RANGE: RangeInclusive<i32>> Guess<RANGE>
{
	pub fn new(guess: i32) -> Result<Self, i32>
	{
		if RANGE.contains(&guess)
		{
			Ok(Guess { value: guess })
		}
		else
		{
			Err(guess)
		}
	}
}

/// Formats Guess for displaying in console output.
///
/// # Examples
///
/// Outputting a Guess to the console:
///
/// ```
/// // use guessing_game::Guess;
/// // println!("The guess displays as {}", Guess::<{ 0..=100000 }>::new(100_i32).expect(""));
/// ```
impl<const RANGE: RangeInclusive<i32>> Display for Guess<RANGE>
{
	fn fmt(
		&self,
		f: &mut Formatter<'_>,
	) -> Result<(), std::fmt::Error>
	{
		self.value.fmt(f)
	}
}

#[cfg(test)]
mod tests
{
	use {
		super::{
			Guess,
			*,
		},
		std::io::stdout,
	};
	#[test]
	fn construction()
	{
		Guess::<{ 0..=0 }>::new(0_i32).expect("guess 1 failed to construct.");
		Guess::<{ 0..=1 }>::new(0_i32).expect("guess 2 failed to construct.");
		Guess::<{ 0..=0 }>::new(1_i32).expect_err("guess 3 failed to fail to construct.");

		Guess::<{ 0..=10 }>::new(0_i32).expect("guess 4 failed to construct.");
		Guess::<{ 0..=10 }>::new(5_i32).expect("guess 5 failed to construct.");
		Guess::<{ 0..=10 }>::new(10_i32).expect("guess 6 failed to construct.");

		Guess::<{ 10..=20 }>::new(0_i32).expect_err("guess 7 failed to fail to construct.");
		Guess::<{ 10..=20 }>::new(10_i32).expect("guess 8 failed to construct.");
		Guess::<{ 10..=20 }>::new(15_i32).expect("guess 9 failed to construct.");
		Guess::<{ 10..=20 }>::new(20_i32).expect("guess 10 failed to construct.");

		Guess::<{ 0..=10000 }>::new(-0_i32).expect("guess 11 failed to construct.");
		Guess::<{ 0..=10000 }>::new(-5_i32).expect_err("guess 12 failed to construct.");
		Guess::<{ 0..=10000 }>::new(-10_i32).expect_err("guess 13 failed to construct.");
	}

	#[test]
	fn equality()
	{
		assert_eq!(
			Guess::<{ 0..=10 }>::new(0_i32).expect("guess 1 failed to construct."),
			Guess::<{ 0..=10 }>::new(0_i32).expect("guess 2 failed to construct.")
		);
		assert_eq!(
			Guess::<{ 0..=20 }>::new(10_i32).expect("guess 3 failed to construct."),
			Guess::<{ 0..=20 }>::new(10_i32).expect("guess 4 failed to construct.")
		);
		assert_eq!(
			Guess::<{ 0..=50 }>::new(50_i32).expect("guess 5 failed to construct."),
			Guess::<{ 0..=50 }>::new(50_i32).expect("guess 6 failed to construct.")
		);
	}

	#[test]
	fn nequality()
	{
		assert_ne!(
			Guess::<{ 0..=10 }>::new(5_i32).expect("guess 1 failed to construct."),
			Guess::<{ 0..=10 }>::new(0_i32).expect("guess 2 failed to construct.")
		);
		assert_ne!(
			Guess::<{ 0..=20 }>::new(10_i32).expect("guess 3 failed to construct."),
			Guess::<{ 0..=20 }>::new(15_i32).expect("guess 4 failed to construct.")
		);
		assert_ne!(
			Guess::<{ 0..=50 }>::new(50_i32).expect("guess 5 failed to construct."),
			Guess::<{ 0..=50 }>::new(0_i32).expect("guess 6 failed to construct.")
		);
	}

	#[test]
	fn respond_test()
	{
		let guess = Guess::<{ 0..=50 }>::new(40).expect("guess 1 failed to construct.");
		let correct = Guess::<{ 0..=50 }>::new(40).expect("guess 2 failed to construct.");
		assert_eq!(respond(guess, correct, &mut stdout()), ControlFlow::Break(()));

		let guess = Guess::<{ 0..=50 }>::new(20).expect("guess 3 failed to construct.");
		let correct = Guess::<{ 0..=50 }>::new(40).expect("guess 4 failed to construct.");
		assert_eq!(respond(guess, correct, &mut stdout()), ControlFlow::Continue(()));

		let guess = Guess::<{ 0..=50 }>::new(40).expect("guess 5 failed to construct.");
		let correct = Guess::<{ 0..=50 }>::new(20).expect("guess 6 failed to construct.");
		assert_eq!(respond(guess, correct, &mut stdout()), ControlFlow::Continue(()));
	}

	#[test]
	fn input_test()
	{
		let correct = Guess::<{ 0..=50 }>::new(50).expect("correct failed to construct");
		let input1 = "50";
		let guess1: Guess<{ 0..=50 }> = input("dummy prompt: ".clear(), &mut input1.as_bytes(), &mut stdout());
		assert_eq!(guess1, correct);

		let input2 = "40";
		let guess2: Guess<{ 0..=50 }> = input("dummy prompt: ".clear(), &mut input2.as_bytes(), &mut stdout());
		assert_ne!(guess2, correct);
	}
}
