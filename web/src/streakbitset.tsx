import { BitSet } from "./bitset";
import dayjs, { Dayjs } from "dayjs";

export class StreakBitSet extends BitSet {
    streak(startTime: Dayjs) {
        const offset = dayjs().diff(startTime, "days");
        let u8_offset = offset >> 3;
        let bit_offset = 7 - offset & 7;
        let streak = 0;
        while((this._buffer[u8_offset] >> bit_offset & 1) && (u8_offset >= 0)) {
            bit_offset++;
            streak++;
            if(bit_offset % 7 == 0) {
                bit_offset = 0;
                u8_offset--;
            }
        }
        return streak;
    }

    toJSON(key: string) {
        return this;
    }
}