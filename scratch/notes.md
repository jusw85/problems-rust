# Run
```
$ cargo run --package aoc2019 --bin day1
```

# Grab input
Get cookie via browser -> Web Developer -> Network -> Copy cookie
```
$ i=1; curl 'https://adventofcode.com/2019/day/$i/input' -H 'Cookie: session=<cookie>' > day$i
```


# GDB tips

## Printing values

```
let nums: &mut [i32]

$ ptype nums

type = struct &mut [i32] {
    data_ptr: *mut i32,
    length: usize,
}

$ p *nums.data_ptr@nums.length
```

```
let nums: &[i32]

ptype nums

type = struct &[i32] {
    data_ptr: *mut i32,
    length: usize,
}

$ p nums[0]@nums.length
```

## Watch variable

```
$ display var
```
