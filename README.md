# Ray Tracing in One Weekend - Rust

![Final scene render](/images/image.jpg)

My implementation in Rust of the [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
book.

It has a few additions:
 - The raytracer is multithreaded, using [scoped
   threads](https://doc.rust-lang.org/stable/std/thread/fn.scope.html)
 - There is an alternative SIMD implementation of Vec3, which sadly is slighly
   slower than the scalar version

##  SIMD vec3

By default, Vec3 is ScalarVec3 under the hood. To use SimdVec3, the SIMD
implementation, you need a nightly version of rust and to enable the "simd"
feature. For example:

```
cargo +nightly run --release --features simd
```
