// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';



/**
 * Settings for the skeletal model that are ratios.
 * These values range from 0 to 1.
 */
export class ModelRatios implements flatbuffers.IUnpackableObject<ModelRatiosT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):ModelRatios {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsModelRatios(bb:flatbuffers.ByteBuffer, obj?:ModelRatios):ModelRatios {
  return (obj || new ModelRatios()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsModelRatios(bb:flatbuffers.ByteBuffer, obj?:ModelRatios):ModelRatios {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new ModelRatios()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

imputeWaistFromChestHip():number|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : null;
}

imputeWaistFromChestLegs():number|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : null;
}

imputeHipFromChestLegs():number|null {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : null;
}

imputeHipFromWaistLegs():number|null {
  const offset = this.bb!.__offset(this.bb_pos, 10);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : null;
}

/**
 * Hip's yaw and roll is set to the average of legs when 1.0
 */
interpHipLegs():number|null {
  const offset = this.bb!.__offset(this.bb_pos, 12);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : null;
}

/**
 * Knee trackers' yaw and roll is set to the ankle's when 1.0
 */
interpKneeTrackerAnkle():number|null {
  const offset = this.bb!.__offset(this.bb_pos, 14);
  return offset ? this.bb!.readFloat32(this.bb_pos + offset) : null;
}

static startModelRatios(builder:flatbuffers.Builder) {
  builder.startObject(6);
}

static addImputeWaistFromChestHip(builder:flatbuffers.Builder, imputeWaistFromChestHip:number) {
  builder.addFieldFloat32(0, imputeWaistFromChestHip, 0);
}

static addImputeWaistFromChestLegs(builder:flatbuffers.Builder, imputeWaistFromChestLegs:number) {
  builder.addFieldFloat32(1, imputeWaistFromChestLegs, 0);
}

static addImputeHipFromChestLegs(builder:flatbuffers.Builder, imputeHipFromChestLegs:number) {
  builder.addFieldFloat32(2, imputeHipFromChestLegs, 0);
}

static addImputeHipFromWaistLegs(builder:flatbuffers.Builder, imputeHipFromWaistLegs:number) {
  builder.addFieldFloat32(3, imputeHipFromWaistLegs, 0);
}

static addInterpHipLegs(builder:flatbuffers.Builder, interpHipLegs:number) {
  builder.addFieldFloat32(4, interpHipLegs, 0);
}

static addInterpKneeTrackerAnkle(builder:flatbuffers.Builder, interpKneeTrackerAnkle:number) {
  builder.addFieldFloat32(5, interpKneeTrackerAnkle, 0);
}

static endModelRatios(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createModelRatios(builder:flatbuffers.Builder, imputeWaistFromChestHip:number|null, imputeWaistFromChestLegs:number|null, imputeHipFromChestLegs:number|null, imputeHipFromWaistLegs:number|null, interpHipLegs:number|null, interpKneeTrackerAnkle:number|null):flatbuffers.Offset {
  ModelRatios.startModelRatios(builder);
  if (imputeWaistFromChestHip !== null)
    ModelRatios.addImputeWaistFromChestHip(builder, imputeWaistFromChestHip);
  if (imputeWaistFromChestLegs !== null)
    ModelRatios.addImputeWaistFromChestLegs(builder, imputeWaistFromChestLegs);
  if (imputeHipFromChestLegs !== null)
    ModelRatios.addImputeHipFromChestLegs(builder, imputeHipFromChestLegs);
  if (imputeHipFromWaistLegs !== null)
    ModelRatios.addImputeHipFromWaistLegs(builder, imputeHipFromWaistLegs);
  if (interpHipLegs !== null)
    ModelRatios.addInterpHipLegs(builder, interpHipLegs);
  if (interpKneeTrackerAnkle !== null)
    ModelRatios.addInterpKneeTrackerAnkle(builder, interpKneeTrackerAnkle);
  return ModelRatios.endModelRatios(builder);
}

unpack(): ModelRatiosT {
  return new ModelRatiosT(
    this.imputeWaistFromChestHip(),
    this.imputeWaistFromChestLegs(),
    this.imputeHipFromChestLegs(),
    this.imputeHipFromWaistLegs(),
    this.interpHipLegs(),
    this.interpKneeTrackerAnkle()
  );
}


unpackTo(_o: ModelRatiosT): void {
  _o.imputeWaistFromChestHip = this.imputeWaistFromChestHip();
  _o.imputeWaistFromChestLegs = this.imputeWaistFromChestLegs();
  _o.imputeHipFromChestLegs = this.imputeHipFromChestLegs();
  _o.imputeHipFromWaistLegs = this.imputeHipFromWaistLegs();
  _o.interpHipLegs = this.interpHipLegs();
  _o.interpKneeTrackerAnkle = this.interpKneeTrackerAnkle();
}
}

export class ModelRatiosT implements flatbuffers.IGeneratedObject {
constructor(
  public imputeWaistFromChestHip: number|null = null,
  public imputeWaistFromChestLegs: number|null = null,
  public imputeHipFromChestLegs: number|null = null,
  public imputeHipFromWaistLegs: number|null = null,
  public interpHipLegs: number|null = null,
  public interpKneeTrackerAnkle: number|null = null
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  return ModelRatios.createModelRatios(builder,
    this.imputeWaistFromChestHip,
    this.imputeWaistFromChestLegs,
    this.imputeHipFromChestLegs,
    this.imputeHipFromWaistLegs,
    this.interpHipLegs,
    this.interpKneeTrackerAnkle
  );
}
}