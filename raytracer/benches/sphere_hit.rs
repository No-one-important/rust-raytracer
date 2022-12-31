use criterion::{black_box, criterion_group, criterion_main, Criterion};
use raytracer::sphere::Sphere;
use raytracer::materials::Glass;
use raytracer::materials::Material;
use raytracer::point3d::Point3D;
use raytracer::ray::Hittable;
use raytracer::ray::Ray;



pub fn criterion_benchmark(c: &mut Criterion) {
    let center = Point3D::new(0.0, 0.0, 0.0);
    let sphere = Sphere::new(center, 1.0, Material::Glass(Glass::new(1.5)));
    let ray = Ray::new(Point3D::new(0.0, 0.0, -5.0), Point3D::new(0.0, 0.0, 1.0));

    c.bench_function("sphere_hit", |b| b.iter(|| sphere.hit(black_box(&ray), black_box(0.0), black_box(f64::INFINITY))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);