
use piston::{
    keyboard,
};
use {
    Cursor,
    KeyPressedCursor,
    SequenceCursor,
    StartState,
    State,
    InvertCursor,
    WaitCursor,
    WhenAllCursor,
    WhileCursor,
};

/// Describes an event.
pub enum Event<A> {
    /// A key was pressed.
    KeyPressed(keyboard::Key),
    /// An event where some action is performed.
    Action(A),
    /// Returns `Success` <=> `Failure`.
    Invert(Box<Event<A>>),
    /// An event waiting for time in seconds to expire.
    ///
    /// This event never fails.
    Wait(f64),
    /// An event where sub events are happening sequentially.
    ///
    /// The sequence fails if one of the sub events fails.
    /// The sequence succeeds if all the sub events succeeds.
    /// Can be used as a short-circuited logical AND block.
    Sequence(Vec<Event<A>>),
    /// While an event is executing, run a sequence of events in a loop..
    While(Box<Event<A>>, Vec<Event<A>>),
    /// An event where all sub events happen.
    WhenAll(Vec<Event<A>>),
}

impl<A: StartState<S>, S> Event<A> {
    /// Creates a cursor structure from an event structure.
    ///
    /// The cursor structure keeps track of the state.
    /// You can define your own actions and use the combinations
    /// to create more complex states.
    pub fn to_cursor<'a>(&'a self) -> Cursor<'a, A, S> {
        match *self {
            KeyPressed(key)
                => KeyPressedCursor(key),
            Action(ref action)
                => State(action, action.start_state()),
            Invert(ref ev)
                => InvertCursor(box ev.to_cursor()),
            Wait(dt)
                => WaitCursor(dt, 0.0),
            Sequence(ref seq)
                => SequenceCursor(seq, 0, box seq[0].to_cursor()),
            While(ref ev, ref rep)
                => WhileCursor(box ev.to_cursor(), rep, 0, box rep[0].to_cursor()),
            WhenAll(ref all)
                => WhenAllCursor(all.iter().map(|ev| Some(ev.to_cursor())).collect()),
        }
    }
}

