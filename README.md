~~~rust
#![feature(alloc_error_handler)]

#[global_allocator]
static GLOBAL: freertos_alloc::Freertosllocator =
    freertos_alloc::Freertosllocator;

#[alloc_error_handler]
fn alloc_error(_layout: core::alloc::Layout) -> ! {
	// ..
}
~~~
