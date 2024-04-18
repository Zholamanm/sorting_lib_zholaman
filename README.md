# Sorting Library

This Rust library implements several sorting algorithms, including Quick Sort, Merge Sort, Selection Sort, and Insertion Sort.

## Usage

To use this library, add it as a dependency in your `Cargo.toml`:

And simply import needed sort;

```rust
use sorting_lib_zholaman::sorts::quick_sort::quick_sort;
use sorting_lib_zholaman::sorts::merge_sort::merge_sort;
use sorting_lib_zholaman::sorts::insertion_sort::insertion_sort;
use sorting_lib_zholaman::sorts::selection_sort::selection_sort;
```

```
Examples:
```

Numbers: 
 
```rust
	use sorting_lib_zholaman::sorts::quick_sort::quick_sort;

	fn main() {
		let mut numbers = vec![10, 3, 15, 6, 2];
		quick_sort(&mut numbers);
		println!("{:?}", numbers);
	}
```
  
Screenshot:

- ![Alt text](/../main/Screenshots/Numbers.PNG?raw=true "Optional Title")

 Structs:
 
```rust
	use sorting_lib_zholaman::sorts::quick_sort::quick_sort;

	#[derive(Clone)]
	struct Employee {
		id: u32,
		name: String,
	}

	impl PartialOrd for Employee {
		fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
			self.id.partial_cmp(&other.id)
		}
	}

	impl PartialEq for Employee {
		fn eq(&self, other: &Self) -> bool {
			self.id == other.id
		}
	}

	fn main() {
		let mut employees_quick = vec![
			Employee { id: 2, name: "Alice".to_string() },
			Employee { id: 4, name: "Bob".to_string() },
			Employee { id: 1, name: "Eve".to_string() },
			Employee { id: 3, name: "Charlie".to_string() },
		];

		let mut employees_merge = employees_quick.clone();
		merge_sort(&mut employees_merge);
		println!("Merge Sorted employees by id:");
		for emp in &employees_merge {
			println!("{}: {}", emp.id, emp.name);
		}
	}
```

Screenshot:
- ![Alt text](/../main/Screenshots/Struct.PNG?raw=true "Optional Title")

```toml
[dependencies]
sorting_lib_zholaman = { git = "https://github.com/Zholamanm/sorting_lib_zholaman.git" }
```

This project is licensed under the MIT License - see the [LICENSE](/LICENSE) file for details.
