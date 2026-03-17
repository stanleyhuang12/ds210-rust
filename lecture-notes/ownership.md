# Notes on ownership

- [[stack]] and [[heap]] are different parts of the memory that can be used to store data
	- Stack: Uses the *last in, first out* method. (e.g., we stack a plate on top and remove a plate on top).
		- Must specify the data size
		- integers, booleans, fixed-size arrays
		- How are data structured on the stack?
			- example:
			  collapsed:: true
				- ptr (pointer): 0x64032fc68d60
				  Length (memory used or initialized elements): 3  
				  Capacity (space available): 4  
	- Heap: requires specifying a certain amount of space but not explicitly how much for the information
	  collapsed:: true
		- The memory allocation will find an empty spot on the heap to store data and return a pointer (which is the address of where the data is on the heap).
		- We *allocate* memory on the heap but push values onto the stack.
		- we actually push the pointer (data on whether the space is occupied and the address) onto the stack because it is a fixed siz.
		- examples: string, vec, box
	- **Pushing the stack is faster than allocating on the heap**: Pushing on the stack faster because it just immediately add data on top of the stack, while allocating on the heap requires manual search of open spaces, verifying the space is good enough to store the data, returning a pointer and performing bookkeeping.
- Rust users an ownership-based model.
  collapsed:: true
	- **Scope**: the range within a program for which an item is valid
		- Out of scope error: a variable declared within a function will be out of scope on the program exits the function
		- A variable comes into scope once it is defined
	- 1. Each value or data has an owner
		2. There can only be one owner at a time.
		3. When owner goes out of scope, the value will be dropped.

	- Let s1 = String::from("hello!");
	- This creates a mutable, growable string "hello!" which is allocated on the heap. Additionally, the pointer and the metadata will be stored on the stack. So, basically s1 is the owner of the data.
	- When we call let s2 = s1; we `move` the ownership from s1 to s2. s1 is basically the data on the stack that references the address of the data on the heap. Rust invalidates s1 immediately and just lets s2 have the ownership. Rust invalidates so that there are no 2 owners that point to the same data because this could lead to double free memory error, which has security issue. For instance if it had made a copy, then s1 goes out of scope it will call the drop function and if s2 goes out of scope it will call the drop function again on allocated memory that has already been freed up
- Copy makes an implicit duplication of data on the stack. It doesn't involve duplication and new allocation of data on the heap.
	- In general we can think of [[shallow copy]] as copy of references or pointers that have shared underlying data; while [[deep copy]] duplicates the underlying data
- Rust has Copy annotations and .clone implementations
	- Copy makes an implicit duplication of data on the stack. Ownership is explicitly duplicated for these cases. Copy is usually / automatically called for data with known sizes.
		- i.e. let x = 5; (x has copy annotations) but not let y = String::from("hello")
	- Whereas clone is explicitly called and could potentially be expensive copy of the underlying data (i.e., typically data on the heap)? It also creates variables that are independent owners.
- Rust can perform reallocation for you automatically by doubling the capacity when the length exceeds capacity
	- But, we can preallocate with `String::with_capacity(...)`; or `Vec::with_capaciity(...)`
- Rust uses references to borrow data on the stack without getting ownership
	- References just reads the data (i.e., contains a pointer to another pointer)
	- We can make multiple immutable references; references does not allow us to make changes unless we specify that they are mutable
	- [[mutable references]] in Rust allow us to modify the underlying data via a reference.
		- Rust only allows one mutable references at a time
		- No other readable (i.e., immutable) references though
		- Reference scopes are only till the end of the line that it was used. Then, it gets cleaned up.
			-