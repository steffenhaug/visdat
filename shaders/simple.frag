#version 430 core

in vec2 v_position;
out vec4 color;

void main()
{
    if (abs(v_position.y - 0.5 * sin(5.0 * v_position.x)) < .02) {
        color = vec4(v_position, 1.0f, 1.0f);
    } else {
        color = vec4(0);
    }
}
