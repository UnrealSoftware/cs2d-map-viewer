miniquad_add_plugin({
    register_plugin: function (importObject) {
        // Attach our function to the WASM environment imports
        importObject.env.get_query_string = function (ptr, max_len) {
            // Get the query string (e.g., "?foo=bar")
            const query = window.location.search;
            if (!query) return 0;

            // Encode the JS string into UTF-8 bytes
            const encoder = new TextEncoder();
            const encoded = encoder.encode(query);
            const len = Math.min(encoded.length, max_len);

            // Write the bytes directly into WASM memory
            // 'wasm_exports' is provided globally by the miniquad JS loader
            const memory = new Uint8Array(wasm_exports.memory.buffer);
            memory.set(encoded.subarray(0, len), ptr);

            return len; // Tell Rust how many bytes we wrote
        };
    },
    version: "1.0",
    name: "query_string_plugin",
});