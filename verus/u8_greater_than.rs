use vstd::prelude::*;

verus! {

fn ct_gt_u8(a: u8, b: u8) -> (result: u8)
    ensures
        result == 0 || result == 1,
        (result == 1) == (a > b),
{
    let gtb = a & !b;
    let ltb_1 = (!a & b) | ((!a & b) >> 1);
    let ltb_2 = ltb_1 | (ltb_1 >> 2);
    let ltb_4 = ltb_2 | (ltb_2 >> 4);
    let bit_0 = gtb & !ltb_4;
    let bit_1 = bit_0 | (bit_0 >> 1);
    let bit_2 = bit_1 | (bit_1 >> 2);
    let bit_4 = bit_2 | (bit_2 >> 4);
    let result = bit_4 & 1;
    assert(
        gtb == (a & !b)
            && ltb_1 == ((!a & b) | ((!a & b) >> 1))
            && ltb_2 == (ltb_1 | (ltb_1 >> 2))
            && ltb_4 == (ltb_2 | (ltb_2 >> 4))
            && bit_0 == (gtb & !ltb_4)
            && bit_1 == (bit_0 | (bit_0 >> 1))
            && bit_2 == (bit_1 | (bit_1 >> 2))
            && bit_4 == (bit_2 | (bit_2 >> 4))
            && result == (bit_4 & 1)
            ==> ((result == 0 || result == 1) && ((result == 1) == (a > b)))
    ) by (bit_vector);
    result
}

fn main() {}

}
