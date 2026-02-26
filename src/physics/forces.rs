use crate::space::Body;

pub fn newton_gravity_force(a: &Body, b: &Body, g: f64) -> (f64, f64) {
    let dx = b.cx - a.cx;
    let dy = b.cy - a.cy;

    let r2 = dx * dx + dy * dy;

    if r2 < 1e-9 {
        return (0.0, 0.0);
    }

    let r = r2.sqrt();

    let f = g * a.mass * b.mass / (r * r);

    // normalizacja
    let ux = dx / r;
    let uy = dy / r;

    (f * ux, f * uy)
}

pub fn nbody_step(bodies: &mut [Body], g: f64, dt: f64) {
    let n = bodies.len();

    if n < 2 {
        return;
    }

    let mut ax = vec![0.0; n];
    let mut ay = vec![0.0; n];

    for i in 0..n {
        for j in (i + 1)..n {
            let (fx, fy) = newton_gravity_force(&bodies[i], &bodies[j], g);

            // a = F/m
            ax[i] += fx / bodies[i].mass;
            ay[i] += fy / bodies[i].mass;

            ax[j] += -fx / bodies[j].mass;
            ay[j] += -fy / bodies[j].mass;
        }
    }

    for i in 0..n {
        let b = &mut bodies[i];

        // v(t + dt) = v(t) + a * dt
        b.vx += ax[i] * dt;
        b.vy += ay[i] * dt;

        // x(t + dt) = x(t) + v(t + dt) * dt
        b.cx += b.vx * dt;
        b.cy += b.vy * dt;
    }
}

#[cfg(test)]

mod ut {
    use super::*;
    #[test]
    fn newton_gravity_force_calculation() {
        let a = Body::new([2.0, 2.0], [0.0, 0.0], 10.0, 10.0);
        let b = Body::new([8.0, 10.0], [0.0, 0.0], 10.0, 10.0);
        let g = 1.0;
        assert_eq!((0.6, 0.8), newton_gravity_force(&a, &b, g));
    }
}
