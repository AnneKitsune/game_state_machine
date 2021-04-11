<a href="https://crates.io/crates/game_state_machine">
    <img src="https://img.shields.io/crates/v/game_state_machine.svg" alt="Game State Machine" />
</a>

# Game State Machine

Support an Open Source Developer! :hearts:  
[![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/jojolepro)

Read the [documentation](https://docs.rs/game_state_machine).

# Features

* `State` trait that is simple to implement.
* Generic stack-based state machine, for all your needs.
* State update functions.
* State pause and unpause.

# Usage
Add the following to you Cargo.toml file:
```
game_state_machine = "*"
```

Use it like so:
```rust
use game_state_machine::*;

type StateData = (isize, isize);

pub struct Test;

impl State<StateData> for Test {
    fn on_start(&mut self, data: &mut StateData) {
        data.0 += data.1;
    }

    fn on_resume(&mut self, data: &mut StateData) {
        self.on_start(data);
    }

    fn update(&mut self, _data: &mut StateData) -> StateTransition<StateData> {
        StateTransition::Push(Box::new(Test))
    }
}

fn main() {
    let mut sm = StateMachine::<StateData>::default();

    let mut state_data = (0, 10);

    sm.push(Box::new(Test), &mut state_data);
    assert!(state_data.0 == 10);

    sm.update(&mut state_data);
    assert!(state_data.0 == 20);

    sm.stop(&mut state_data);
    assert!(state_data.0 == 20);
    assert!(!sm.is_running())
}
```

### Maintainer Information

* Maintainer: Jojolepro
* Contact: jojolepro [at] jojolepro [dot] com
* Website: [jojolepro.com](https://jojolepro.com)
* Patreon: [patreon](https://patreon.com/jojolepro)

