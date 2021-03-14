//! A generic stack-based state machine.
//! This state machine contains a stack of states and handles transitions between them.
//! StateTransition happen based on the return value of the currently running state's functions.
//! Only one state can run at once.

/// A transition from one state to the other.
/// ## Generics
/// - S: State data, the data that is sent to states for them to do their operations.
pub enum StateTransition<S> {
    /// Stay in the current state.
    None,
    /// End the current state and go to the previous state on the stack, if any.
    /// If we Pop the last state, the state machine exits.
    Pop,
    /// Push a new state on the stack.
    Push(Box<dyn State<S>>),
    /// Pop all states on the stack and insert this one.
    Switch(Box<dyn State<S>>),
    /// Pop all states and exit the state machine.
    Quit,
}

/// Trait that states must implement.
///
/// ## Generics
/// - S: State data, the data that is sent to states for them to do their operations.
pub trait State<S> {
    /// Called when the state is first inserted on the stack.
    fn on_start(&mut self, _state_data: &mut S) {}
    /// Called when the state is popped from the stack.
    fn on_stop(&mut self, _state_data: &mut S) {}
    /// Called when a state is pushed over this one in the stack.
    fn on_pause(&mut self, _state_data: &mut S) {}
    /// Called when the state just on top of this one in the stack is popped.
    fn on_resume(&mut self, _state_data: &mut S) {}
    /// Executed on every frame immediately, as fast as the engine will allow.
    /// If you need to execute logic at a predictable interval (for example, a physics engine)
    /// it is suggested to use the state data information to determine when to run such fixed timed
    /// logic.
    fn update(&mut self, _state_data: &mut S) -> StateTransition<S> {
        StateTransition::None
    }
}

/// A state machine that holds the stack of states and performs transitions between states.
/// It can be created using
/// ```rust,ignore
/// StateMachine::<()>::default()
/// ```
/// ## Generics
/// - S: State data, the data that is sent to states for them to do their operations.
#[derive(Default)]
pub struct StateMachine<S> {
    state_stack: Vec<Box<dyn State<S>>>,
}

impl<S> StateMachine<S> {
    /// Returns if the state machine still has states in its stack.
    pub fn is_running(&self) -> bool {
        !self.state_stack.is_empty()
    }

    /// Updates the state at the top of the stack with the provided data.
    /// If the states returns a transition, perform it.
    pub fn update(&mut self, state_data: &mut S) {
        let trans = match self.state_stack.last_mut() {
            Some(state) => state.update(state_data),
            None => StateTransition::None,
        };

        self.transition(trans, state_data);
    }

    fn transition(&mut self, request: StateTransition<S>, state_data: &mut S) {
        match request {
            StateTransition::None => (),
            StateTransition::Pop => self.pop(state_data),
            StateTransition::Push(state) => self.push(state, state_data),
            StateTransition::Switch(state) => self.switch(state, state_data),
            StateTransition::Quit => self.stop(state_data),
        }
    }

    fn switch(&mut self, mut state: Box<dyn State<S>>, state_data: &mut S) {
        if let Some(mut state) = self.state_stack.pop() {
            state.on_stop(state_data)
        }

        state.on_start(state_data);
        self.state_stack.push(state);
    }

    /// Push a state on the stack and start it.
    /// Pauses any previously active state.
    fn push(&mut self, mut state: Box<dyn State<S>>, state_data: &mut S) {
        if let Some(state) = self.state_stack.last_mut() {
            state.on_pause(state_data);
        }

        state.on_start(state_data);
        self.state_stack.push(state);
    }

    fn pop(&mut self, state_data: &mut S) {
        if let Some(mut state) = self.state_stack.pop() {
            state.on_stop(state_data);
        }

        if let Some(state) = self.state_stack.last_mut() {
            state.on_resume(state_data);
        }
    }

    /// Removes all currently running states from the stack.
    pub fn stop(&mut self, state_data: &mut S) {
        while let Some(mut state) = self.state_stack.pop() {
            state.on_stop(state_data);
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::*;

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

    #[test]
    fn sm_test() {
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
}
