fn main() {

    // TUPLES
    // Fixed size once declared
    // Each position has a type
    let my_tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // To get individual values from a tuple, you can use destructuring
    // Tuples can also be accessed by index
    let x = my_tup.1; // == 6.4
    let y = my_tup.0; // == 500

    // Arrays
    // Fixed length
    // All elements of same type
    let a = [1, 2, 3, 4];
    // Useful if you want to allocate data on stack rather than heap
    // Type annotating arrays:
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // Initializing with a fixed value:
    let a = [3; 5]; // == [3,3,3,3,3]
    // Accessing array elements
    let first_array_element = a[0];
}