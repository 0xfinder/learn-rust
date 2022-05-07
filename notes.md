# Rust

attempt at making notes because i've read this 3 times and still do not understand

## Ownership

memory is managed in rust by the concept of ownership

### the stack and the heap

#### Stack
the stack stores values in the order it gets them and removes the values in the opposite order (LIFO)

term:
adding data -> pushing onto the stack
removing data -> popping off the stack

data has to have a known, fixed size

#### Heap
putting data on heap -> request certain space -> os finds empty spot, marks it as used, returns pointer

this is called allocating on the heap
pointer is a known fixed size, can be stored on stack, but to access data you have to follow the pointer

#### Comparison
Storing data: Pushing data to the stack is faster than allocating to the heap because the os does not have to search for a place to store the data. 

Allocating to the heap req more work as os has to find space & perform bookkeeping

Accessing data: in heap is slower than accessing data on the stack because the os has to follow the pointer to the data.

values passed in function parameters are stored on the stack, after function execution, values are popped off the stack