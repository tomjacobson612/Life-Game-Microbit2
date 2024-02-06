# Life

## Tom Jacobson

### Writeup

For this assignment I used the poll-button example as a template to begin from. I took the randomize_fb() function from that repository and reused it here as it achieved the job of randomizing a given board for me. I was able to write a complement_fb function using similar logic to the randomize_fb() function, and used that complement function to achieve the desired effect when the b button was pressed.  

The rest of the code was fairly trivial to implement as it just involved setting up the correct logic and using the functions given to me via the life module or the randomize_fb function. I ran into some trouble implementing the button ignores and having the board wait 5 frames to randomize. However, a helpful hint from Bart made me realize that the delays were in increments of 100ms, which is very nice since our game waits 100ms after every loop iteration. Using that hint I implemented two counters to keep track of the last time any button was pressed and the last time the b button was pressed. Another direction I was considering going was reading the current time of the timer and using that to calculate whether or not 500ms had passed since a button press. 

Overall I think the assignment went very well. It was a bit daunting at first but a lot of the necessary pieces were provided to us either by Bart or by the Board itself, it was a more a matter of reading through the docs and finding the correct funcitonality. 

#### Sources
https://github.com/pdx-cs-rust-embedded/poll-button/blob/main/src/main.rs
Game of life code given as part of the assignment. 
Nanorand code given as part of the assignment. 