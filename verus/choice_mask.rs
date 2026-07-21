use vstd::prelude::*;

verus! {

fn choice_mask(choice: u8) -> (mask: u8)
    requires
        choice == 0 || choice == 1,
    ensures
        choice == 0 ==> mask == 0,
        choice == 1 ==> mask == u8::MAX,
{
    let mask = 0u8.wrapping_sub(choice);
    if choice == 0 {
        assert(mask == 0);
    } else {
        assert(choice == 1);
        assert(mask == u8::MAX);
    }
    mask
}

fn conditional_select_u8(a: u8, b: u8, choice: u8) -> (selected: u8)
    requires
        choice == 0 || choice == 1,
    ensures
        choice == 0 ==> selected == a,
        choice == 1 ==> selected == b,
{
    let mask = choice_mask(choice);
    let selected = a ^ (mask & (a ^ b));
    if choice == 0 {
        assert(mask == 0);
        assert(a ^ (0u8 & (a ^ b)) == a) by (bit_vector);
    } else {
        assert(choice == 1);
        assert(mask == u8::MAX);
        assert(a ^ (u8::MAX & (a ^ b)) == b) by (bit_vector);
    }
    selected
}

fn conditional_assign_u8(current: u8, other: u8, choice: u8) -> (assigned: u8)
    requires
        choice == 0 || choice == 1,
    ensures
        choice == 0 ==> assigned == current,
        choice == 1 ==> assigned == other,
{
    let mask = choice_mask(choice);
    let assigned = current ^ (mask & (current ^ other));
    if choice == 0 {
        assert(mask == 0);
        assert(current ^ (0u8 & (current ^ other)) == current) by (bit_vector);
    } else {
        assert(choice == 1);
        assert(mask == u8::MAX);
        assert(current ^ (u8::MAX & (current ^ other)) == other) by (bit_vector);
    }
    assigned
}

fn conditional_swap_u8(a: u8, b: u8, choice: u8) -> (swapped: (u8, u8))
    requires
        choice == 0 || choice == 1,
    ensures
        choice == 0 ==> swapped == (a, b),
        choice == 1 ==> swapped == (b, a),
{
    let mask = choice_mask(choice);
    let t = mask & (a ^ b);
    let swapped = (a ^ t, b ^ t);
    if choice == 0 {
        assert(mask == 0);
        assert(a ^ (0u8 & (a ^ b)) == a) by (bit_vector);
        assert(b ^ (0u8 & (a ^ b)) == b) by (bit_vector);
    } else {
        assert(choice == 1);
        assert(mask == u8::MAX);
        assert(a ^ (u8::MAX & (a ^ b)) == b) by (bit_vector);
        assert(b ^ (u8::MAX & (a ^ b)) == a) by (bit_vector);
    }
    swapped
}

fn main() {}

}
