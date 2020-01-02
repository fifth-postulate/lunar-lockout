# Lunar Lockout
A solver for [lunar lockout][lunar].

## Strategy
We will determine the game tree and seek a solution.

### Number of Positions
Below is a tabulation of the number of possible positions, without taking symmetry into account, but differentiating different robots.

| #Robots | #tableux | #Positions |
|---------|----------|------------|
| 0       | 1        | 1          | 
| 1       | 25       | 25         | 
| 2       | 300      | 600        | 
| 3       | 2300     | 13800      | 
| 4       | 12650    | 303600     | 
| 5       | 53130    | 6375600    |  
| 6       | 177100   | 127512000  | 

Although these numbers seem large, if one reasons about the connectedness of these positions, one can come to the conclusion that there are a lot of components.

[lunar]: http://www.thinkfun.com/products/lunar-landing/
[stars-bars]: https://en.wikipedia.org/wiki/Stars_and_bars_(combinatorics)
