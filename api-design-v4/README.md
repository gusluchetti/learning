2024-04-20 - tech lead at my current job suggested that, given our current (incoming) tech stack, we brush up on:
- express
- prisma
- node in general

so i figured frontend masters course ['Api Design in Node.js, v4'](https://frontendmasters.com/courses/api-design-nodejs-v4) would be
a great way to accomplish that.

having now finished the course, i'd like to note a few things.

- it's interesting to see such 'partitioned' code after seeing a lot of 'you should see
  everything that your function does just by glancing at it' sentiment online. in this case, it does
  make complete sense for it to be all separate, since a handler might need to do
  lots of other things before actually completing a request (in this case, some middleware, auth,
  input validation, error input validation, and then we reached the db with our
  query).

- i thought prisma would overcomplicate things but honestly it makes everything so
  much easier. not a big fan of its syntax but just by it throwing errors if you're
  missing something makes quick development a whole lot easier. that's without
  taking into consideration how easy is it to make more migrations whenever
  necessary. i'd still like to try going 'raw', but prisma proved me wrong. 

- the teacher didn't go super into it, but the 2 episodes on dynamic environment
  config were a nice touch! usually people ignore that sort of deployment config
  step and it was nice to see how you can preemptively make your program more
  resilient/pre-configured depending on what environment it's on.

other than that, great course! very proud that i was able to get it done
reasonably quick (had done the two initial chapters earlier, but i really started
watching it thursday, 2024-04-18), and that my new nvim-tmux setup made things a lot
easier with multiple windows/panes, and quick switching between them to see server
status/tests and all that. still haven't figured out how to see my actual DB on the
terminal, but that's a challenge for later.
