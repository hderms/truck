# Truck
simple/toy Make-like command runner configured using TOML. 

## Example
``` toml
[[tasks]]
id = 1
command = "echo"
args = ["foo"]
depends_on = []

[[tasks]]
id = 2
command = "echo"
args = ["bar"]
depends_on = [1]

[[tasks]]
id = 4
command = "echo"
args = ["done"]
depends_on = [3]

[[tasks]]
id = 3
command = "echo"
args = ["3"]
depends_on = [2]
```
