#version 430 core

in vec2 v_position;
out vec4 color;

void main()
{
    /* Zoomed polar coordinates. */
    float zoom = 0.05;
    float rho  = length(v_position.xy) / zoom;
    float phi  = atan(v_position.y, v_position.x);

    /* Spiralling gradient. */
    vec3  c1  = vec3(0.55, 0.90, 0.60);
    vec3  c2  = vec3(0.90, 0.17, 0.57);

    /* transform phi -> [0, 1] continuous at 1->0 */
    float k = 2.0 * distance(0.5 + phi / 6.28, 0.5);
    vec3  c = mix(c1, c2, k);

    /* Gradient background. */
    color = vec4(c, 1.0);

    /* Check N revolutions. */
    int N = 10;
    for (int i = 0; i < N; i++) {
        float delta = rho - phi - (6.28 * float(i));

        /* Assumes fixed resolution 800x800. */
        vec2 uv = gl_FragCoord.xy / 800.0;
        float M = 100;
        float p = mod(M*uv.x, 2);
        float q = mod(M*uv.y, 2);

        float Q = 1.0; /* white */
        /* p,q both > 1 or both < 1 => black. */
        if (p > 1.0 && q > 1.0 || p < 1.0 && q < 1.0) {
            Q = 0.0;
        }

        float epsilon = 1.0;
        if (abs(delta) < epsilon) {
            /* Checker pattern. */
            color = vec4(vec3(Q), 1.0);

            return;
        }
    }
}
