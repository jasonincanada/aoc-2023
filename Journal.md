
find the 

## Day 16

- first didn't use Direction param to energize()
- could be sped up a lot with bitflags and a better
- could use some pre-processing, does_light_go_from_to() could be cached

## Day 13

`next_k

## Day 12

- did on sunday 17th
- had static sample input at first d'oh

## Day 11 

Almost did this one with Dijkstra.... then BFS

Studied the Bresenham line algorithm pointlessly last night

- the (width + height - 2) formula i felt sure of last night, but it's - 0

In ExpandingSpace::around() I was duplicating all the Positions funnily

---

This could be the last year I write code better than the computer

## Day 10 -

### AI's Righteous Swap

ChatGPT got clever here and switched the last_position/positions, then used the last_position name
since it still contains the actual value we want:

```rust
// Update last_position to the current position and move to the new position
std::mem::swap(&mut last_position, &mut position);
position = position_in_direction(&last_position, &new_direction);
```

I was using `Option<Position>` because...

... but there was a clever way to make up for it, and ChatGPT found it for me.


## Day 8 - something

19ms to compile this regex (in debug vs 1ms in release)!

```rust
        let re = Regex::new(r"^(\w+) = \((\w+), (\w+)\)$")
                    .map_err(|e| e.to_string())?;
```

### Part 2

Measure cycle lengths of each starting point?


## Day 5 -

- finally wrote some parsing code from hand to get practice with propagating errors properly in Rust.
- but I have left out the prompts that got me there, because maybe I was being a bit rude to it at the time
- me after reading part 2: <michaelscott> "nooooo"

```
Yes, the collect() method in Rust is quite powerful and flexible due to Rust's trait system, particularly the FromIterator trait which collect() relies on. The behavior of collect() depends on the type you are trying to collect into. When you use collect() to convert an iterator of Result<T, E> into Result<Vec<T>, E>, collect() will automatically stop and return the first Err it encounters. If there are no errors, it will collect all the Ok values into a Vec<T>.

This behavior is because the Result type implements FromIterator in such a way that it will either return a Result::Ok containing a collection of all the Ok values if all are successful, or a Result::Err containing the first error encountered.

Here's a simplified version of how this works:
```

```rust
impl<T, E> FromIterator<Result<T, E>> for Result<Vec<T>, E> {
    fn from_iter<I: IntoIterator<Item = Result<T, E>>>(iter: I) -> Self {
        let mut vec = Vec::new();

        for item in iter {
            match item {
                Ok(ok) => vec.push(ok),
                Err(err) => return Err(err),
            }
        }

        Ok(vec)
    }
}
```
