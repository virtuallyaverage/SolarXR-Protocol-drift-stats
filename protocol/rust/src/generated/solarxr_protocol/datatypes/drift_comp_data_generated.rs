// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem;
use core::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum driftCompDataOffset {}
#[derive(Copy, Clone, PartialEq)]

/// Data packet containing data for each tracking reset.
/// difference_deg: list of error degrees calculated on a reset (CCW(+),C(-))
/// comp_deg: yaw degrees/s generated through drift compensation (CCW(+),C(-))
/// delta_times: a list of time (seconds) it took for the drift to develop
/// difference_quats: a list of the error quaternions calculated on a reset
/// comp_quats: yaw compensation/s quaternions generated through drift compensation
pub struct driftCompData<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for driftCompData<'a> {
  type Inner = driftCompData<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> driftCompData<'a> {
  pub const VT_DRIFT_COMP_ENABLE: flatbuffers::VOffsetT = 4;
  pub const VT_DIFFERENCE_QUATS: flatbuffers::VOffsetT = 6;
  pub const VT_COMPENSATION_QUATS: flatbuffers::VOffsetT = 8;
  pub const VT_DIFFERENCE_DEG: flatbuffers::VOffsetT = 10;
  pub const VT_COMPENSATION_DEG: flatbuffers::VOffsetT = 12;
  pub const VT_RESET_INTERVAL: flatbuffers::VOffsetT = 14;
  pub const VT_MAX_RESETS: flatbuffers::VOffsetT = 16;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    driftCompData { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args driftCompDataArgs<'args>
  ) -> flatbuffers::WIPOffset<driftCompData<'bldr>> {
    let mut builder = driftCompDataBuilder::new(_fbb);
    if let Some(x) = args.reset_interval { builder.add_reset_interval(x); }
    if let Some(x) = args.compensation_deg { builder.add_compensation_deg(x); }
    if let Some(x) = args.difference_deg { builder.add_difference_deg(x); }
    if let Some(x) = args.compensation_quats { builder.add_compensation_quats(x); }
    if let Some(x) = args.difference_quats { builder.add_difference_quats(x); }
    builder.add_max_resets(args.max_resets);
    builder.add_drift_comp_enable(args.drift_comp_enable);
    builder.finish()
  }


  #[inline]
  pub fn drift_comp_enable(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(driftCompData::VT_DRIFT_COMP_ENABLE, Some(false)).unwrap()}
  }
  #[inline]
  pub fn difference_quats(&self) -> Option<flatbuffers::Vector<'a, math::Quat>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, math::Quat>>>(driftCompData::VT_DIFFERENCE_QUATS, None)}
  }
  #[inline]
  pub fn compensation_quats(&self) -> Option<flatbuffers::Vector<'a, math::Quat>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, math::Quat>>>(driftCompData::VT_COMPENSATION_QUATS, None)}
  }
  #[inline]
  pub fn difference_deg(&self) -> Option<flatbuffers::Vector<'a, f32>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, f32>>>(driftCompData::VT_DIFFERENCE_DEG, None)}
  }
  #[inline]
  pub fn compensation_deg(&self) -> Option<flatbuffers::Vector<'a, f32>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, f32>>>(driftCompData::VT_COMPENSATION_DEG, None)}
  }
  #[inline]
  pub fn reset_interval(&self) -> Option<flatbuffers::Vector<'a, f32>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, f32>>>(driftCompData::VT_RESET_INTERVAL, None)}
  }
  #[inline]
  pub fn max_resets(&self) -> u8 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u8>(driftCompData::VT_MAX_RESETS, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for driftCompData<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<bool>("drift_comp_enable", Self::VT_DRIFT_COMP_ENABLE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, math::Quat>>>("difference_quats", Self::VT_DIFFERENCE_QUATS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, math::Quat>>>("compensation_quats", Self::VT_COMPENSATION_QUATS, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, f32>>>("difference_deg", Self::VT_DIFFERENCE_DEG, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, f32>>>("compensation_deg", Self::VT_COMPENSATION_DEG, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, f32>>>("reset_interval", Self::VT_RESET_INTERVAL, false)?
     .visit_field::<u8>("max_resets", Self::VT_MAX_RESETS, false)?
     .finish();
    Ok(())
  }
}
pub struct driftCompDataArgs<'a> {
    pub drift_comp_enable: bool,
    pub difference_quats: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, math::Quat>>>,
    pub compensation_quats: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, math::Quat>>>,
    pub difference_deg: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, f32>>>,
    pub compensation_deg: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, f32>>>,
    pub reset_interval: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, f32>>>,
    pub max_resets: u8,
}
impl<'a> Default for driftCompDataArgs<'a> {
  #[inline]
  fn default() -> Self {
    driftCompDataArgs {
      drift_comp_enable: false,
      difference_quats: None,
      compensation_quats: None,
      difference_deg: None,
      compensation_deg: None,
      reset_interval: None,
      max_resets: 0,
    }
  }
}

pub struct driftCompDataBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> driftCompDataBuilder<'a, 'b> {
  #[inline]
  pub fn add_drift_comp_enable(&mut self, drift_comp_enable: bool) {
    self.fbb_.push_slot::<bool>(driftCompData::VT_DRIFT_COMP_ENABLE, drift_comp_enable, false);
  }
  #[inline]
  pub fn add_difference_quats(&mut self, difference_quats: flatbuffers::WIPOffset<flatbuffers::Vector<'b , math::Quat>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(driftCompData::VT_DIFFERENCE_QUATS, difference_quats);
  }
  #[inline]
  pub fn add_compensation_quats(&mut self, compensation_quats: flatbuffers::WIPOffset<flatbuffers::Vector<'b , math::Quat>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(driftCompData::VT_COMPENSATION_QUATS, compensation_quats);
  }
  #[inline]
  pub fn add_difference_deg(&mut self, difference_deg: flatbuffers::WIPOffset<flatbuffers::Vector<'b , f32>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(driftCompData::VT_DIFFERENCE_DEG, difference_deg);
  }
  #[inline]
  pub fn add_compensation_deg(&mut self, compensation_deg: flatbuffers::WIPOffset<flatbuffers::Vector<'b , f32>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(driftCompData::VT_COMPENSATION_DEG, compensation_deg);
  }
  #[inline]
  pub fn add_reset_interval(&mut self, reset_interval: flatbuffers::WIPOffset<flatbuffers::Vector<'b , f32>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(driftCompData::VT_RESET_INTERVAL, reset_interval);
  }
  #[inline]
  pub fn add_max_resets(&mut self, max_resets: u8) {
    self.fbb_.push_slot::<u8>(driftCompData::VT_MAX_RESETS, max_resets, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> driftCompDataBuilder<'a, 'b> {
    let start = _fbb.start_table();
    driftCompDataBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<driftCompData<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for driftCompData<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("driftCompData");
      ds.field("drift_comp_enable", &self.drift_comp_enable());
      ds.field("difference_quats", &self.difference_quats());
      ds.field("compensation_quats", &self.compensation_quats());
      ds.field("difference_deg", &self.difference_deg());
      ds.field("compensation_deg", &self.compensation_deg());
      ds.field("reset_interval", &self.reset_interval());
      ds.field("max_resets", &self.max_resets());
      ds.finish()
  }
}
