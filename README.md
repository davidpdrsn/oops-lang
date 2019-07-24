# OOPS lang

This is a small programming language I'm working on in my spare time. Currently not much works.

## Example syntax

Small talk/objective-c syntax where conditionals are implemented with polymorphic methods. Nothing out of the ordinary.

```
// Create class named "User"
[Class subclass name: #User fields: [#id]];

// Define "User#set" method
[User def: #set do: |id:| {
    let @id = id;
}];

// Define "User#follow" method
[User def: #follow do: |user: source: block:| {
    let follow = [Follow new followee_id: [self id] follower_id: [user id] source: source];
    [follow save];

    [block if then: || {
        [block call];
    } else: || {}];
}];

// Define "User#id" method
[User def: #id do: || { return @id; }];

// Define "User#next_match" method
[User def: #next_match do: |exclude_crowdsourced:| {
    return @id;
}];

// Make a variable
let user = [User new];

// Call some methods
[user id];
[user set id: 123];
[user next_match exclude_crowdsourced: true];
[user follow user: other_user source: 123];
```

## TODO

- [x] Lexing
- [x] Parsing
- [ ] Interpretation
    - [x] Build class vtable
    - [ ] Evaluate statements;
- [ ] Compilation to JavaScript
