# Ray Tracing in One Weekend - Rust

My implementation in Rust of [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

##  SIMD vec3

By default, Vec3 is ScalarVec3 under the hood. To use SimdVec3, the SIMD
implementation, you need a nightly version of rust and to enable the "simd"
feature. For example:

```
cargo +nightly run --release --features simd
```
