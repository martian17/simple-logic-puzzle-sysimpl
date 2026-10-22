# Liars and Truth-Tellers Problem
This set of problems were originally submitted to [Cornell Math Explorer's club in 2009](https://pi.math.cornell.edu/~mec/Winter2009/Lipa/Puzzles/lesson3.html), and it borrows elements from the classic [knights and knaves problem (Wikipedia)](https://en.wikipedia.org/wiki/Knights_and_Knaves) and [the hardest logic puzzle ever (Wikipedia)](https://en.wikipedia.org/wiki/The_Hardest_Logic_Puzzle_Ever), which was published in the Harvard Review of Philosophy in 1996.

## Problem setting
Liars and Truth-Tellers is a class of riddles where you have to figure out some piece or pieces of information by asking yes-or-no questions to fictional characters who will either always tell the truth (Truth-Tellers) or always lie (Liars). Occasionally, a third type is brought in that answers completely randomly (Randoms).

The riddles usually require one to figure out the information using the smallest possible number of questions, and the number is usually quite small.

### Easy Problem
You are standing at a fork in the road, and you would like to know which way is the next town. Before you are a Liar and a Truth-Teller, but you don't know which is which. Figure out one yes-no question that you can ask one of the people before you that will enable you to figure out the correct road.

### Hard Problem
Same situation as the first easy problem, but instead of two people, there are three people: a Truth-Teller, a Liar, and a Random. Use two questions to figure out the correct road.

You may ask the two questions to different people or to the same person. You can use the answer to the first question to determine who you ask the second question to and what you ask him.

### Very Hard Problem
Same setup as the hard problem, but now, even though the three people before you know and understand English, they answer your questions in their native tongue using the words "Ja" and "Da". These words translate as "yes" and "no", but you don't know which word means what. It is rather surprising, but you can still find the correct road using only two questions.

### Hints
For the first problem, think about how you can get the guy you ask the question to to talk about what the other person would say. How does this help you?
For the second two problems, does asking a question to the Random tell you any information at all? How can you use your first question to ensure that you don't ask your second question to the Random?
How can you combine multiple questions into one to cover contingencies?

## Structure of the Repository
`src/types.rs` contains the programmatic definition of this problem, with relies on Rust's core features like traits, generics, enums, and exhaustive matching. `src/main.rs` tries to combine primitives defined in `src/enum`, to represent questions that will be asked to liars and truth tellers. For example,

contains the programmati

## Original Inspiration
What motivated me to make this exercise was this video [https://www.youtube.com/watch?v=3sutYh31UD8](https://www.youtube.com/watch?v=3sutYh31UD8) (Caution: You may find the figures in this video to be slightly inappropriate. Also spoilers around 1:38).  
The original archived rust source file contains a one-file solution to this variation of the liars-and-truth-tellers problem.
