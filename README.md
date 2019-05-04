The guessing game we built as an example Rust program
while reading [The Book Chapter 2](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) reminded me of
the version of [Mastermind](https://en.wikipedia.org/wiki/Mastermind_(board_game)) I played with my dad growing up. He'd
ask us to think of a 6 digit number and he'd then guess, with
the only hints being the responses "x are correct and in the correct" and "x more are correct, but not in the correct place."
He did this in his head and always got the number pretty quick.

I wanted to program 2 versions of the game to explore concepts
as I learned Rust.

 - The version I played with my dad - basically complete in the initial commit
 - One more similar to the board game that actually displayed
 previous guesses for us mere mortals that can't do this in
 our head.

Before I tackle the second version, I'm going to clean up the code, with some functions and various other concepts I pick up as I continue learning Rust. 