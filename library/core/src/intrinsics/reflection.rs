//! Intrinsics for compile time reflection
//!
//! This are all methods operating on [`TypeId`](core::any::TypeId) which is the
//! entry point to all compile time reflection.

/// Compute the type information of a concrete type.
/// It can only be called at compile time, the backends do
/// not implement it.
#[rustc_intrinsic]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_comptime]
pub fn type_id_type_of(_id: crate::any::TypeId) -> crate::mem::type_info::Type;

#[rustc_intrinsic]
#[rustc_comptime]
#[unstable(feature = "core_intrinsics", issue = "none")]
/// Check if a type represented by a `TypeId` implements a trait represented by a `TypeId`.
/// It can only be called at compile time, the backends do
/// not implement it. If it implements the trait the dyn metadata gets returned for vtable access.
pub fn type_id_vtable(
    _id: crate::any::TypeId,
    _trait: crate::any::TypeId,
) -> Option<ptr::DynMetadata<*const ()>>;

/// Gets a static string slice containing the name of a type.
///
/// Note that, unlike most intrinsics, this can only be called at compile-time
/// as backends do not have an implementation for it. The only caller (its
/// stable counterpart) wraps this intrinsic call in a `const` block so that
/// backends only see an evaluated constant.
///
/// The stabilized version of this intrinsic is [`core::any::type_name`].
#[rustc_nounwind]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_intrinsic]
#[rustc_comptime]
pub fn type_id_type_name<T: ?Sized>() -> &'static str;

/// Returns whether the type represented by this `TypeId` is a signed integer.
///
/// The more user-friendly version of this intrinsic is [`core::any::TypeId::is_signed`].
#[rustc_intrinsic]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_comptime]
pub fn type_id_is_signed(_id: crate::any::TypeId) -> bool;

/// Gets the length of the array represented by this `TypeId`.
///
/// The more user-friendly version of this intrinsic is [`core::any::TypeId::array_len`].
#[rustc_intrinsic]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_comptime]
pub fn type_id_array_len(_id: crate::any::TypeId) -> usize;

/// Gets the type of each element of the array or slice represented by this `TypeId`.
///
/// The more user-friendly version of this intrinsic is [`core::any::TypeId::element_ty`].
#[rustc_intrinsic]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_comptime]
pub fn type_id_element_ty(_id: crate::any::TypeId) -> Option<crate::any::TypeId>;

/// Gets the size of the type represented by this `TypeId`.
///
/// The more user-friendly version of this intrinsic is [`core::any::TypeId::size`].
#[rustc_intrinsic]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_comptime]
pub fn type_id_size_of(_id: crate::any::TypeId) -> Option<usize>;

/// Gets the number of variants of the type represented by this `TypeId`.
///
/// The more user-friendly version of this intrinsic is [`core::any::TypeId::variants`].
#[rustc_intrinsic]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_comptime]
pub fn type_id_variants(_id: crate::any::TypeId) -> usize;

/// Gets the name of the variant represented by the base `TypeId` and variant_idx.
///
/// The more user-friendly version of this intrinsic is [`core::mem::type_info::VariantId::name`].
///
/// [`TypeId`]: crate::any::TypeId
#[rustc_intrinsic]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_comptime]
pub fn type_id_variant_name(_base: crate::any::TypeId, _variant_index: usize) -> &'static str;

/// Returns true when the variant represented by the base `TypeId` and variant_idx is non
/// exhaustive.
///
/// The more user-friendly version of this intrinsic is
/// [`core::mem::type_info::VariantId::non_exhaustive`].
///
/// [`TypeId`]: crate::any::TypeId
#[rustc_intrinsic]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_comptime]
pub fn variant_non_exhaustive(base: crate::any::TypeId, variant: usize) -> bool;

/// Gets the number of fields at the given `variant_index` represented by this `TypeId`.
///
/// The more user-friendly version of this intrinsic is [`core::any::TypeId::fields`].
#[rustc_intrinsic]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_comptime]
pub fn type_id_fields(_id: crate::any::TypeId, _variant_index: usize) -> usize;

/// Gets the [`FieldRepresentingType`]'s `TypeId` at the given index of the type represented by this `TypeId`.
///
/// The more user-friendly version of this intrinsic is [`core::any::TypeId::field`].
///
/// [`FieldRepresentingType`]: crate::field::FieldRepresentingType
#[rustc_intrinsic]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_comptime]
pub fn type_id_field_representing_type(
    _id: crate::any::TypeId,
    _variant_index: usize,
    _field_index: usize,
) -> crate::any::TypeId;

/// Gets the actual field `TypeId` of the [`FieldRepresentingType`]'s `TypeId`.
///
/// The more user-friendly version of this intrinsic is [`core::mem::type_info::FieldId::type_id`].
///
/// [`FieldRepresentingType`]: crate::field::FieldRepresentingType
#[rustc_intrinsic]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_comptime]
pub fn type_id_field_type(_frt_type_id: crate::any::TypeId) -> crate::any::TypeId;

/// Gets the name of the field represented by the [`FieldRepresentingType`]'s `TypeId`.
///
/// The more user-friendly version of this intrinsic is [`core::mem::type_info::FieldId::name`].
///
/// [`FieldRepresentingType`]: crate::field::FieldRepresentingType
#[rustc_intrinsic]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_comptime]
pub fn type_id_field_name(_frt_type_id: crate::any::TypeId) -> &'static str;

/// Gets the name of the field represented by the [`FieldRepresentingType`]'s `TypeId`.
///
/// The more user-friendly version of this intrinsic is [`core::mem::type_info::FieldId::name`].
///
/// [`FieldRepresentingType`]: crate::field::FieldRepresentingType
#[rustc_intrinsic]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_comptime]
pub fn type_id_field_offset(_frt_type_id: crate::any::TypeId) -> usize;

/// Given a `TypeId` that represents a function pointer returns an [`core::mem::type_info::FnPtr`].
/// When called on something else this returns `None`.
///
/// The more user-friendly version of this intrinsic is [`core::any::TypeId::function_ptr`].
#[rustc_intrinsic]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_comptime]
pub fn type_id_function_ptr(_type_id: crate::any::TypeId) -> Option<crate::mem::type_info::FnPtr>;

/// Checks whether this type is non-exhaustive.
#[rustc_intrinsic]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_comptime]
pub fn type_id_non_exhaustive(_id: crate::any::TypeId) -> bool;

/// Returns the list of generic args on this type.
/// Only meaningful for Adts, closures, ... Everything else returns an empty slice.
#[rustc_intrinsic]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_comptime]
pub fn type_id_generics(_id: crate::any::TypeId) -> &'static [crate::mem::type_info::Generic];

/// Given a `TypeId` that represents a pointer this returns the `TypeId` which that pointer
/// points to. When called on anything else this returns None.
///
/// The more user-friendly version of this intrinsic is [`core::any::TypeId::points_to`].
#[rustc_intrinsic]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_comptime]
pub fn type_id_points_to(_id: crate::any::TypeId) -> Option<crate::any::TypeId>;

/// Given a `TypeId` that represents a pointer returns whether that pointer is mutable.
/// When called on anything else this returns `false`.
///
/// The more user-friendly version of this intrinsic is [`core::any::TypeId::points_mutably`].
#[rustc_intrinsic]
#[unstable(feature = "core_intrinsics", issue = "none")]
#[rustc_comptime]
pub fn type_id_points_mutably(_id: crate::any::TypeId) -> bool;
