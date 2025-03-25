pub struct TriangularPdf {
    a: u32,
    c: u32,
    b: u32,
    ba: u32,
    ca: u32,
    bc: u32,
}

impl TriangularPdf {
    pub fn new(a: u32, c: u32, b: u32) -> Self {
        Self {
            a,
            c,
            b,
            ba: b - a,
            ca: c - a,
            bc: b - c,
        }
    }

    pub fn pdf(&self, x: u32) -> f32 {
        if x < self.a {
            return 0.0;
        }
        if x < self.c {
            return (2 * (x - self.a)) as f32 / ((self.ba) * (self.ca)) as f32;
        }
        if x == self.c {
            return 2.0 / (self.ba) as f32;
        }
        if x <= self.b {
            return (2 * (self.b - x)) as f32 / ((self.ba) * (self.bc)) as f32;
        }

        0.0
    }
}

pub struct TriangularPdfSampler {
    a: f32,
    b: f32,
    fc: f32, // (c - a) / (b - a)
    ba: f32, // (b - a)
    ca: f32, // (c - a)
    bc: f32, // (b - c)
    skew: bool,
}

impl TriangularPdfSampler {
    pub fn new(a: f32, c: f32, b: f32, skew: bool) -> Self {
        // assert!(a < c && c < b, "Parameters must satisfy a < c < b.");
        let ba = b - a;
        let ca = c - a;
        let bc = b - c;
        let fc = ca / ba;
        Self {
            a,
            b,
            fc,
            ba,
            ca,
            bc,
            skew,
        }
    }

    pub fn sample<T: rand::Rng>(&self, rng: &mut T) -> u32 {
        let rand = {
            let rand: f32 = rng.random();
            // Try to match the incorrect distribution of the assignment.
            // Taking the cube root of the random number seems to decently approximate it.
            if self.skew { rand.cbrt() } else { rand }
        };

        let result = if rand < self.fc {
            // Invert the CDF for the rising edge:
            //   u = ((x - a)^2) / ((b - a)(c - a))
            //   x = a + sqrt(u * (b - a)(c - a))
            self.a + (rand * self.ba * self.ca).sqrt()
        } else {
            // Invert the CDF for the falling edge:
            //   u = 1 - ((b - x)^2) / ((b - a)(b - c))
            //   x = b - sqrt((1 - u) * (b - a)(b - c))
            self.b - ((1.0 - rand) * self.ba * self.bc).sqrt()
        };

        // Convert the continuous sample to an integer.
        result.round() as u32
    }
}
