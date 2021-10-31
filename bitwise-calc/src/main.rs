/// Simple bitwise calculator in rust
/// Supports numbers upto 255


// Function to turn u8 into bool array
fn bool_array_of(u8: u8) -> [bool; 8] {
    let mut u: [bool; 8] = [false; 8];
    for bit in 0..8 {
        // Byte Shift and Check Against 0
        u[bit] = &u8 & (1 << bit) != 0
    }
    return u;
}

// Function to turn bool_array into u8
fn u8_of(bool_array: [bool; 8]) -> u8 {
    let mut u8: u8 = 0;
    //Iterate and left shift
    for bool in bool_array.iter().rev() {
        u8 = (u8 << 1) | *bool as u8;
    }
    return u8;
}

// Half adder function (Does not accept carry)
fn byte_half_adder(a: &bool, b: &bool) -> (bool, bool) {
    /// Returns sum, carry
    /// The sum a xor of both args
    /// Carry is an & if they are both one, then there will be a carry
    (a ^ b, a & b)
}

// Full adder function (Accepts carry)
fn byte_full_adder(a: &bool, b: &bool, carry_in: &bool) -> (bool, bool) {
    // Same as above except with an additional carry input
    let sum = carry_in ^ (a ^ b);
    let carry_out = (a & b) | (carry_in & (a ^ b));
    return (sum, carry_out);
}

fn byte_half_subtractor(a: &bool, b: &bool) -> (bool, bool) {
    /// Steal is if b > a and difference is a > b
    /// Greater than is a xor of two values and an and of the one you want to check if is greater
    let steal = (a ^ b) & b;
    let difference = ((a ^ b) & a) || steal;
    return (difference, steal);
}

fn byte_full_substractor(a: &bool, b: &bool, borrow_in: &bool) -> (bool, bool) {
    /// Same as above except you can also have a borrow with a b
    /// You cannot have !a & b & c at the same time
    let steal = (a & b & borrow_in) | (!a & (b | borrow_in));
    let difference = (a & !(b | borrow_in)) | steal;
    return (difference, steal);
}


fn byte_add(a: u8, b: u8) -> u8 {
    let a_bit_array = bool_array_of(a);
    let b_bit_array = bool_array_of(b);
    let mut sum_array: [bool; 8] = [false; 8];
    let mut carry = false;

    for bit in 0..8 {
        let tuple;
        if (bit == 0) {
            tuple = byte_half_adder(&a_bit_array[bit], &b_bit_array[bit]);
        } else {
            tuple = byte_full_adder(&a_bit_array[bit], &b_bit_array[bit], &carry);
        }
        sum_array[bit] = tuple.0;
        carry = tuple.1;
    }
    return u8_of(sum_array);
}

fn byte_subtract(a: u8, b: u8) -> u8 {
    if b > a { 0 } else { // This is cheating a bit but we don't support negative numbers 😐
        let a_bit_array = bool_array_of(a);
        let b_bit_array = bool_array_of(b);
        let mut difference_array: [bool; 8] = [false; 8];
        let mut borrow = false;

        for bit in 0..8 {
            let tuple;
            if (bit == 0) {
                tuple = byte_half_subtractor(&a_bit_array[bit], &b_bit_array[bit]);
            } else {
                tuple = byte_full_substractor(&a_bit_array[bit], &b_bit_array[bit], &borrow);
            }
            difference_array[bit] = tuple.0;
            borrow = tuple.1;
        }
        return u8_of(difference_array);
    }
}


fn main() {
    let res = byte_add(2, 2);
    println!("{:?}", res)
}

