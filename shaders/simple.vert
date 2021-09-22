#version 430 core

in vec3 position;
in vec4 color;

out vec3 v_position;
out vec4 v_col;

uniform mat4 u_MVP;

void main() {
    /* pass on the position and color */
    v_position = position;
    v_col = color;

    /* place the vertex */
    gl_Position = u_MVP * vec4(position, 1.0f);
}
