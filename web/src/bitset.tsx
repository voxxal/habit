export class BitSet {
    _buffer: Uint8Array;
    _bit_capacity: number;

    constructor(len: number, buffer?: Uint8Array) {
        this._bit_capacity = len;
        this._buffer = buffer ?? new Uint8Array((len >> 3) + ((len & 7) == 0 ? 0 : 1));
    }

    get(idx: number) {
        return this._buffer[idx >> 3] >> (7 - (idx & 7)) & 1;
    }   

    set(idx: number) {
        this._buffer[idx >> 3] |= (1 << (7 - (idx & 7)));
    }   

    clear(idx: number) {
        this._buffer[idx >> 3] &= ~(1 << (7 - (idx & 7)));
    }
}