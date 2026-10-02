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
    let night = background & 0xFFFFFF == 0x111B46;
    let space = background & 0xFFFFFF == 0x04071A;
    let stadium = background & 0xFFFFFF == 0x060A18;
    let horizon = if space {
        Vec3::new(0.008, 0.012, 0.035)
    } else if stadium {
        Vec3::new(0.002, 0.004, 0.012)
    } else if night {
        Vec3::new(0.035, 0.06, 0.16)
    } else if sunset {
        Vec3::new(0.95, 0.42, 0.22)
    } else {
        Vec3::new(
            fallback_red * 0.9 + 0.12,
            fallback_green * 0.9 + 0.18,
            fallback_blue * 0.9 + 0.22,
        )
    };
    let zenith = if space {
        Vec3::new(0.001, 0.002, 0.012)
    } else if stadium {
        Vec3::new(0.0005, 0.001, 0.004)
    } else if night {
        Vec3::new(0.008, 0.015, 0.06)
    } else if sunset {
        Vec3::new(0.12, 0.16, 0.4)
    } else {
        Vec3::new(0.08, 0.32, 0.72)
    };
    let height = direction.y.clamp(-0.15, 1.0);
    let sky_factor = ((height + 0.15) / 1.15).powf(0.72);
    let mut sky = horizon * (1.0 - sky_factor) + zenith * sky_factor;

    let sun_direction = if space {
        normalize(&Vec3::new(-0.3, 0.5, 0.15))
    } else if stadium {
        normalize(&Vec3::new(-0.42, 0.82, 0.32))
    } else if night {
        normalize(&Vec3::new(-0.35, 0.58, 0.2))
    } else if sunset {
        normalize(&Vec3::new(-0.45, 0.42, 0.35))
    } else {
        normalize(&Vec3::new(-0.45, 0.78, 0.35))
    };
    let sun_dot = dot(&direction, &sun_direction).max(0.0);
    let sun_glow = sun_dot.powf(32.0) * 0.18;
    let sun_disc = sun_dot.powf(520.0) * 1.2;
    let sky_light = if stadium {
        Vec3::new(0.18, 0.3, 0.58)
    } else if night {
        Vec3::new(0.45, 0.68, 1.0)
    } else {
        Vec3::new(1.0, 0.72, 0.35)
    };
    sky += sky_light * (sun_glow + sun_disc);
    if night || space {
        let star_wave =
            (direction.x * 91.0).sin() * (direction.y * 117.0).cos() * (direction.z * 73.0).sin();
        if star_wave > 0.94 {
            sky += Vec3::new(0.24, 0.3, 0.5) * ((star_wave - 0.94) * 8.0).min(0.42);
        }
    }
    if space {
        let lava_factor = ((-direction.y - 0.04) / 0.52).clamp(0.0, 1.0);
        if lava_factor > 0.0 {
            let wave = (direction.x * 22.0 + direction.z * 11.0).sin()
                * (direction.x * 8.0 - direction.z * 17.0).cos();
            let pulse = wave * 0.5 + 0.5;
            let lava = Vec3::new(
                0.32 + pulse * 0.5,
                0.015 + pulse * 0.07,
                0.004 + pulse * 0.008,
            );
            sky = sky * (1.0 - lava_factor) + lava * lava_factor;
        }
    }

    let cloud_wave = (direction.x * 11.0 + direction.z * 7.0).sin()
        * (direction.x * 4.0 - direction.z * 9.0).cos();
    let cloud_band = (cloud_wave * 0.5 + 0.5) * (1.0 - (direction.y - 0.2).abs() * 2.8);
    let cloud_amount =
        cloud_band.clamp(0.0, 1.0).powf(5.0) * if night || space || stadium { 0.0 } else { 0.16 };
    sky * (1.0 - cloud_amount) + Vec3::new(0.62, 0.72, 0.92) * cloud_amount
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

    let light_direction = if scene.background & 0xFFFFFF == 0x060A18 {
        normalize(&Vec3::new(-0.42, 0.82, 0.32))
    } else if scene.background & 0xFFFFFF == 0x111B46 {
        normalize(&Vec3::new(-0.5, 0.82, 0.35))
    } else if scene.background & 0xFFFFFF == 0xD66B45 {
        normalize(&Vec3::new(-0.72, 0.58, 0.48))
    } else {
        normalize(&Vec3::new(-0.6, 1.0, 0.8))
    };
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
    let ambient = if scene.background & 0xFFFFFF == 0x060A18 {
        0.13
    } else if scene.background & 0xFFFFFF == 0x111B46 {
        0.16
    } else if scene.background & 0xFFFFFF == 0xD66B45 {
        0.2
    } else {
        0.24
    };
    let direct = diffuse
        * visible_lights
        * if scene.background & 0xFFFFFF == 0x060A18 {
            0.7
        } else if scene.background & 0xFFFFFF == 0x111B46 {
            0.62
        } else {
            0.8
        };
    let navi_distance = scene
        .navi_light_position
        .map(|position| (hit.point - position).magnitude())
        .unwrap_or(99.0);
    let navi_light = if scene.background & 0xFFFFFF == 0x111B46 {
        (1.0 - navi_distance / 3.0).max(0.0) * 1.35
    } else {
        0.0
    };
    let local = Vec3::new(
        texture[0] * hit.material.albedo[0] * (ambient + direct)
            + specular
            + hit.material.emission[0]
            + navi_light * 0.18,
        texture[1] * hit.material.albedo[1] * (ambient + direct) * 0.9
            + specular * 0.84
            + hit.material.emission[1]
            + navi_light * 0.28,
        texture[2] * hit.material.albedo[2] * (ambient + direct) * 0.72
            + specular * 0.65
            + hit.material.emission[2]
            + navi_light * 0.4,
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
