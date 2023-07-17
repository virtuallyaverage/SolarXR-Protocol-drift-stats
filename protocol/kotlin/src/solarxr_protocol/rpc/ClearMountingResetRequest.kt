// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc

import java.nio.*
import kotlin.math.sign
import com.google.flatbuffers.*

/**
 * Clears mounting reset data, defaulting to the manually set mounting orientations
 */
@Suppress("unused")
class ClearMountingResetRequest : Table() {

    fun __init(_i: Int, _bb: ByteBuffer)  {
        __reset(_i, _bb)
    }
    fun __assign(_i: Int, _bb: ByteBuffer) : ClearMountingResetRequest {
        __init(_i, _bb)
        return this
    }
    companion object {
        @JvmStatic
        fun validateVersion() = Constants.FLATBUFFERS_22_10_26()
        @JvmStatic
        fun getRootAsClearMountingResetRequest(_bb: ByteBuffer): ClearMountingResetRequest = getRootAsClearMountingResetRequest(_bb, ClearMountingResetRequest())
        @JvmStatic
        fun getRootAsClearMountingResetRequest(_bb: ByteBuffer, obj: ClearMountingResetRequest): ClearMountingResetRequest {
            _bb.order(ByteOrder.LITTLE_ENDIAN)
            return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb))
        }
        @JvmStatic
        fun startClearMountingResetRequest(builder: FlatBufferBuilder) = builder.startTable(0)
        @JvmStatic
        fun endClearMountingResetRequest(builder: FlatBufferBuilder) : Int {
            val o = builder.endTable()
            return o
        }
    }
}