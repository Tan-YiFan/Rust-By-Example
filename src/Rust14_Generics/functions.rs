// sometimes explicitly specifying the tpye parameters is required

struct A; // concrete

struct S(A); // concrete

struct SGen<T>(T); // generic

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {} // not generic

fn gen_spec_i32(_s: SGen<i32>) {} // not generic

fn generic<T>(_s: SGen<T>) {} // generic

fn main() {
    // Using the non-generic functions
    reg_fn(S(A));          // Concrete type.
    gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
    gen_spec_i32(SGen(6)); // Implicitly specified type parameter `i32`.

    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('a'));

    // Implicitly specified type parameter `char` to `generic()`.
    generic(SGen('c'));
}