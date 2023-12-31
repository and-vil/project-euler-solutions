# project-euler-solutions
Public-domain solutions to selected Project Euler problems. For a complete list of these problems, visit [https://projecteuler.net/archives](https://projecteuler.net/archives).

The primary goal of this repo is to show how each problem can be solved with self-contained per-problem code and attempts to avoid exposing the similarity of like problems (Project Euler has an option to hide problem tags to encourage people to find solution methods/classes on their own). I would highly recommend that each person develop their own solutions first, then lean on these only as a point of reference that may spur additional thought or help if one gets stuck and needs a fresh perspective on a given problem. As best stated on the main page for Project Euler, "There is nothing quite like that 'Aha!' moment when you finally beat a problem which you have been working on for some time."

On top of that, this is a great exercise in solving these types of problems, and a great refresher for the languages used (i.e. Rust and C++ for now, more may come later). In some cases, I made some small and incremental optimizations, such as using matrix exponentiation to more efficiently compute Fibonacci sequence terms, for example.

As a side note, writing solutions in different languages and benchmarking them may be another good idea for a secondary repo, and may highlight some good problem-solving methods. I haven't dug into that yet, but would like to at some point.

Project Euler completion status:

![Project Euler Completion Status](https://projecteuler.net/profile/avillalobos.png?timestamp=2)

<!--Note: to refresh this image, the following needs run in Git Bash:
curl -X PURGE https://camo.githubusercontent.com/....
* see https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/about-anonymized-urls#removing-an-image-from-camos-cache*
--adding `?timestamp=` query param with manually updated timestamp val to trick Camo into refreshing, may be able to use an API to trigger refreshes :shrug:
-->
