use std::hash::Hasher;

#[inline]
pub(crate) fn hash_vec2(hasher: &mut impl Hasher, vec: glam::Vec2) {
  hasher.write_u32(vec.x.to_bits());
  hasher.write_u32(vec.y.to_bits());
}

#[inline]
pub(crate) fn hash_vec3(hasher: &mut impl Hasher, vec: glam::Vec3) {
  hasher.write_u32(vec.x.to_bits());
  hasher.write_u32(vec.y.to_bits());
  hasher.write_u32(vec.z.to_bits());
}

#[inline]
pub(crate) fn hash_vec4(hasher: &mut impl Hasher, vec: glam::Vec4) {
  hasher.write_u32(vec.x.to_bits());
  hasher.write_u32(vec.y.to_bits());
  hasher.write_u32(vec.z.to_bits());
  hasher.write_u32(vec.w.to_bits());
}