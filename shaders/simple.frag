#version 430 core

in vec2 v_position;
out vec4 color;

void main()
{
    /* Zoomed polar coordinates. */
    float zoom = 0.05;
    float rho  = length(v_position.xy) / zoom;
    float phi  = atan(v_position.y, v_position.x);

    /* Background gradient. */
    vec2 uv   = 0.5 * (v_position.xy + 1.0);
    color = vec4(uv.x-0.1*uv.y, 0.0, uv.y+0.1*uv.x, 1.0);

    /* Spiralling gradient. */
    float psi = (3.14 + phi) / 6.28;

    vec3  c1  = vec3(0.55, 0.90, 0.60);
    vec3  c2  = vec3(0.90, 0.17, 0.57);
    /* trick: the distance to .5 is conitnuos at 1->0. */
    float k   = 2 * distance(psi, 0.5);
    vec3  c   = mix(c1, c2, k);

    /* Check N revolutions. */
    int N = 10;
    for (int i = 0; i < N; i++) {
        float delta = rho - phi - (6.28 * float(i));

        float epsilon = 1.0;

        if (abs(delta) < epsilon) {
            color = vec4(c, 1.0);
            return;
        }
    }
}
