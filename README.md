# Lunar Lockout
A solver for [lunar lockout][lunar].

## Strategy
We will determine the game tree and seek a solution.

### Number of Positions
Below is a tabulation of the number of possible positions, without taking symmetry into account. The numbers are calculated with the [stars-and-bars][start-bars] method.

| #Robots | #tableux       | #Positions        |
|---------|----------------|-------------------|
| 0       | 1              | 1                 | 
| 1       | 625            | 625               | 
| 2       | 195000         | 390000            | 
| 3       | 40495000       | 242970000         | 
| 4       | 6296972500     | 151127340000      | 
| 5       | 782083984500   | 93850078140000    |  
| 6       | 80815345065000 | 58187048446800000 | 

Although these numbers seem large, if one reasons about the connectedness of these positions, one can come to the conclusion that there are a lot of components.

[lunar]: http://www.thinkfun.com/products/lunar-landing/
[stars-bars]: https://en.wikipedia.org/wiki/Stars_and_bars_(combinatorics)
