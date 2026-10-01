use crate::camera::Camera;
use crate::framebuffer::Framebuffer;
use crate::ray::Ray;
use crate::scene::Scene;
use nalgebra_glm::{dot, normalize, Vec3};

const MAX_BOUNCES: u32 = 2;

fn background_color(background: u32, direction: Vec3) -> Vec3 {
    let fallback_red = ((background >> 16) & 0xFF) as f32 / 255.0;
    let fallback_green = ((background >> 8) & 0xFF) as f32 / 255.0;
    let fallback_blue = (background & 0xFF) as f32 / 255.0;
    let sunset = background & 0xFFFFFF == 0xD66B45;
    let horizon = if sunset {
        Vec3::new(0.95, 0.32, 0.16)
    } else {
        Vec3::new(
            fallback_red * 0.9 + 0.12,
            fallback_green * 0.9 + 0.18,
            fallback_blue * 0.9 + 0.22,
        )
    };
    let zenith = if sunset {
        Vec3::new(0.16, 0.08, 0.22)
    } else {
        Vec3::new(0.08, 0.32, 0.72)
    };
    let height = direction.y.clamp(-0.15, 1.0);
    let sky_factor = ((height + 0.15) / 1.15).powf(0.72);
    let mut sky = horizon * (1.0 - sky_factor) + zenith * sky_factor;

    let sun_direction = if sunset {
        normalize(&Vec3::new(-0.45, 0.28, 0.35))
    } else {
        normalize(&Vec3::new(-0.45, 0.78, 0.35))
    };
    let sun_dot = dot(&direction, &sun_direction).max(0.0);
    let sun_glow = sun_dot.powf(32.0) * 0.18;
    let sun_disc = sun_dot.powf(520.0) * 1.2;
    sky += Vec3::new(1.0, 0.72, 0.35) * (sun_glow + sun_disc);

    let cloud_wave = (direction.x * 11.0 + direction.z * 7.0).sin()
        * (direction.x * 4.0 - direction.z * 9.0).cos();
    let cloud_band = (cloud_wave * 0.5 + 0.5) * (1.0 - (direction.y - 0.2).abs() * 2.8);
    let cloud_amount = cloud_band.clamp(0.0, 1.0).powf(5.0) * 0.16;
    sky * (1.0 - cloud_amount) + Vec3::new(0.94, 0.96, 1.0) * cloud_amount
}

fn reflect(direction: Vec3, normal: Vec3) -> Vec3 {
    normalize(&(direction - normal * 2.0 * dot(&direction, &normal)))
}

fn refract(direction: Vec3, normal: Vec3, eta_ratio: f32) -> Option<Vec3> {
    let cos_theta = dot(&-direction, &normal).min(1.0);
    let perpendicular = (direction + normal * cos_theta) * eta_ratio;
    let parallel_length = 1.0 - dot(&perpendicular, &perpendicular);
    if parallel_length < 0.0 {
        return None;
    }
    Some(normalize(
        &(perpendicular - normal * parallel_length.sqrt()),
    ))
}

fn schlick(cosine: f32, refractive_index: f32) -> f32 {
    let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn trace_ray(ray: Ray, scene: &Scene, depth: u32) -> Vec3 {
    let background = background_color(scene.background, ray.direction);
    let Some(hit) = scene.intersect(&ray) else {
        return background;
    };

    let light_direction = normalize(&Vec3::new(-0.6, 1.0, 0.8));
    let shadow_origin = hit.point + hit.normal * 0.012;
    let visible_lights = if scene
        .intersect(&Ray::new(shadow_origin, light_direction))
        .is_none()
    {
        1.0
    } else {
        0.0
    };
    let diffuse = dot(&hit.normal, &light_direction).max(0.0);
    let half_vector = normalize(&(light_direction - ray.direction));
    let specular = dot(&hit.normal, &half_vector)
        .max(0.0)
        .powf(hit.material.specular)
        * 0.35
        * visible_lights;

    let texture = hit.material.texture.sample(hit.uv[0], hit.uv[1]);
    let ambient = 0.24;
    let direct = diffuse * visible_lights * 0.76;
    let local = Vec3::new(
        texture[0] * hit.material.albedo[0] * (ambient + direct)
            + specular
            + hit.material.emission[0],
        texture[1] * hit.material.albedo[1] * (ambient + direct) * 0.9
            + specular * 0.84
            + hit.material.emission[1],
        texture[2] * hit.material.albedo[2] * (ambient + direct) * 0.72
            + specular * 0.65
            + hit.material.emission[2],
    );

    if depth >= MAX_BOUNCES {
        return local;
    }

    let reflection_amount = hit.material.reflectivity.clamp(0.0, 1.0);
    let transparency = hit.material.transparency.clamp(0.0, 1.0);
    let mut reflected = Vec3::zeros();
    let mut refracted = Vec3::zeros();
    let mut reflection_weight = reflection_amount;

    if reflection_amount > 0.0 || transparency > 0.0 {
        let front_face = dot(&ray.direction, &hit.normal) < 0.0;
        let normal = if front_face { hit.normal } else { -hit.normal };
        let eta_ratio = if front_face {
            1.0 / hit.material.refractive_index
        } else {
            hit.material.refractive_index
        };
        let cosine = dot(&-ray.direction, &normal).min(1.0);
        let fresnel = schlick(cosine, hit.material.refractive_index);
        reflection_weight = reflection_amount.max(transparency * fresnel);

        if reflection_weight > 0.0 {
            let reflected_ray =
                Ray::new(hit.point + normal * 0.015, reflect(ray.direction, normal));
            reflected = trace_ray(reflected_ray, scene, depth + 1);
        }

        if transparency > 0.0 {
            if let Some(direction) = refract(ray.direction, normal, eta_ratio) {
                let refracted_ray = Ray::new(hit.point - normal * 0.015, direction);
                refracted = trace_ray(refracted_ray, scene, depth + 1);
            } else {
                reflection_weight = 1.0;
            }
        }
    }

    let local_weight = (1.0 - reflection_weight - transparency).max(0.0);
    local * local_weight
        + reflected * reflection_weight
        + refracted * transparency * (1.0 - reflection_weight)
}

pub fn render_raytraced_scene(framebuffer: &mut Framebuffer, camera: &Camera, scene: &Scene) {
    framebuffer.clear(0);
    let aspect = framebuffer.width as f32 / framebuffer.height as f32;
    let fov = std::f32::consts::FRAC_PI_3;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = 2.0 * (x as f32 + 0.5) / framebuffer.width as f32 - 1.0;
            let screen_y = 1.0 - 2.0 * (y as f32 + 0.5) / framebuffer.height as f32;
            let direction = camera.ray_direction(screen_x, screen_y, aspect, fov);
            let ray = Ray::new(camera.eye(), direction);
            let color = trace_ray(ray, scene, 0);
            let red = (color.x.clamp(0.0, 1.0) * 255.0) as u32;
            let green = (color.y.clamp(0.0, 1.0) * 255.0) as u32;
            let blue = (color.z.clamp(0.0, 1.0) * 255.0) as u32;
            framebuffer.set_pixel(x, y, (red << 16) | (green << 8) | blue);
        }
    }
}
