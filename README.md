# Tre

A small programming language.

## Specification

### Expressions

```
# Comments start with a # 
# Expression works with + - * / **
1 + 1
1 - 1
1 * 1
1 / 1
1 ** 1
```

### Variables

```
let one = 123;
let type: i32 = 1337;
let more = one + type;
```

### Functions

```
def add(x: i32, y: i32): i32 {
    return x + y;
}

def void() {
    print("Hello world");
}

# Every program needs a main function
# that will be executed
def main() {
    void();
    print(add(1, 1));
}
```

### Loop, If, etc

```
if (some_bool) {
    # do something
} else if (another_bool) {
    # do something else
} else {
    # else
}
```

```
loop {
    # an endless loop
}

while (some_bool) {
    # a normal while loop
}
```
