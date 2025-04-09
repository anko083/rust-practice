use itertools::Itertools;

fn main() {
    for perm in (1..=8).permutations(8) {
        let m = perm[0];
        let u = perm[1];
        let x = perm[2];
        let a = perm[3];
        let s = perm[4];
        let l = perm[5];
        let o = perm[6];
        let n = perm[7];

        let muxa = m * 1000 + u * 100 + x * 10 + a;
        let slon = s * 1000 + l * 100 + o * 10 + n;

        if muxa * a == slon {
            println!(
                "\n  {}\n×    {}\n-------\n  {}\n",
                muxa, a, slon
            );
        }
    }
}
