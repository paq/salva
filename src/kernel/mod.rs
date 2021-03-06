//! Smoothing kernels.

pub use self::cubic_spline_kernel::CubicSplineKernel;
pub use self::kernel::Kernel;
pub use self::poly6_kernel::Poly6Kernel;
pub use self::spiky_kernel::SpikyKernel;
pub use self::viscosity_kernel::ViscosityKernel;

mod cubic_spline_kernel;
mod kernel;
mod poly6_kernel;
mod spiky_kernel;
mod viscosity_kernel;
