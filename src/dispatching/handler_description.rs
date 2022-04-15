use std::collections::HashSet;

use dptree::{EventKindDescription, HandlerDescription};
use teloxide_core::types::AllowedUpdate;

/// Handler description that is used by [`Dispatcher`].
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
pub struct DpHandlerDescription {
    allowed: EventKindDescription<AllowedUpdate>,
}

impl DpHandlerDescription {
    pub(crate) fn of(allowed: AllowedUpdate) -> Self {
        let mut set = HashSet::with_capacity(1);
        set.insert(allowed);
        Self { allowed: EventKindDescription::InterestingEventKinds(set) }
    }

    pub(crate) fn allowed_updates(&self) -> Vec<AllowedUpdate> {
        use AllowedUpdate::*;

        match &self.allowed {
            EventKindDescription::InterestingEventKinds(set) => set.iter().copied().collect(),
            EventKindDescription::Entry => panic!("No updates were allowed"),
            EventKindDescription::UserDefined => vec![
                Message,
                EditedMessage,
                ChannelPost,
                EditedChannelPost,
                InlineQuery,
                ChosenInlineResult,
                CallbackQuery,
                ShippingQuery,
                PreCheckoutQuery,
                Poll,
                PollAnswer,
                MyChatMember,
                ChatMember,
            ],
        }
    }
}

impl HandlerDescription for DpHandlerDescription {
    fn entry() -> Self {
        Self { allowed: HandlerDescription::entry() }
    }

    fn user_defined() -> Self {
        Self { allowed: HandlerDescription::user_defined() }
    }

    fn merge_chain(&self, other: &Self) -> Self {
        Self { allowed: self.allowed.merge_chain(&other.allowed) }
    }

    fn merge_branch(&self, other: &Self) -> Self {
        Self { allowed: self.allowed.merge_branch(&other.allowed) }
    }
}
