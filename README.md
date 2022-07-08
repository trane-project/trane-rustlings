# trane-rustlings

This course is meant to showcase how Trane can be used to augment existing educational material, in
this case, [rustlings](https://github.com/rust-lang/rustlings/). The exercises here just reference
the exercises in rustlings by name. Using both side by side, you can reinforce your practice by
automatically moving on to harder topics once the easier topics are mastered and periodically
reinforcing already mastered exercises.

The biggest difference with respects to rustlings is in the order in which the exercises are
presented. In particular, the original order in rustlings is entirely linear. This course takes
advantage of Trane's ability to define arbitrary dependency relationships to arrange the exercises
in a more parallel order. The two intro exercises in rustlings are not included here since they just
introduce how rustlings work.

# Instructions

Follow the instructions in the rustlings repo to install it. Then, install
[trane-cli](https://github.com/trane-project/trane-cli) somewhere in your system's path by either
building it from source or downloading one of the prebuilt binaries.

Then, open two terminals. In the first one go to the root of this repo and type `trane`. You will be
met with Trane's command line. Run the command `open ./` to load the course. You can enter the
command `next` to get the next question.

```
trane >> next
Course ID: trane::programming::rust::rustlings
Lesson ID: trane::programming::rust::rustlings::lesson_1
Exercise ID: trane::programming::rust::rustlings::lesson_1::variables1

Solve Rustlings exercise variables1.
```

Trane is telling you to work on exercise `variables1`. In the other terminal, type `rustlings run
variables1`. This should show you there was an error compiling the file as expected. Open the
exercise in your preferred editor and fix the compilation issue. Once you feel you are done with
this exercise, assign yourself a score from one to five (five being the highest). Let's say your
score is five. You can tell Trane to record it by running the command `score 5`. Then, run `next`
again to move on to the next exercise.

# Tips

- Undo your work after you are finished with each exercise, so you do not see the answer the next
  time the exercise is selected.
- If you feel like the current exercise is too easy, and you would like to not see it again, you can
  tell Trane so by running the command `blacklist exercise`. You can also run `blacklist lesson` to
  add the current exercise's lesson to the blacklist.
