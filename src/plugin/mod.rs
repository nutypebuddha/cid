//! WASM plugin system for CID.
//!
//! Provides sandboxed plugin loading via WebAssembly, allowing
//! third-party validation gates to run safely inside CID.
//!
//! # Feature
//! This module requires the `plugins` feature (enabled by default
//! when `wasmtime` is available).

use std::collections::HashMap;

/// Result returned by a plugin validation.
#[derive(Debug, Clone)]
pub struct PluginResult {
    /// Whether the input passed validation.
    pub passed: bool,
    /// Confidence score (0.0 - 1.0).
    pub score: f64,
    /// Human-readable message.
    pub message: String,
}

impl PluginResult {
    pub fn pass(score: f64, message: &str) -> Self {
        PluginResult { passed: true, score, message: message.to_string() }
    }
    pub fn fail(score: f64, message: &str) -> Self {
        PluginResult { passed: false, score, message: message.to_string() }
    }
}

/// Plugin type classification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PluginType {
    Math,
    Logic,
    Fact,
    Confidence,
    Custom(String),
}

impl PluginType {
    pub fn as_str(&self) -> &str {
        match self {
            PluginType::Math => "math",
            PluginType::Logic => "logic",
            PluginType::Fact => "fact",
            PluginType::Confidence => "confidence",
            PluginType::Custom(s) => s.as_str(),
        }
    }
}

/// Metadata describing a loaded plugin.
#[derive(Debug, Clone)]
pub struct PluginMeta {
    pub name: String,
    pub version: String,
    pub plugin_type: PluginType,
    pub author: String,
    pub description: String,
}

/// A loaded WASM plugin instance.
///
/// Each plugin runs in its own Wasmtime instance with
/// sandboxed linear memory and limited host function access.
pub struct WasmPlugin {
    meta: PluginMeta,
    #[cfg(feature = "plugins")]
    instance: wasmtime::Instance,
    #[cfg(feature = "plugins")]
    store: wasmtime::Store<PluginHostState>,
}

#[cfg(feature = "plugins")]
struct PluginHostState {
    call_count: u64,
    last_result: PluginResult,
}

impl WasmPlugin {
    /// Get plugin metadata (always available).
    pub fn meta(&self) -> &PluginMeta {
        &self.meta
    }
}

#[cfg(feature = "plugins")]
impl WasmPlugin {
    /// Create a new WasmPlugin from compiled wasm bytes.
    pub fn new(
        meta: PluginMeta,
        wasm_bytes: &[u8],
        engine: &wasmtime::Engine,
        linker: &wasmtime::Linker<PluginHostState>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let module = wasmtime::Module::new(engine, wasm_bytes)?;
        let store = wasmtime::Store::new(engine, PluginHostState {
            call_count: 0,
            last_result: PluginResult::pass(1.0, "initialized"),
        });
        let instance = linker.instantiate(&store, &module)?;
        Ok(WasmPlugin { meta, instance, store })
    }

    /// Validate an input string through this plugin.
    pub fn validate(&mut self, input: &str, context: &str) -> Result<PluginResult, Box<dyn std::error::Error>> {
        let validate_func = self.instance.get_typed_func::<(wasmtime::Memory, i32, i32, i32, i32), i32>(
            &self.store, "validate",
        )?;

        self.store.data_mut().call_count += 1;

        let passed = validate_func.call(&mut self.store, (
            self.instance.get_memory(&mut self.store, "memory").unwrap(),
            0,
            input.len() as i32,
            0,
            context.len() as i32,
        ))? != 0;

        let result = if passed {
            PluginResult::pass(0.9, "plugin validated")
        } else {
            PluginResult::fail(0.3, "plugin rejected")
        };
        self.store.data_mut().last_result = result.clone();
        Ok(result)
    }
}

#[cfg(not(feature = "plugins"))]
impl WasmPlugin {
    /// Stub: plugins feature not enabled.
    pub fn validate(&mut self, _input: &str, _context: &str) -> Result<PluginResult, Box<dyn std::error::Error>> {
        Err("plugins feature not enabled".into())
    }
}

/// Loader that manages WASM plugins.
pub struct PluginLoader {
    #[cfg(feature = "plugins")]
    engine: wasmtime::Engine,
    #[cfg(feature = "plugins")]
    linker: wasmtime::Linker<PluginHostState>,
    plugins: HashMap<String, WasmPlugin>,
}

impl PluginLoader {
    /// Create a new plugin loader.
    pub fn new() -> Self {
        #[cfg(feature = "plugins")]
        {
            let mut config = wasmtime::Config::new();
            config.wasm_multi_value(true);
            config.wasm_memory64(false);
            let engine = wasmtime::Engine::new(&config).unwrap();
            let mut linker = wasmtime::Linker::<PluginHostState>::new(&engine);

            // Register host functions available to plugins
            linker.func_wrap("cid", "log", |caller: wasmtime::Caller<'_, PluginHostState>, ptr: i32, len: i32| {
                // Plugin requested a log — we capture the call for observability
                eprintln!("[plugin:log] {} bytes at ptr {}", len, ptr);
            }).unwrap();

            PluginLoader {
                engine,
                linker,
                plugins: HashMap::new(),
            }
        }
        #[cfg(not(feature = "plugins"))]
        {
            PluginLoader {
                plugins: HashMap::new(),
            }
        }
    }

    /// Load a plugin from raw WASM bytes.
    pub fn load(&mut self, _meta: PluginMeta, _wasm_bytes: &[u8]) -> Result<(), String> {
        #[cfg(feature = "plugins")]
        {
            let plugin = WasmPlugin::new(_meta.clone(), _wasm_bytes, &self.engine, &self.linker)
                .map_err(|e| format!("failed to load plugin '{}': {}", _meta.name, e))?;
            self.plugins.insert(_meta.name.clone(), plugin);
            Ok(())
        }
        #[cfg(not(feature = "plugins"))]
        {
            Err("plugins feature not enabled (recompile with --features plugins)".to_string())
        }
    }

    /// Load a plugin from a `.wasm` file on disk.
    pub fn load_file(&mut self, meta: PluginMeta, path: &str) -> Result<(), String> {
        let bytes = std::fs::read(path)
            .map_err(|e| format!("cannot read '{}': {}", path, e))?;
        self.load(meta, &bytes)
    }

    /// Get a loaded plugin by name.
    pub fn get(&self, name: &str) -> Option<&WasmPlugin> {
        self.plugins.get(name)
    }

    /// Get a loaded plugin by name (mutable).
    pub fn get_mut(&mut self, name: &str) -> Option<&mut WasmPlugin> {
        self.plugins.get_mut(name)
    }

    /// List all loaded plugin names.
    pub fn list(&self) -> Vec<&str> {
        self.plugins.keys().map(|s| s.as_str()).collect()
    }

    /// Number of loaded plugins.
    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    /// Check if any plugins are loaded.
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }
}

impl Default for PluginLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Plugin validation gate — wraps a loaded WASM plugin as a CID gate.
pub struct PluginGate {
    loader: PluginLoader,
    plugin_name: String,
}

impl PluginGate {
    /// Create a new PluginGate that delegates to a named plugin.
    pub fn new(loader: PluginLoader, plugin_name: &str) -> Self {
        PluginGate {
            loader,
            plugin_name: plugin_name.to_string(),
        }
    }

    /// Validate input through the wrapped plugin.
    pub fn validate(&mut self, input: &str, context: &str) -> PluginResult {
        match self.loader.get_mut(&self.plugin_name) {
            Some(plugin) => plugin.validate(input, context).unwrap_or_else(|_| {
                PluginResult::fail(0.0, &format!("plugin '{}' error", self.plugin_name))
            }),
            None => PluginResult::fail(0.0, &format!("plugin '{}' not loaded", self.plugin_name)),
        }
    }

    /// Get a reference to the underlying loader.
    pub fn loader(&self) -> &PluginLoader {
        &self.loader
    }

    /// Get a mutable reference to the underlying loader.
    pub fn loader_mut(&mut self) -> &mut PluginLoader {
        &mut self.loader
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_loader_new() {
        let loader = PluginLoader::new();
        assert!(loader.is_empty());
        assert_eq!(loader.len(), 0);
    }

    #[test]
    fn test_plugin_result_pass() {
        let r = PluginResult::pass(0.95, "all good");
        assert!(r.passed);
        assert!((r.score - 0.95).abs() < 0.001);
        assert_eq!(r.message, "all good");
    }

    #[test]
    fn test_plugin_result_fail() {
        let r = PluginResult::fail(0.2, "nope");
        assert!(!r.passed);
        assert!((r.score - 0.2).abs() < 0.001);
    }

    #[test]
    fn test_plugin_type_as_str() {
        assert_eq!(PluginType::Math.as_str(), "math");
        assert_eq!(PluginType::Logic.as_str(), "logic");
        assert_eq!(PluginType::Fact.as_str(), "fact");
        assert_eq!(PluginType::Confidence.as_str(), "confidence");
        assert_eq!(PluginType::Custom("foo".into()).as_str(), "foo");
    }

    #[test]
    fn test_plugin_meta_roundtrip() {
        let meta = PluginMeta {
            name: "test-plugin".into(),
            version: "0.1.0".into(),
            plugin_type: PluginType::Math,
            author: "test".into(),
            description: "a test".into(),
        };
        assert_eq!(meta.name, "test-plugin");
        assert_eq!(meta.plugin_type, PluginType::Math);
    }
}
