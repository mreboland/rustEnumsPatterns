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


    
    // Tuple and Struct Patterns

    // Tuple patterns match tuples. They're useful any time we want to get multiple pieces of data involved in a single match:
    fn describe_point(x: i32, y: i32) -> &'static str {
        use std::cmp::Ordering::*;

        match (x.cmp(&0), y.cmp(&0)) {
            (Equal, Equal) => "at the origin",
            (_, Equal) => "on the x axis",
            (Equal, _) => "on the y axis",
            (Greater, Greater) => "in the first quadrant",
            (Less, Greater) => "in the second quadrant",
            _ => "somewhere else"
        }
    }

    // Struct patterns use curly braces, just like struct expressions. They contain a subpattern for each field:
    match balloon.location {
        Point { x: 0, y: height } =>
            println!("straight up {} meters", height),
        Point { x: x, y: y} =>
            println!("at ({}m, {}m", x, y)
    }

    // In the above example, if the first arm matches, then balloon.location.y is stored in the new local variable height.

    // Suppose balloon.location is Point { x: 30, y: 40 }. Rust checks each component of each pattern in turn (see page 357 for diagram).

    // The second arm matches, so the output would be "at (30m, 40m)".

    // Patterns like Point { x: x, y: y } are common when matching struct, and the redundant names are visual clutter, so Rust has a shorthand for this: Point(x, y). The meaning is the same. This pattern still stores a point's x field in a new local x and its y field in a new local y.

    // Even with the shorthand, it's cumbersome to match a large struct when we only care about a few fields:
    match get_account(id) {
        ...
        Some(Account {
            name, language, // <--- the 2 things we care about
            id: _, status: _, address: _, birthday: _, eye_colour: _,
            pet: _, security_question: _, hashed_innermost_secret: _,
            is_adamantium_preferred_customer: _}) =>
          language.show_custom_greeting(name)
    }

    // To avoid the above, use .. to tell Rust we don't care about any of the other fields:
    Some(Account { name, language, .. }) =>
        language.show_custom_greeting(name)



    // Reference Patters

    // Rust patterns support two features for working with references. ref patterns borrow parts of a matched value. & patterns match references.

    // Matching on a noncopyable value moves the value. Continuing with the account example, this code would be invalid:
    match account {
        Account { name, language, .. } => {
            ui.greet(&name, &language);
            ui.show_settings(&account); // error, use of moved value `account`
        }
    }

    // Here, the fields account.name and account.language are moved into local variables name and language. The rest of account is dropped. That's why we can't call methods on account afterward.

    // If name and language were both copyable values, Rust would copy the fields instead of moving them, and this code would be fine. But suppose these are String. What can we do?

    // We need a kind of pattern that borrows matched values instead of moving them. The ref keyword does just that:
    match account {
        Account { ref name, ref language, .. } => {
            ui.greet(name, language);
            ui.show_settings(&account); // ok
        }
    }

    // Now the local variables name and language are references to the corresponding fields in account. Since account is inly being borrowed, not consumed, it's OK to continue calling methods in it.

    // We can use ref mut to borrow mut references:
    match line_result {
        Err(ref err) => log_error(err), // `err` is &Error (shared ref)
        Ok(ref mut line) => { // `line` is &mut String (mut ref)
            trim_comments(line); // modify the String in place
            handle(line);
        }
    }

    // The pattern Ok(ref mut line) matches any success result and borrows a mut ref to the success value stored inside it.

    // The opposite kind of ref pattern is the & pattern. A pattern starting with & matches a ref.
    match sphere.center() {
        &Point3d { x, y, z } => ...
    }

    // In this example, suppose sphere.center returns a ref to a private field of sphere, a common pattern in Rust. The value return is the address of a Point3d. If the center is a the origin, then sphere.center() returns &Point3d { x: 0.0, y: 0.0, z: 0.0 }. See page 360 for diagram.

    // This is a bit tricky because Rust is following a pointer here, an action we usually associate with the * operator, not the & operator. The thing to remember is that patterns and expressions are natural opposites. The expression (x, y) makes two values into a new tuple, but the pattern (x, y) does the opposite. It matches a tuple and breaks out the two values. It's the same with &. In an expression, & creates a reference. In a pattern, & matches a reference.

    // Matching a ref follows all the rules we've come to expect. Lifetimes are enforced. We can't get mut access via a shared ref. We can't move a value out of a ref, even a mut ref. When we match &Point3d{ x, y, z }, the variables x, y, and z receive copies of the coordinates, leaving the original Point3d value intact. It works because those fields are copyable. If we try the same thing on a struct with noncopyable fields, we'll get an error:
    match friend.borrow_car() {
        Some(&Car { engine, .. }) => // error, can't move out of borrow
        ...
        None => {}
    }

    // Scrapping a borrowed car for parts is not nice, and Rust won't stand for it. We can use a ref pattern to borrow a ref to a part. We just don't own it.
    Some(&Car { ref engine, .. }) => // ok, engine is a ref

    // Let's look at one more example of an & pattern. Suppose we have an iterator chars over the characters in a string, and it has a method chars.peek() that returns an Option<&char>. A reference to the next character, if any (Peekable iterators do in fact return an Option<&Item Type>, more in chapt 15).

    // A program can use an & pattern to get the pointed_to character:
    match chars.peek() {
        Some(&c) => println!("comming up: {:?}", c),
        None => println!("end of chars")
    }



    // Matching Multiple Possibilities

    // The vertical bar(|) can be used to combine several patterns in a single match arm:
    let at_end =
        match chars.peek() {
            Some(&'\r') | Some(&'\n') | None => true,
            _ => false
        };

    // In an expression, | is the bitwise OR operator, but here it works more like the | symbol in a regular expression. at_end is set to true if chars.peek() matches any of the three patterns.

    // Use ... to match a whole range of values. Range patterns include the begin and end values, so '0' ... '9' matches all the ASCII digits:
    match next_char {
        '0' ... '9' =>
            self.read_number(),
        'a' ... 'z' | 'A' ... 'Z' =>
            self.read_word(),
        ' ' | '\t' | '\n' =>
            self.skip_whitespace(),
        _ =>
            self.handle_punctuation()
    }

    // Ranges in patterns are inclusive, so that both '0' and '9' match the pattern '0' ... '9'. By contrast, range expressions (written with two dots, as in for n in 0..100) are half-open, or exclusive (covering 0 but not 100). The reason for the inconsistency is simply that exclusive ranges are more useful for loops and slicing, but inclusive ranges are more useful in pattern matching.



    // Pattern Guards

    // Use the if keyword to add a guard to a match arm. The match succeeds only if the guard evaluates to true:
    match robot.last_known_location() {
        Some(point) if self.distance_to(point) < 10 =>
            short_distance_strategy(point),
        Some(point) =>
            long_distance_strategy(point),
        None =>
            searching_strategy()
    }

    // If a pattern moves any values, we can't put a guard on it. The guard might evaluate to false, and then Rust would go on to the next pattern. But it can't do that if we've moved bits out of the value to be matched. Therefore, the preceding code works only if point is copyable. If it's not, we'll get an error:
    // error: cannot bind by-move into a pattern guard
    // enums_move_into_guard.rs...

    // The workaround would be to change the pattern to borrow point instead of moving it, Some(ref point).



    // @ patterns

    // Finally, x @ pattern matches exactly like the given pattern, but on success, instead of creating variables for parts of the matched value, it creates a single variable x and moves or copies the whole value into it. For example:
    match self.get_selection() {
        Shape::Rect(top_left, bottom_right) =>
            optimized_paint(&Shape::Rect(top_left, bottom_right)),
        other_shape =>
            paint_outline(other_shape.get_outline()),
    }

    // The first case unpacks a Shape::Rect value, only to rebuild an identical Shape::Rect value on the next line. This can be rewritten to use an @ pattern:
        rect @ Shape::Rect(..) =>
            optimized_paint(&rect),

    // @ patterns are also useful with ranges:
    match chars.next() {
        Some(digit @ '0' ... '9') => read_number(digit, chars)
        ...
    }



    // Where Patterns Are Allowed

    // Although patterns are most prominent in match expressions, they are also allowed in several other places. Typically, in place of an identifier. The meaning is always the same. Instead of just storing a value in a single variable, Rust uses pattern matching to take the value apart.

    // This means patterns can be used to...

    // ...unpack a struct into three new local variables
    let Track { album, track_number, title, .. } = song;

    // ...unpack a function argument that's a tuple
    fn distance_to((x, y): (f64, f64)) -> f64 { ... }

    // ...iterate over keys and values of a HashMap
    for (id, document) in &cache_map {
        println!("Document #{}: {}", id, document.title);
    }

    // ...automatically dereference an argument to a closure
    // (handy because sometimes other code passes use a ref
    // when we'd rather have a copy)
    let sum = numbers.fold(0, |a, &num| a + num);

    // The above saves a few lines of code. The same concept exists in JS, called destructuring, and Python, unpacking.

    // In all four examples, the patterns used are guaranteed to match. Patterns that always match are special in Rust. They're called irrefutable patterns, and they're the only patterns allowed in the four places shown above (after let, in function arguments, after for, and in closure arguments).

    // A refutable pattern is one that might not match, like Ok(x), which doesn't match an error result, or '0' ... '9', which doesn't match the character 'Q'. Refutable patterns can be used in match arms, because match is designed for them. If on patterns fails to match, it's clear what happens next. The four examples above are places in Rust program where a pattern can be handy, but the languges doesn't allow for match failure.

    // Refutable patterns are also allowed in if let and while let expressions, which can be used to...

    // ...handle just one enum variant specially
    if let RoughTime::InTheFuture(_, _) = user.date_of_birth() {
        user.set_time_traveler(true);
    }

    // ...run some code only if a table lookup succeeds
    if let Some(document) = cache_map.get(&id) {
        return send_cached_response(document);
    }

    // ...repeatedly try something until it succeeds
    while let Err(err) = present_cheesy_anti_robot_task() {
        log_robot_attempt(err);
        // let the user try again (it might still be a human)
    }

    // ...manually loop over an iterator
    while let Some(_) = lines.peek() {
        read_paragraph(&mut lines);
    }



    // Populating a Binary Tree

    // Earlier it was mentioned a how to implement a method. BinaryTree::add(), that adds a node to a BinaryTree of this type:
    enum BinaryTree<T> {
        Empty,
        NonEmpty(Box<TreeNode<T>>)
    }

    struct TreeNode<T> {
        element: T,
        left: BinaryTree<T>,
        right: BinaryTree<T>
    }

    impl<T: Ord>BinaryTree<T> {
        fn add(&mut self, value: T) {
            match *self {
                BinaryTree::Empty =>
                    *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                        element: value,
                        left: BinaryTree::Empty,
                        right: BinaryTree::Empty
                    })),
                BinaryTree::NonEmpty(ref mut node) =>
                    if value <= node.element {
                        node.left.add(value);
                    } else {
                        node.right.add(value);
                    }
            }
        }
    }

    // Line 370 tells Rust that we're defining a method on BinaryTrees of ordered types. This is exactly the same syntax we use to define methods on generic structs, explained in "Defining Methods with impl" in chapt 9.

    // If the existing tree *self is empty, that's the easy case. Lines 374-378 run, changing the Empty tree to a NonEmpty one. The call to Box::new() here allocates a new TreeNode in the heap. When we're done, the tree contains one element. Its left and right subtrees are both Empty.

    // If *self is not empty, we match the pattern on line 379:
    BinaryTree::NonEmpty(ref mut node) =>

    // This pattern borrows a mutable ref to the Box<TreeNode<T>>, so we can access and modify data in that tree node. That ref is named node, and it's in scope from line 380 to 384. Since there's already an element in this node, the code must recursively call .add() to add the new element to either the left or the right subtree.

    // The new method can be used like this:
    let mut tree = BinaryTree::Empty;
    tree.add("Mercury");
    tree.add("Venus");
    ...




}
