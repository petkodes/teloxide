This document describes breaking changes of `teloxide` crate, as well as the ways to update code.
Note that the list of required changes is not fully exhaustive and it may lack something in rare cases.

## 0.9 -> 0.10

### core

We've added some convenience functions to `InlineKeyboardButton` so it's easier to construct it. Consider using them instead of variants:
```diff
-InlineKeyboardButton::new("text", InlineKeyboardButtonKind::Url(url))
+InlineKeyboardButton::url("text", url)
```

`file_size` fields are now `u32`, you may need to update your code accordingly:

```diff
-let file_size: u64 = audio.file_size?;
+let file_size: u32 = audio.file_size;
```

Some places now use `FileMeta` instead of `File`, you may need to change types.

`Sticker` and `StickerSet` now has a `kind` field instead of `is_animated` and `is_video`:

```diff
+use teloxide::types::StickerKind::*;
-match () {
+match sticker.kind {
-    _ if sticker.is_animated => /* handle animated */,
+    Animated => /* handle animated */,
-    _ if sticker.is_video => /* handle video */,
+    Video => /* handle video */,
-    _ => /* handle normal */,
+    Webp => /* handle normal */,
}
```

### teloxide

Teloxide itself doesn't have any major API changes.
Note however that some function were deprecated:
- Instead of `dispatching::update_listeners::polling` use `polling_builder`
- Instead of `Dispatcher::setup_ctrlc_handler` use `DispatcherBuilder::enable_ctrlc_handler`

## 0.7 -> 0.8

### core

`user.id` now uses `UserId` type, `ChatId` now represents only _chat id_, not channel username, all `chat_id` function parameters now accept `Recipient` (if they allow for channel usernames).

If you used to work with chat/user ids (for example saving them to a database), you may need to change your code to account for new types. Some examples how that may look like:
```diff
-let user_id: i64 = user.id;
+let UserId(user_id) = user.id;
db.save(user_id, ...);

-let chat_id: i64 = db.get();
+let chat_id = ChatId(db.get());
bot.send_message(chat_id, "Hi!").await?;
```

`RequestError::RetryAfter` now has a field of type `Duration`, instead of `i32`.

### teloxide

The old dispatching system was removed. If you still hasn't moved to the new one, read the [0.5 -> 0.6 migration guide] for more information on this topic. Note that since the old dispatching was removed, the new dispatching system now lives in the `dispatching` module, **not** `dispatching2` module.

If you implement `UpdateListener` yourself, note that `StopToken` is now required to be `Send`.

`BotCommand` trait was renamed to `BotCommands`. `BotCommands::descriptions` not returns `CommandDescriptions` instead of `String`. To get string, you can call `.to_string()`.

`#[derive(DialogueState)]` is deprecated in favour of `teloxide::handler!`, a more flexible API for dealing with dialogues. [`examples/dialogue.rs`](https://github.com/teloxide/teloxide/blob/03521bfd3d68f6f576dcc44b5473aaa5ce9b553f/examples/dialogue.rs) shows how to use it.

[0.5 -> 0.6 migration guide]: #05---06

## 0.6 -> 0.7

### teloxide

In order to make `Dispatcher` implement `Send`, `DispatcherBuilder::{default_handler, error_handler}` now accept handlers that implements `Send + Sync`. If you used `!Send` or `!Sync` handlers here, you may need to change that.

## 0.5 -> 0.6

### core

 - `InputFile` now can't be created like `InputFile::Url(url)` or matched on, use constructors like `InputFile::url`, `InputFile::file`, etc.
 - `RequestError` and `DownloadError` error variants were slightly renamed
- `ChatPermissions` is now bitflags.

### teloxide

v0.6 of teloxide introduces a new dispatching model based on the [chain of responsibility pattern]. To use it, you need to replace `prelude` with `prelude2` and `dispatching` with `dispatching2`. Instead of using old REPLs, you should now use `teloxide::repls2`.

The whole design is different than the previous one based on Tokio streams. In this section, we are only to address the most common usage scenarios.

First of all, now there are no streams. Instead of using streams, you use [`dptree`], which is a more suitable alternative for our purposes. Thus, if you previously used `Dispatcher::messages_handler`, now you should use `Update::filter_message()`, and so on.

Secondly, `Dispatcher` has been split into two separate abstractions: `DispatcherBuilder` and `Dispatcher`. The calling sequence is simple: you call `Dispatcher::builder(bot, handler)`, set up your stuff, and then call `.build()` to obtain `Dispatcher`. Later, you can `.setup_ctrlc_handler()` on it and finally `.dispatch()` (or `.dispatch_with_listener()`).

Lastly, the dialogue management system has been greatly simplified. Just compare the [new `examples/dialogue.rs`](https://github.com/teloxide/teloxide/blob/25f863402d4f377f573ce2ba394f5b768ee8052e/examples/dialogue.rs) with the [old one](https://github.com/teloxide/teloxide/tree/2a6067fe94773a0015627a6aaa1930b8f88b6da0/examples/dialogue_bot/src) to see the difference. Now you don't need `TransitionIn`, `TransitionOut`, `#[teloxide(subtransition)]`, etc. All you need is to derive `DialogueState` for your FSM enumeration, call `.enter_dialogue()` and write handlers for each of a dialogue's states. Instead of supplying dependencies in the `aux` parameter of `Transition::react`, you can just call `.dependencies()` while setting up the dispatcher and all the dependencies will be passed to your handler functions as parameters.

For more information, please look at the appropriate documentation pages and the [updated examples](https://github.com/teloxide/teloxide/tree/master/examples). Note that, in one of the upcoming releases, the old dispatching model will be removed, so we highly encourage you to migrate your bots to the new one.

Thanks for using teloxide!

[chain of responsibility pattern]: https://en.wikipedia.org/wiki/Chain-of-responsibility_pattern
[`dptree`]: https://github.com/p0lunin/dptree

## 0.4 -> 0.5

### core

#### Field type changes

Types of some fields were changed to be more accurate. 
If you used them, you may need to change types in your code too.

Example:
```diff
let ps: PhotoSize = /* ... */;
-let w: i32 = ps.width;
+let w: u32 = ps.width;
```

List of changed types:
- `PhotoSoze::width`: `i32` -> `u32`
- `PhotoSoze::height`: `i32` -> `u32`
- `Restricted::until_date`: `i32` -> `DateTime<Utc>`
- `Kicked::until_date` (`Banned::until_date`): `i32` -> `DateTime<Utc>`
- `PublicChatSupergroup::slow_mode_delay`: `Option<i32>` -> `Option<u32>`
- `User::id`: `i32` -> `i64` (note: all methods which are accepting `user_id` were changed too)


#### Method output types

In teloxide `v0.4` (core `v0.2`) some API methods had wrong return types.
This made them practically unusable as they've always returned parsing error.
On the offchance you were using the methods, you may need to adjust types in your code.

List of changed return types:
- `get_chat_administrators`: `ChatMember` -> `Vec<ChatMember>`
- `send_chat_action`: `Message` -> `True`
- `leave_chat`: `String` -> `True`
- `pin_chat_message`: `String` -> `True`
- `set_chat_description`: `String` -> `True`
- `set_chat_photo`: `String` -> `True`
- `set_chat_title`: `String` -> `True`
- `unpin_all_chat_messages`: `String` -> `True`
- `unpin_chat_message`: `String` -> `True`


#### Method parameter types

Some API methods accept different types now. 
If you've used changed parameters, you need to adjust code for new types.

Examples:
```diff
let bot = Bot::new("TOKEN").auto_send();

-bot.set_webhook("url").await?;
+bot.set_webhook(Url::parse("url").unwrap()).await?;

let link = bot
    .create_chat_invite_link(chat_id)
-    .expire_date(timestamp)
# Note: this is not the only way to create `DateTime`. Refer to `chrono` docs for more.
+    .expire_date(DateTime::<Utc>::from_utc(
+        NaiveDateTime::from_timestamp(timestamp, 0), Utc)
+    )
    .await?;
```

See also: [teloxide examples fixes](https://github.com/teloxide/teloxide/pull/408/files/369e43aa7ed1b192d326e6bdfe76f3560001353f..18f88cc034e97fd437c48930728c1d5d2da7a14d).

List of changed required params:
- `SetWebhook::url`: `String` -> `Url`

List of changed optional params:
- `AnswerCallbackQuery::url`: `String` -> `Url`
- `SendInvoice::photo_url`: `String` -> `Url`
- `CreateChatInviteLink::expire_date`: `i64` -> `DateTime<Utc>` 
- `EditChatInviteLink::expire_date`: `i64` -> `DateTime<Utc>` 
- `KickChatMember::until_date`: `u64` -> `DateTime<Utc>` 
- `RestrictChatMember::until_date`: `u64` -> `DateTime<Utc>` 
- `SendPoll::close_date`: `u64` -> `DateTime<Utc>` 


#### Renamed items

Some items (fields, variants, types, methods) were renamed.
If you used them, you should start using new names.

Example:
```diff
-bot.send_chat_action(chat, ChatAction::RecordAudio).await?;
+bot.send_chat_action(chat, ChatAction::RecordVoice).await?;

-if chat_member.is_kicked() {
+if chat_member.is_banned() {
    /* ... */
}
```

List of renamed items:
- `ChatAction::RecordAudio` -> `RecordVoice`
- `ChatAction::UploadAudio` -> `UploadVoice`
- `ChatMemberKind::Creator` -> `Owner`
- `ChatMemberKind::Kicked` -> `Banned`
- `Creator` -> `Owner`
- `Kicked` -> `Banned`
- `ChatMemberKind::is_Creator` -> `is_owner` *
- `ChatMemberKind::is_kicked` -> `is_banned` *
- `ChatMemberStatus::Creator` -> `Owner`
- `ChatMemberStatus::Kicked` -> `Banned`
- `kick_chat_member` -> `ban_chat_member` *
- `get_chat_members_count` -> `get_chat_member_count` *

\* Old methods are still accessible, but deprecated


#### Added `impl Clone` for {`CacheMe`, `DefaultParseMode`, `Throttle`}

Previously said bot adaptors were lacking `Clone` implementation. 
To workaround this issue it was proposed to wrap bot in `Arc`.
Now it's not required, so you can remove the `Arc`:

```diff
let bot = Bot::new(token).parse_mode(ParseMode::MarkdownV2);
-let bot = Arc::new(bot);
```


### teloxide

#### Mutable reference for dispatching

`Dispatcher::dispatch` and `Dispatcher::dispatch_with_listener` now require mutable (unique) reference to self.
If you've used variable to store `Dispatcher`, you need to make it mutable:

```diff
-let dp = Dispatcher::new();
+let mut dp = Dispatcher::new();
/* ... */
dp.dispatch();
```


#### Listener refactor

`UpdateListener` trait was refactored.
If you've used `polling`/`polling_default` provided by teloxide, no changes are required.
If, however, you've used or implemented `UpdateListener` directly or used a `Stream` as a listener, 
then you need to refactor your code too.

See also: [teloxide examples fixes](https://github.com/teloxide/teloxide/pull/385/files/8785b8263cb4caebf212e2a66a19f73e653eb060..c378d6ef4e524da96718beec6f989e8ac51d1531).


#### `polling_default`

`polling_default` is now async, but removes webhook.

Example fix:
```diff
-let listener = polling_default(bot);
+let listener = polling_default(bot).await;
```