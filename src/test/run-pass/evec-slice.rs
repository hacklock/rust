fn main() {
    let x : [int]/& = [1,2,3,4,5]/&;
    let mut z = [1,2,3,4,5]/&;
    z = x;
    assert z[0] == 1;
    assert z[4] == 5;
}
