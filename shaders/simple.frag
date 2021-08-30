#version 430 core

in vec2 v_position;
out vec4 color;

void main()
{
    color = vec4(v_position, 1.0f, 1.0f);
}