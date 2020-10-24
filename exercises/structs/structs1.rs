// structs1.rs
// Address all the TODOs to make the tests pass!

// these `&str`s should live at least as long as ColorClassicStruct lives
struct ColorClassicStruct<'a> {
    name: &'a str,
    hex: &'a str,
}

// struct ColorTupleStruct(&'static str, &'static str);
struct ColorTupleStruct<'a>(&'a str, &'a str);

#[derive(Debug)]
struct UnitStruct();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // DONE: Instantiate a classic c struct!
        let green = ColorClassicStruct {
            name: "green",
            hex: "#00FF00",
        };

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // DONE: Instantiate a tuple struct!
        let green = ColorTupleStruct("green", "#00FF00");

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // DONE: Instantiate a unit struct!
        let unit_struct = UnitStruct();
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
