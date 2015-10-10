fn print_vec(vec: Vec<i32>) {
    // unimplemented!()
    for i in vec.iter() {
        println!("{}", i);
    }
}

// Ownsership
// In Rust, every value has an “owning scope,”
// and passing or returning a value means transferring ownership (“moving” it) to a new scope.
// Values that are still owned when a scope ends are automatically destroyed at that point.
fn main() {
    // unimplemented!()
    let mut vec = vec![1, 2, 3];

    // ownership transfer
    // print_vec(vec);
    // Once ownership has been given away, a value can no longer be used.
    // print_vec(vec);

    // use borrowing
    for _ in 0..2 {
        print_vec_borrow(&vec);
    }
}

// borrowing
// What we really want is to grant print_vec temporary access to the vector, and then continue using the vector afterwards.
// This is where borrowing comes in. If you have access to a value in Rust, you can lend out that access to the functions you call. Rust will check that these leases do not outlive the object being borrowed.

// To borrow a value, you make a reference to it (a kind of pointer), using the & operator
fn print_vec_borrow(vec: &Vec<i32>) {
    // unimplemented!()
    for i in vec.iter() {
        println!("{}", i);
    }
}


// Each reference is valid for a limited scope, which the compiler will automatically determine. References come in two flavors:
//
// + Immutable references &T, which allow sharing but not mutation. There can be multiple &T references to the same value simultaneously, but the value cannot be mutated while those references are active.
//
// + Mutable references &mut T, which allow mutation but not sharing. If there is an &mut T reference to a value, there can be no other active references at that time, but the value can be mutated.
//
// Rust checks these rules at compile time; borrowing has no runtime overhead.

// Rust ensures that whenever a mutable borrow is active, no other borrows of the object are active
