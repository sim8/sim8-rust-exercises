- Best practices of references
  - should you only use them if you have to (i.e. avoid value getting dropped) or should they be semantically used? OR by default?
- Can you modify an arg without passing it as a reference?
- Type ascription?
- Macros?
- How do you have multiple crates in a package?
- Dereferencing?
- Difference between std and core library


```
        let result = get_result(&guess, word);
        let guess_result = GuessResult { guess, result };
        print_result(result);
    // why don't I get a "value moved" error once used in a struct? Is the value copied?
```
