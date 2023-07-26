/*!
Description of the syntax for regular expressions.

The regular expression syntax is modeled after Perl.

# Matching one character

```plain
.             any character except new line (includes new line with s flag)
[0-9]         any ASCII digit
\d            digit (\p{Nd})
\D            not digit
\pX           Unicode character class identified by a one-letter name
\p{Greek}     Unicode character class (general category or script)
\PX           Negated Unicode character class identified by a one-letter name
\P{Greek}     negated Unicode character class (general category or script)
```

# Character classes

```plain
[xyz]         A character class matching either x, y or z (union).
[^xyz]        A character class matching any character except x, y and z.
[a-z]         A character class matching any character in range a-z.
[[:alpha:]]   ASCII character class ([A-Za-z])
[[:^alpha:]]  Negated ASCII character class ([^A-Za-z])
[x[^xyz]]     Nested/grouping character class (matching any character except y and z)
[a-y&&xyz]    Intersection (matching x or y)
[0-9&&[^4]]   Subtraction using intersection and negation (matching 0-9 except 4)
[0-9--4]      Direct subtraction (matching 0-9 except 4)
[a-g~~b-h]    Symmetric difference (matching `a` and `h` only)
[\[\]]        Escaping in character classes (matching [ or ])
[a&&b]        An empty character class matching nothing
```

Any named character class may appear inside a bracketed `[...]`
character class. For example, `[\p{Greek}[:digit:]]` matches
any Greek or ASCII digit. `[\p{Greek}&&\pL]` matches Greek letters.

Precedence in character classes, from most binding to least:

1. Ranges: `[a-cd]` == `[[a-c]d]`
2. Union: `[ab&&bc]` == `[[ab]&&[bc]]`
3. Intersection, difference, symmetric difference.
All three have equivalent precedence, and are evaluated in
left-to-right order. For example, `[\pL--\p{Greek}&&\p{Uppercase}]` ==
`[[\pL--\p{Greek}]&&\p{Uppercase}]`.
4. Negation: `[^a-z&&b]` == `[^[a-z&&b]]`.

# Composites

```plain
xy    concatenation (x followed by y)
x|y   alternation (x or y, prefer x)
```

This example shows how an alternation works, and what it means to
prefer a branch in the alternation over subsequent branches.

```
use rialight::prelude::*;

let haystack = "samwise";
// If 'samwise' comes first in our alternation, then it is
// preferred as a match, even if the regex engine could
// technically detect that 'sam' led to a match earlier.
let re = reg_exp!(r"samwise|sam");
assert_eq!("samwise", re.find(haystack).unwrap().as_str());
// But if 'sam' comes first, then it will match instead.
// In this case, it is impossible for 'samwise' to match
// because 'sam' is a prefix of it.
let re = reg_exp!(r"sam|samwise");
assert_eq!("sam", re.find(haystack).unwrap().as_str());
```

# Repetitions

```plain
x*        zero or more of x (greedy)
x+        one or more of x (greedy)
x?        zero or one of x (greedy)
x*?       zero or more of x (ungreedy/lazy)
x+?       one or more of x (ungreedy/lazy)
x??       zero or one of x (ungreedy/lazy)
x{n,m}    at least n x and at most m x (greedy)
x{n,}     at least n x (greedy)
x{n}      exactly n x
x{n,m}?   at least n x and at most m x (ungreedy/lazy)
x{n,}?    at least n x (ungreedy/lazy)
x{n}?     exactly n x
```

# Empty matches

```plain
^     the beginning of a haystack (or start-of-line with multi-line mode)
$     the end of a haystack (or end-of-line with multi-line mode)
\A    only the beginning of a haystack (even with multi-line mode enabled)
\z    only the end of a haystack (even with multi-line mode enabled)
\b    a Unicode word boundary (\w on one side and \W, \A, or \z on other)
\B    not a Unicode word boundary
```

The empty regex is valid and matches the empty string.
For example, the empty regex matches `abc` at positions
`0`, `1`, `2` and `3`. When using the top-level `RegExp` on `&str` haystacks,
an empty match that splits a codepoint is guaranteed to never be returned.
However, such matches are permitted when using a [binary RegExp][super::binary::RegExp].
For example:

```
let re = reg_exp!(r"");
let ranges: Vec<_> = re.find_iter("💩").map(|m| m.range()).collect();
assert_eq!(ranges, vec![0..0, 4..4]);

use rialight::util::reg_exp::binary::BytesRegExp;
let re = BytesRegExp::new(r"").unwrap();
let ranges: Vec<_> = re.find_iter("💩".as_bytes()).map(|m| m.range()).collect();
assert_eq!(ranges, vec![0..0, 1..1, 2..2, 3..3, 4..4]);
```

Note that an empty regex is distinct from a regex that can never match.
For example, the regex `[a&&b]` is a character class that represents the
intersection of `a` and `b`. That intersection is empty, which means the
character class is empty. Since nothing is in the empty set, `[a&&b]`
matches nothing, not even the empty string.
*/