use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::Type;

/// A wrapper to serialize `T: Type + Serialize` as a value.
///
/// When the type of a value is well-known, you may avoid the cost and complexity of wrapping to a
/// generic [`Value`] and instead use this wrapper.
///
/// ```
/// # use zvariant::{to_bytes, serialized::Context, SerializeValue, LE};
/// #
/// # let ctxt = Context::new_dbus(LE, 0);
/// let _ = to_bytes(ctxt, &SerializeValue(&[0, 1, 2])).unwrap();
/// ```
///
/// [`Value`]: enum.Value.html
pub struct SerializeValue<'a, T: Type + Serialize>(pub &'a T);

impl<T: Type + Serialize> Serialize for SerializeValue<'_, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serializer implementation needs to ensure padding isn't added for Value.
        let mut structure = serializer.serialize_struct("Variant", 2)?;

        structure.serialize_field("signature", T::SIGNATURE)?;
        structure.serialize_field("value", self.0)?;

        structure.end()
    }
}

impl<T: Type + Serialize> Type for SerializeValue<'_, T> {
    const SIGNATURE: &'static crate::Signature = &crate::Signature::Variant;
}
