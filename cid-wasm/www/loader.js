/**
 * CID-WASM Raw Loader
 * 
 * This module provides a way to load the CID WASM module without wasm-bindgen.
 * It uses the raw WebAssembly API to load and interact with the WASM module.
 * 
 * Note: For production use, it's recommended to use wasm-bindgen or wasm-pack
 * to generate proper JS bindings. This loader is for development/testing.
 */

let wasmInstance = null;
let memory = null;
let initialized = false;

/**
 * Initialize the CID WASM module
 * @param {string} wasmPath - Path to the WASM file
 * @returns {Promise<void>}
 */
export async function init(wasmPath = './target/wasm32-unknown-unknown/release/cid_wasm.wasm') {
    if (initialized) return;
    
    try {
        // Fetch the WASM file
        const response = await fetch(wasmPath);
        const bytes = await response.arrayBuffer();
        
        // Create import object
        const importObject = {
            env: {
                memory: new WebAssembly.Memory({ initial: 256, maximum: 512 }),
            },
            wbg: {
                __wbindgen_placeholder__: () => {},
            }
        };
        
        // Instantiate the WASM module
        const { instance } = await WebAssembly.instantiate(bytes, importObject);
        
        wasmInstance = instance;
        memory = instance.exports.memory;
        
        // Try to call init if available
        if (instance.exports.init) {
            instance.exports.init();
        }
        
        initialized = true;
        console.log('CID WASM module loaded successfully');
        
        // Log available exports
        console.log('Available exports:', Object.keys(instance.exports));
        
    } catch (error) {
        console.error('Failed to load CID WASM:', error);
        throw error;
    }
}

/**
 * Check if the module is initialized
 * @returns {boolean}
 */
export function isInitialized() {
    return initialized;
}

/**
 * Get the raw WASM instance (for advanced use)
 * @returns {WebAssembly.Instance|null}
 */
export function getInstance() {
    return wasmInstance;
}

/**
 * Get the memory buffer
 * @returns {ArrayBuffer|null}
 */
export function getMemory() {
    return memory ? memory.buffer : null;
}

// Export functions for the demo
export default {
    init,
    isInitialized,
    getInstance,
    getMemory
};
