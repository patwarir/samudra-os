use wasmi::*;

pub struct KernelWasmModule<HostState> {
    module: Module,
    store: Store<HostState>,
    linker: Linker<HostState>,
}

impl<HostState> KernelWasmModule<HostState> {
    /* private */
    const KERNEL_MODULE_NAME: &str = "env";

    /* private */
    fn get_default_config() -> Config {
        let mut config = Config::default();
        config.floats(true);
        config.wasm_bulk_memory(true);
        config.wasm_extended_const(false);
        config.wasm_multi_value(true);
        config.wasm_mutable_global(true);
        config.wasm_reference_types(true);
        config.wasm_saturating_float_to_int(true);
        config.wasm_sign_extension(true);
        config.wasm_tail_call(true);
        config
    }

    pub fn new(initial_state: HostState, stream: impl Read) -> Self {
        let config = Self::get_default_config();
        let engine = Engine::new(&config);

        let module = Module::new(&engine, stream).unwrap();
        let store = Store::<HostState>::new(&engine, initial_state);
        let linker = Linker::<HostState>::new(&engine);

        Self {
            module,
            store,
            linker,
        }
    }

    pub fn define<Params, Results>(&mut self, name: &str, func: impl IntoFunc<HostState, Params, Results>) {
        let function = Func::wrap(&mut self.store, func);
        self.linker.define(Self::KERNEL_MODULE_NAME, name, function).unwrap();
    }

    pub fn run<Params: WasmParams, Results: WasmResults>(&mut self, name: &str, params: Params) -> Results {
        let instance_pre = self.linker.instantiate(&mut self.store, &self.module).unwrap();
        let instance = instance_pre.ensure_no_start(&mut self.store).unwrap();

        let func = instance.get_typed_func::<Params, Results>(&mut self.store, name).unwrap();

        func.call(&mut self.store, params).unwrap()
    }
}
