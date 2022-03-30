class BitSet {
    _buffer: Uint32Array;
    _bit_capcity: number;

    constructor(len: number) {
        this._bit_capcity = len;
        this._buffer = new Uint32Array((len >> 5) + ((len & 31) == 0 ? 0 : 1));
    }   

    get(idx: number) {
        return this._buffer[idx >> 5] >> (31 - (idx & 31)) & 1;
    }   

    set(idx: number) {
        this._buffer[idx >> 5] |= (1 << (31 - (idx & 31)));
    }   

    clear(idx: number) {
        this._buffer[idx >> 5] &= ~(1 << (31 - (idx & 31)));
    }   

    toJSON() {
        let bytes: string[] = []; 
        this._buffer.forEach(u32 => {
            for(let i = 0; i < 4; i++) {
                bytes.push((u32 >> (24 - i * 8) & 0xff).toString());
            }   
        })  
        return "[" + bytes.join(",") + "]";
    }

    // TODO: calculate streak
    streak() {
        return this._bit_capcity;
    }
}

export {
    BitSet,
}