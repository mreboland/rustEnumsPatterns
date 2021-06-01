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



    


}
