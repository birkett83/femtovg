use super::{
    MemAlign,
    WGPUContext,
    WGPUVar,
};

pub trait DeviceExt {}

impl DeviceExt for wgpu::Device {}

pub struct WGPUVecIterator<'a, T: Copy> {
    inner: &'a WGPUVec<T>,
}

impl<'a, T: Copy> WGPUVecIterator<'a, T> {
    fn new(inner: &'a WGPUVec<T>) -> Self {
        Self { inner }
    }
}

pub struct WGPUVec<T: Copy> {
    ctx: WGPUContext,
    inner: wgpu::Buffer,
    len: usize,
    mem_align: MemAlign<T>,
}

impl<T: Copy> WGPUVec<T> {
    pub fn new(ctx: &WGPUContext, capacity: usize) -> Self {
        let mem_align = MemAlign::new(capacity);

        let inner = ctx.device().create_buffer(&wgpu::BufferDescriptor {
            label: None,
             /// Debug label of a buffer. This will show up in graphics debuggers for easy identification.
            // pub label: L,
            /// Size of a buffer.
            // pub size: BufferAddress,
            size: 0,
            /// Usages of a buffer. If the buffer is used in any way that isn't specified here, the operation
            /// will panic.
            // pub usage: BufferUsage,
            usage: wgpu::BufferUsage::COPY_DST,
            /// Allows a buffer to be mapped immediately after they are made. It does not have to be [`BufferUsage::MAP_READ`] or
            /// [`BufferUsage::MAP_WRITE`], all buffers are allowed to be mapped at creation.
            // pub mapped_at_creation: bool,
            mapped_at_creation: true,
        });
        // Self {
        //     cpu: vec![],
        //     gpu:
        // }
        Self {
            ctx: ctx.clone(),
            inner,
            len: 0,
            mem_align,
        }
    }

    pub fn new_index(ctx: &WGPUContext) -> Self {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn extend_from_slice(&mut self, other: &[T]) {
        // self.cpu.extend_from_slice(other);
        todo!()
    }

    pub fn resize(&mut self, capacity: usize) {
        if capacity <= self.capacity() {
            return;
        }
        let mem_align = MemAlign::<T>::new(capacity);
        // let inner = self.device.new_mem(
        //     mem_align,
        //     metal::MTLResourceOptions::CPUCacheModeDefaultCache,
        // );
        // unsafe {
        //     std::ptr::copy(
        //         self.as_ptr(),
        //         // inner.contents() as *mut T,
        //         inner.as_mut_ptr(),
        //         self.len(),
        //     );
        // }
        self.mem_align = mem_align;
        // self.inner = inner;
    }

    pub fn iter(&self) -> WGPUVecIterator<'_, T> {
        WGPUVecIterator::new(self)
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        // self.inner.slice(bounds)
        todo!()
    }

    pub fn capacity(&self) -> usize {
        self.mem_align.capacity
    }

    pub fn upload(&mut self) {
        // self.gpu.destroy()
        todo!()
    }

    pub fn as_slice<S: std::ops::RangeBounds<wgpu::BufferAddress>>(&self, bounds: S) -> wgpu::BufferSlice {
        self.inner.slice(bounds)
    }

    pub fn slice(&self) -> wgpu::BufferSlice {
        todo!()
    }

    pub fn clear(&mut self) {
        todo!()
    }

    // pub fn as_slice<'a>(&'a self) -> wgpu::BufferSlice<'a> {
    //     // self.gpu.slice(0..0)
    //     todo!()
    // }

    // pub fn as_mut_slice<'a>(&'a mut self) -> wgpu::BufferMutSlice<'a> {
    //     todo!()
    // }
}

impl<T: Copy> Drop for WGPUVec<T> {
    fn drop(&mut self) {
        self.inner.destroy()
    }
}

impl WGPUVec<u32> {
    pub fn extend_with_triange_fan_indices_cw(&mut self, start: u32, count: u32) -> usize {
        let mut added = 0;
        for index in 1..(count - 1) {
            self.extend_from_slice(&[start, start + index, start + index + 1]);
            added += 3;
        }

        added
    }
}

impl<T: Copy> AsRef<wgpu::Buffer> for WGPUVec<T> {
    fn as_ref(&self) -> &wgpu::Buffer {
        &self.inner
    }
}
