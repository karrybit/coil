import * as wasm from './index_bg.wasm';

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function _assertBoolean(n) {
    if (typeof(n) !== 'boolean') {
        throw new Error('expected a boolean argument');
    }
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? require('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');

    heap[idx] = obj;
    return idx;
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error('expected a number argument');
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? require('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (typeof(arg) !== 'string') throw new Error('expected a string argument');

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) wasm.__wbindgen_export_2.get(dtor)(a, state.b);
            else state.a = a;
        }
    };
    real.original = state;
    return real;
}

function logError(e) {
    let error = (function () {
        try {
            return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
        } catch(_) {
            return "<failed to stringify thrown value>";
        }
    }());
    console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
    throw e;
}
function __wbg_adapter_22(arg0, arg1) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hffe9ffea6aca9165(arg0, arg1);
}

function __wbg_adapter_25(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h740375be5a32e048(arg0, arg1, addHeapObject(arg2));
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1);
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function handleError(e) {
    wasm.__wbindgen_exn_store(addHeapObject(e));
}
function __wbg_adapter_109(arg0, arg1, arg2, arg3) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm.wasm_bindgen__convert__closures__invoke2_mut__hbc73de5fc6151158(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}

/**
*/
export class Pager {

    constructor() {
        throw new Error('cannot invoke `new` directly');
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_pager_free(ptr);
    }
    /**
    * @param {Uint8Array} before_image
    * @param {Uint8Array} after_image
    */
    static initialize(before_image, after_image) {
        var ptr0 = passArray8ToWasm0(before_image, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = passArray8ToWasm0(after_image, wasm.__wbindgen_malloc);
        var len1 = WASM_VECTOR_LEN;
        wasm.pager_initialize(ptr0, len0, ptr1, len1);
    }
    /**
    * @param {number} interval
    * @returns {any}
    */
    static up(interval) {
        _assertNum(interval);
        var ret = wasm.pager_up(interval);
        return takeObject(ret);
    }
    /**
    * @param {number} interval
    * @returns {any}
    */
    static right(interval) {
        _assertNum(interval);
        var ret = wasm.pager_right(interval);
        return takeObject(ret);
    }
    /**
    * @param {number} interval
    * @returns {any}
    */
    static down(interval) {
        _assertNum(interval);
        var ret = wasm.pager_down(interval);
        return takeObject(ret);
    }
    /**
    * @param {number} interval
    * @returns {any}
    */
    static left(interval) {
        _assertNum(interval);
        var ret = wasm.pager_left(interval);
        return takeObject(ret);
    }
}

export const __wbindgen_cb_drop = function(arg0) {
    const obj = takeObject(arg0).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    var ret = false;
    _assertBoolean(ret);
    return ret;
};

export const __wbindgen_string_new = function(arg0, arg1) {
    var ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

export const __wbg_instanceof_Window_a633dbe0900c728a = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof Window;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_document_07444f1bbea314bb = function(arg0) {
    try {
        var ret = getObject(arg0).document;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_requestAnimationFrame_10a415a97fc2123f = function(arg0, arg1) {
    try {
        try {
            var ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
            _assertNum(ret);
            return ret;
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_getElementById_633c94a971ae0eb9 = function(arg0, arg1, arg2) {
    try {
        var ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_instanceof_HtmlCanvasElement_c6a06fc9a851a478 = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof HTMLCanvasElement;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_width_e29d6e8a5c409d12 = function(arg0) {
    try {
        var ret = getObject(arg0).width;
        _assertNum(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_height_f1097727b2ec35e1 = function(arg0) {
    try {
        var ret = getObject(arg0).height;
        _assertNum(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_getContext_2151b76e11a6eb39 = function(arg0, arg1, arg2) {
    try {
        try {
            var ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbindgen_object_clone_ref = function(arg0) {
    var ret = getObject(arg0);
    return addHeapObject(ret);
};

export const __wbg_instanceof_WebGlRenderingContext_3aadcbc31d1748d3 = function(arg0) {
    try {
        var ret = getObject(arg0) instanceof WebGLRenderingContext;
        _assertBoolean(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_bufferData_985a5ff391474177 = function(arg0, arg1, arg2, arg3) {
    try {
        getObject(arg0).bufferData(arg1 >>> 0, getObject(arg2), arg3 >>> 0);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_texImage2D_d058822cc7d49b43 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    try {
        try {
            getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, getObject(arg9));
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_attachShader_9564db836e3d4ece = function(arg0, arg1, arg2) {
    try {
        getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
    } catch (e) {
        logError(e)
    }
};

export const __wbg_bindBuffer_6cc973b0a3488535 = function(arg0, arg1, arg2) {
    try {
        getObject(arg0).bindBuffer(arg1 >>> 0, getObject(arg2));
    } catch (e) {
        logError(e)
    }
};

export const __wbg_bindTexture_812a67a84575f09d = function(arg0, arg1, arg2) {
    try {
        getObject(arg0).bindTexture(arg1 >>> 0, getObject(arg2));
    } catch (e) {
        logError(e)
    }
};

export const __wbg_compileShader_91ce1c5df480321c = function(arg0, arg1) {
    try {
        getObject(arg0).compileShader(getObject(arg1));
    } catch (e) {
        logError(e)
    }
};

export const __wbg_createBuffer_1b29c13abf687b68 = function(arg0) {
    try {
        var ret = getObject(arg0).createBuffer();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_createProgram_0bbeea9ffc5daa63 = function(arg0) {
    try {
        var ret = getObject(arg0).createProgram();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_createShader_cdd9f1769cd1de1e = function(arg0, arg1) {
    try {
        var ret = getObject(arg0).createShader(arg1 >>> 0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_createTexture_7fc81a3938b40da8 = function(arg0) {
    try {
        var ret = getObject(arg0).createTexture();
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_drawArrays_dfc8bd56cfd2c50e = function(arg0, arg1, arg2, arg3) {
    try {
        getObject(arg0).drawArrays(arg1 >>> 0, arg2, arg3);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_enableVertexAttribArray_1b8360d81db7d6f0 = function(arg0, arg1) {
    try {
        getObject(arg0).enableVertexAttribArray(arg1 >>> 0);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_getAttribLocation_ce1df105f2722b0b = function(arg0, arg1, arg2, arg3) {
    try {
        var ret = getObject(arg0).getAttribLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
        _assertNum(ret);
        return ret;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_getProgramInfoLog_2a1da5b17664faa9 = function(arg0, arg1, arg2) {
    try {
        var ret = getObject(arg1).getProgramInfoLog(getObject(arg2));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_getProgramParameter_ff1b7fa34d0991f5 = function(arg0, arg1, arg2) {
    try {
        var ret = getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0);
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_getShaderInfoLog_5a8842f27648dd20 = function(arg0, arg1, arg2) {
    try {
        var ret = getObject(arg1).getShaderInfoLog(getObject(arg2));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    } catch (e) {
        logError(e)
    }
};

export const __wbg_getShaderParameter_9fe8d76217a4969c = function(arg0, arg1, arg2) {
    try {
        var ret = getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0);
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_getUniformLocation_bcdd3b3a38c50a03 = function(arg0, arg1, arg2, arg3) {
    try {
        var ret = getObject(arg0).getUniformLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_linkProgram_cba038b57a3871ef = function(arg0, arg1) {
    try {
        getObject(arg0).linkProgram(getObject(arg1));
    } catch (e) {
        logError(e)
    }
};

export const __wbg_pixelStorei_219f6dc606402fc4 = function(arg0, arg1, arg2) {
    try {
        getObject(arg0).pixelStorei(arg1 >>> 0, arg2);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_shaderSource_57dcf3bb9d5a2045 = function(arg0, arg1, arg2, arg3) {
    try {
        getObject(arg0).shaderSource(getObject(arg1), getStringFromWasm0(arg2, arg3));
    } catch (e) {
        logError(e)
    }
};

export const __wbg_texParameteri_0538bb1eb7de4f3b = function(arg0, arg1, arg2, arg3) {
    try {
        getObject(arg0).texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_uniform2f_c1a2fa4599b15748 = function(arg0, arg1, arg2, arg3) {
    try {
        getObject(arg0).uniform2f(getObject(arg1), arg2, arg3);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_useProgram_324a22a196d1f113 = function(arg0, arg1) {
    try {
        getObject(arg0).useProgram(getObject(arg1));
    } catch (e) {
        logError(e)
    }
};

export const __wbg_vertexAttribPointer_2f730a4ef1717caf = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    try {
        getObject(arg0).vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_viewport_e581bdce9dbf078f = function(arg0, arg1, arg2, arg3, arg4) {
    try {
        getObject(arg0).viewport(arg1, arg2, arg3, arg4);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_textContent_1fb8e2642c9c164e = function(arg0, arg1, arg2) {
    try {
        getObject(arg0).textContent = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_newnoargs_ebdc90c3d1e4e55d = function(arg0, arg1) {
    try {
        var ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_call_804d3ad7e8acd4d5 = function(arg0, arg1) {
    try {
        try {
            var ret = getObject(arg0).call(getObject(arg1));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_call_1ad0eb4a7ab279eb = function(arg0, arg1, arg2) {
    try {
        try {
            var ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_new_1bf1b0dbcaa9ee96 = function(arg0, arg1) {
    try {
        try {
            var state0 = {a: arg0, b: arg1};
            var cb0 = (arg0, arg1) => {
                const a = state0.a;
                state0.a = 0;
                try {
                    return __wbg_adapter_109(a, state0.b, arg0, arg1);
                } finally {
                    state0.a = a;
                }
            };
            var ret = new Promise(cb0);
            return addHeapObject(ret);
        } finally {
            state0.a = state0.b = 0;
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_resolve_3e5970e9c931a3c2 = function(arg0) {
    try {
        var ret = Promise.resolve(getObject(arg0));
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_then_d797310661d9e275 = function(arg0, arg1) {
    try {
        var ret = getObject(arg0).then(getObject(arg1));
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_globalThis_48a5e9494e623f26 = function() {
    try {
        try {
            var ret = globalThis.globalThis;
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_self_25067cb019cade42 = function() {
    try {
        try {
            var ret = self.self;
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_window_9e80200b35aa30f8 = function() {
    try {
        try {
            var ret = window.window;
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_global_7583a634265a91fc = function() {
    try {
        try {
            var ret = global.global;
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    } catch (e) {
        logError(e)
    }
};

export const __wbg_newwithbyteoffsetandlength_284676320876299d = function(arg0, arg1, arg2) {
    try {
        var ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbg_newwithbyteoffsetandlength_7ccfa06426575282 = function(arg0, arg1, arg2) {
    try {
        var ret = new Float32Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbindgen_is_undefined = function(arg0) {
    var ret = getObject(arg0) === undefined;
    _assertBoolean(ret);
    return ret;
};

export const __wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
};

export const __wbg_buffer_f897a8d316863411 = function(arg0) {
    try {
        var ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbindgen_boolean_get = function(arg0) {
    const v = getObject(arg0);
    var ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
    _assertNum(ret);
    return ret;
};

export const __wbindgen_debug_string = function(arg0, arg1) {
    var ret = debugString(getObject(arg1));
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export const __wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export const __wbindgen_rethrow = function(arg0) {
    throw takeObject(arg0);
};

export const __wbindgen_memory = function() {
    var ret = wasm.memory;
    return addHeapObject(ret);
};

export const __wbindgen_closure_wrapper138 = function(arg0, arg1, arg2) {
    try {
        var ret = makeMutClosure(arg0, arg1, 32, __wbg_adapter_22);
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

export const __wbindgen_closure_wrapper11250 = function(arg0, arg1, arg2) {
    try {
        var ret = makeMutClosure(arg0, arg1, 461, __wbg_adapter_25);
        return addHeapObject(ret);
    } catch (e) {
        logError(e)
    }
};

