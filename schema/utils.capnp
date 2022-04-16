@0x9acbae5536535c84;

# Equivalent to Rust's Option enum
struct Option(SomeType) {
    union {
        none @0: Void;
        some @1: SomeType;
    }
}