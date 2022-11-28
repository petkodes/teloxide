//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;
use url::Url;

use crate::types::{InlineKeyboardMarkup, LabeledPrice, Message, Recipient};

impl_payload! {
    /// Use this method to send invoices. On success, the sent [`Message`] is returned.
    ///
    /// [`Message`]: crate::types::Message
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SendInvoice (SendInvoiceSetters) => Message {
        required {
            /// Unique identifier for the target private chat
            pub chat_id: Recipient [into],
            /// Product name, 1-32 characters
            pub title: String [into],
            /// Product description, 1-255 characters
            pub description: String [into],
            /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for your internal processes.
            pub payload: String [into],
            /// Payments provider token, obtained via [Botfather]
            ///
            /// [Botfather]: https://t.me/botfather
            pub provider_token: String [into],
            /// Three-letter ISO 4217 currency code, see more on currencies
            pub currency: String [into],
            /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)
            pub prices: Vec<LabeledPrice> [collect],
        }
        optional {
            /// The maximum accepted amount for tips in the smallest units of the currency (integer, **not** float/double). For example, for a maximum tip of `US$ 1.45` pass `max_tip_amount = 145`. See the exp parameter in [`currencies.json`], it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0
            ///
            /// [`currencies.json`]: https://core.telegram.org/bots/payments/currencies.json
            pub max_tip_amount: u32,
            /// A JSON-serialized array of suggested amounts of tips in the smallest units of the currency (integer, **not** float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed _max_tip_amount_.
            pub suggested_tip_amounts: Vec<u32> [collect],
            /// Unique deep-linking parameter. If left empty, **forwarded copies** of the sent message will have a Pay button, allowing multiple users to pay directly from the forwarded message, using the same invoice. If non-empty, forwarded copies of the sent message will have a URL button with a deep link to the bot (instead of a Pay button), with the value used as the start parameter
            pub start_parameter: String [into],
            /// A JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
            pub provider_data: String [into],
            /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for.
            pub photo_url: Url,
            /// Photo size in bytes
            pub photo_size: String [into],
            /// Photo width
            pub photo_width: String [into],
            /// Photo height
            pub photo_height: String [into],
            /// Pass _True_, if you require the user's full name to complete the order
            pub need_name: bool,
            /// Pass _True_, if you require the user's phone number to complete the order
            pub need_phone_number: bool,
            /// Pass _True_, if you require the user's email address to complete the order
            pub need_email: bool,
            /// Pass _True_, if you require the user's shipping address to complete the order
            pub need_shipping_address: bool,
            /// Pass _True_, if user's phone number should be sent to provider
            pub send_phone_number_to_provider: bool,
            /// Pass _True_, if user's email address should be sent to provider
            pub send_email_to_provider: bool,
            /// Pass _True_, if the final price depends on the shipping method
            pub is_flexible: bool,
            /// Sends the message [silently]. Users will receive a notification with no sound.
            ///
            /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
            pub disable_notification: bool,
            /// Protects the contents of sent messages from forwarding and saving
            pub protect_content: bool,
            /// If the message is a reply, ID of the original message
            pub reply_to_message_id: i32,
            /// Pass _True_, if the message should be sent even if the specified replied-to message is not found
            pub allow_sending_without_reply: bool,
            /// A JSON-serialized object for an [inline keyboard]. If empty, one 'Pay `total price`' button will be shown. If not empty, the first button must be a Pay button.
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            pub reply_markup: InlineKeyboardMarkup,
        }
    }
}