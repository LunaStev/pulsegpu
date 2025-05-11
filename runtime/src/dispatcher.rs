use pulsegpu_ir::parallel_ir::GpuParallelIR;

pub fn dispatch(ir: GpuParallelIR) {
    println!("🚀 Dispatching {} threads (block: {})", ir.threads, ir.block_size);
    println!("Body:\n{}", ir.body);
}