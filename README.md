# Computor_v2

```
 ./computorv2 
> func(x) = x^2 + 3x + 3 
  x ^ 2 + 3 * x + 3
> y = 1 
  1
> func(x) = y ?
  2 + 3x^1 + x^2 = 0
Two solutions on R:
-1
-2
```

## Overview

I created an interpreter that can perform various calculations in 42 assignments.

## Requirement

- cargo 1.66.0

## Usage

```
git clone .....
cd Computor_v2
make
./computorv2
```

## Features

- Calculation

```
> 1 + 2 * ( 3 - 1 )^2 = ?
  9
```

- Registering Variables

```
> x = 2
  2
> x + x^2 - 1 = ?
  5
```

- Registering Functions

```
> f(x) = x^2 + 2x + 1
  x ^ 2 + 2 * x + 1
> f(3) = ?
  16
```

- Find solutions to equations of the second degree or less

```
> x^2 + 2x + 1 = 0 ?
  1 + 2x^1 + x^2 = 0
Only one solution on R:
-1
```

- Complex number

```
> x = 1 + i
  1 + i
> x * 3i = ?
  -3 + 3i
```

- Matrix

```
> x = [[3,3,3];[3,3,3]]
  [ 3 , 3 , 3 ]
  [ 3 , 3 , 3 ]
> y = [[1,2];[2,2];[2,2]]
  [ 1 , 2 ]
  [ 2 , 2 ]
  [ 2 , 2 ]
> x ** y = ?
  [ 15 , 18 ]
  [ 15 , 18 ]
```

- Listing of variables

```
> x = 1
  1
> y = 2
  2
> variables
x: 1
y: 2
```

- Listing of functions

```
> f(x) = x + 1
  x + 1
> g(x) = x - 1
  x - 1
> functions
f(x): x + 1
g(x): x - 1
```

- View command history

```
> x = 1
  1
> y = 1
  1
> x + y =?
  2
> history
  x = 1: 1
  y = 1: 1
  x + y =?: 2
```

- Special Functions
    - exp
    - sqrt
    - abs
    - sin
    - cos
    - tan

```
> sqrt(2) = ?
  1.4142135623730951
```

- Special Variables
    - pi
    - i

```
> cos(2pi) = ?
  1
```

- operator
    - +
    - -
    - *
    - /
    - %
    - ** (matrix product)
    - ^

- variables
    - Special characters such as i cannot be registered
    - Alphabet only
    - uppercase letters are recognized as lowercase

## Author

[twitter](https://twitter.com/Kotabrog)

## Licence

[MIT](https://github.com/kotabrog/Computor_v2/blob/main/LICENSE)
