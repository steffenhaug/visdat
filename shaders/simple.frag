#version 430 core

in vec3 v_position;
in vec4 v_col;

out vec4 color;

void main() {
    color = v_col;
}
