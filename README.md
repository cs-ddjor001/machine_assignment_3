# The Program #
The Program computes a approximate value for the derivative of f(x) = sin(x),
using the finite difference formula.

# Input #
There is no input for the program. All the values are hardcoded.
- f(x) = sin(x)
- x = 1
- known f'(x) = cos(x)

# Output #
The output takes the form of the following table
```
|  h   |       x       | Approx. f'(x) |  Known f'(x)  |  Abs. Error   |
|:----:|--------------:|--------------:|--------------:|--------------:|
|2^-01 |    1.00000000 |    0.31204800 |    0.54030231 |    0.22825430 |
|2^-02 |    1.00000000 |    0.43005454 |    0.54030231 |    0.11024777 |
|2^-03 |    1.00000000 |    0.48637287 |    0.54030231 |    0.05392943 |
|2^-04 |    1.00000000 |    0.51366321 |    0.54030231 |    0.02663910 |
```
The program also generates a graph of h vs Abs. Error.

# How to run the Program #
Use cargo build to compile the program. Use cargo run to run it.