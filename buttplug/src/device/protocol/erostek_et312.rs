use crate::create_buttplug_protocol;

create_buttplug_protocol!(
    // Protocol name
    ErostekET312,
    // Use the default protocol creator implementation. No special init needed.
    true,
    // No extra members
    (),
    // No implementations. Just see if anything even runs.
    ((VibrateCmd, {Ok(messages::Ok::default().into())}))
);
