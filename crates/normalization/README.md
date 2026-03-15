# DeepCorr Normalization

A rust based normalization crate, part of the DeepCorr Deep CCA system currently in development by Harry Woodhouse at the University of York. 

## Current Normalization Functions:
- Cosine
- Z-score
- MinMax

## Formulas:

### Cosine Normalization
Pass in a 2d matrix which replicates the structure of your data set (i.e. point per row) and it will normalize it. Cosine normalization normalizes data to a unit of 1 by divind values by the euclidean value. I.e.

$$
(X, Y, Z) = (8.0, 32.0, 9.0)
$$
$$
r = \sqrt{X^2 + Y^2 + Z^2}
$$
$$
r = 34.2
$$

$$
(X',Y',Z') = (\frac{X}{r}, \frac{Y}{r}, \frac{Z}{r})
$$
$$
(0.23,0.93, 0.26) = (\frac{8.0}{34.2}, \frac{32.0}{34.2}, \frac{9.0}{34.2})
$$

We can veridy these calculations by ensuring the square root of the sum of the square = 1

$$
\sqrt{0.23^2 + 0.93^2 + 0.26^2} = 0.99 
$$
Off slightly due to rounding but the math checks out.

### Z-Score Normalization
Subtrac the mean from a data point divided by the standard deviation of that row of the matrix

$$
z = \frac{x - \mu}{\sigma + \epsilon}
$$

### Min-Max
$$
x' = \frac{x - \text{min}(x)}{(\text{max}(x) - \text{min}(x)) + \epsilon}
$$

