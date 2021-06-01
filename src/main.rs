fn main() {
    println!("Hello, world!");



    // Patterns

    // Looking at our RoughTime Type from earlier in the enums chapt:
    enum RoughTime {
        InThePast(TimeUnit, u32),
        JustNow,
        InTheFuture(TimeUnit, u32)
    }

    // Suppose you have a RoughTime value and you'd like to display it on a web page. You need to access the TimeUnit and u32 fields inside the value. Rust doesn't let you access them directly, by writing rough_time.0 and rough_time.1, because after all, the value might be RoughTime::JustNow, which has no fields. But then, how can you get the data out?

    // We need a match expression:
    fn rough_time_to_english(rt: RoughTime) -> String {
        match rt {
            RoughTime::InThePast(units, count) =>
                format!("{} {} ago", count, units.plural()),
            RoughTime::JustNow =>
                format!("just now"),
            RoughTime::InTheFuture(units, count) =>
                format!("{} {} from now", count, units.plural())
        }
    }

    // match performs pattern matching. In this example, the patterns are the parts that appear before the => symbol. Patterns that match RoughTime values look just like the expressions used to create RoughTime values. This is no coincidence. Expressions produce values, patterns consume values. The two use a lot of the same syntax.

    // Let's step through what happens when this match expression runs. Suppose rt is the value RoughTime::InTheFuture(TimeUnit::Months, 1). Rust first tries to match this value against the pattern on line 20. It doesn't match because line 20 is for InThePast.

    // Pattern matching on an enum, struct, or tuple works as though Rust is doing a simple left-to-right scan, checking each component of the pattern to see if the value matches it. If it doesn't Rust moves on to the next pattern.

    // The patterns for line 20 and 22 above fail to match but line 24 is a success. InTheFuture matches, ::Months, matches units, and , 1 matches count.

    // When a pattern contains simple identifiers like units and count, those become local variables in the code following the pattern. Whatever is present in the value is copied or moved into the new variables. Rust stores TimeUnit::Months in units and 1 in count, runs line 25, and returns the string "1 months from now"

    // The above output has a minor grammatical issue, which can be fixed by adding another arm to the match:
    RoughTime::InTheFuture(unit, 1) =>
        format!("a {} from now", unit.singular()),

    // This arm matches only if the count field is exactly 1. This new field must be added before like 24. If we added it at the end, Rust will get to it because the pattern on line 24 matches all InTheFuture values.

    // The Rust compiler will warn about an "unreachable pattern" if you make this kind of mistake.

    // Unfortunately, even with the new code, there is still a problem with RoughTime::InTheFuture(TimeUnit::Hours, 1). The result "a hour from now" is not quite right. Such is the English language. This too can be fixed by adding another arm to the match.

    // So far, we've only seen patterns that match enum values. There's more to it than that. Rust patterns are their own little language. See page 352 for a table of patterns.



    // Literals, Variables, and Wildcards in Patterns

    // We've shown match expressions working with enums earlier. Other types can be matched too. We can use match with an integer value. Integer literals like 0 and 1 can serve as patterns:
    match meadow.count_rabbits() {
        0 => {} // nothing to say
        1 => println!("A rabbit is nosing around in the clover."),
        n => println!("There are {} rabbits hopping about in the meadow", n)
    }

    // The pattern 0 matches if there are no rabbits in the meadow. 1 matches if there is just one. If there are two or more rabbits, we reach the third pattern, n. This pattern is just a var name. It can match any value, and the matched value is moved or copied into a new local var. So in this case, the value of meadow.count_rabbits() is stored in a new local variable n, which we then print.

    // Other literals can be used as patterns too, including Booleans, characters, and even strings:
    let calendar =
        match settings.get_string("calendar") {
            "gregorian" => Calendar::Gregorian,
            "chinese" => Calendar::Chinese,
            "ethiopian" => Calendar::Ethiopian,
            other => return parse_error("calendar", other)
        };

    // In this example, other serves as a catch-all pattern, like n in the previous example. These patterns play the same role as a default case in a switch statement, matching values that don't match any of the other patterns.

    // If we need a catch-all pattern, but we don't care about the matched value, we can use a single underscore _ as a pattern. The wildcard pattern:
    let caption =
        match photo.tagged_pet() {
            Pet::Tyrannosaur => "RRRRAAAHHHHHHHHH",
            Pet::Samoyed => "dog thoughts*",
            _ => "I'm cute, love me" // generic caption, works for any pet
        };

    // The wildcard pattern matches any value, but without storing it anywhere. Since Rust requires every match expression to handle all possible values, a wildcard is often required at the end. Even if we're very sure the remaining cases can't occur, we must at least add a fallback arm that panics:
        
    // There are many Shapes, but we only support "selecting"
    // either some text, or everything in a rectangular area.
    // You can't select an ellipse or trapezoid.
    match document.selection() {
        Shape::TextSpan(start, end) => paint_text_selection(start, end),
        Shape::Rectangle(rect) => paint_rect_selection(rect),
        _ => panic!("unexpected selection type")
    }

    // It's worth noting that existing variables can't be used in patterns. Suppose we're implementing a board game with hexagonal spaces, and the player just clicked to move a piece. To confirm that the clock was valid, we might try something like this:
    fn check_move(current_hex: Hex, click: Point) -> game::Result<Hex> {
        match point_to_hex(click) {
            None =>
                Err("That's not a game space."),
            Some(current_hex) => // try to match if user clicked the current_hex
                // (it doesn't work, see explanation below)
                Err("You are already there! You must click somewhere else."),
            Some(other_hex) =>
                Ok(other_hex)
        }
    }

    // This fails because identifiers in patterns introduce new variables. The pattern Some(current_hex) here creates a new local variable current_hex, shadowing the argument current_hex. Rust emits several warnings about this code, in particular, the last arm of the match is unreachable. To fix it, use an if expression:
    Some(hex) =>
        if hex == current_hex {
            Err("You are already there! You must click somewhere else")
        } else {
            Ok(hex)
        }

    // We'll cover guards, which offer another way to solve the above problem.


    
    



}
