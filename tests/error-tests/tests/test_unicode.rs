// Ensures that unicode characters are handled in the JSON output.

fn main() {
    let foo = "❤";
//      ^^^WARN unused variable
//      ^^^NOTE(>=1.17.0) #[warn(unused_variables)]
//      ^^^NOTE(>=1.21.0,<1.22.0) to disable this warning
//      ^^^NOTE(>=1.22.0) to avoid this warning
}
