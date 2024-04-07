// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.datatypes;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

public class driftCompDataT {
  private boolean driftCompEnable;
  private solarxr_protocol.datatypes.math.QuatT[] differenceQuats;
  private solarxr_protocol.datatypes.math.QuatT[] compensationQuats;
  private float[] differenceDeg;
  private float[] compensationDeg;
  private float[] resetInterval;
  private int maxResets;

  public boolean getDriftCompEnable() { return driftCompEnable; }

  public void setDriftCompEnable(boolean driftCompEnable) { this.driftCompEnable = driftCompEnable; }

  public solarxr_protocol.datatypes.math.QuatT[] getDifferenceQuats() { return differenceQuats; }

  public void setDifferenceQuats(solarxr_protocol.datatypes.math.QuatT[] differenceQuats) { this.differenceQuats = differenceQuats; }

  public solarxr_protocol.datatypes.math.QuatT[] getCompensationQuats() { return compensationQuats; }

  public void setCompensationQuats(solarxr_protocol.datatypes.math.QuatT[] compensationQuats) { this.compensationQuats = compensationQuats; }

  public float[] getDifferenceDeg() { return differenceDeg; }

  public void setDifferenceDeg(float[] differenceDeg) { this.differenceDeg = differenceDeg; }

  public float[] getCompensationDeg() { return compensationDeg; }

  public void setCompensationDeg(float[] compensationDeg) { this.compensationDeg = compensationDeg; }

  public float[] getResetInterval() { return resetInterval; }

  public void setResetInterval(float[] resetInterval) { this.resetInterval = resetInterval; }

  public int getMaxResets() { return maxResets; }

  public void setMaxResets(int maxResets) { this.maxResets = maxResets; }


  public driftCompDataT() {
    this.driftCompEnable = false;
    this.differenceQuats = null;
    this.compensationQuats = null;
    this.differenceDeg = null;
    this.compensationDeg = null;
    this.resetInterval = null;
    this.maxResets = 0;
  }
}

