# OOPS lang

This is a small programming language I'm working on in my spare time. Currently not much works.

## Example syntax

Small talk/objective-c syntax where conditionals are implemented with polymorphic methods. Nothing out of the ordinary.

```
let User = [Class subclass ivars: [@id]];

[User def: #set, do: |id:| {
  @id = (id);
}];

[User def: #follow, do: |user:, source:, block:| {
  [Follow create followee_id: [self id], follower_id: [user id], source: source];

  [block if: || {
    [block call];
  }];
}];

[User def: #id, do: || { @id }];

[User def: #next_match, do: |exclude_crowdsourced:| { @id }];

let user = [User new];
[user id];
[user set id: 123];
[user next_match exclude_crowdsourced: true]

[user follow user: other_user, source: 123]
```
