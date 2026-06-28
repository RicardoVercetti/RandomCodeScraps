## Iterators
- anything thats iterable should implement the `Iterator` trait.
- the point is what is returned when the next() function is called.
- its always `Option<T>` type, the iterator runs as long as the first `None` is not returned
- In the example from the book example, the `iter` and the `iter_mut` example loops and copies the elements to a new location, it's kinda inefficient?

## doubts
- what about the iterator that is lazy? don't access the data until the method is called?
- try to have some kinda pointer number, so that when the next is called, get element with the pointer. Perhaps the struct from the book example can have one more member called pointer!
- why does the comparison of an owned value and borrowed value not directly usable in an if condition with `==`?
