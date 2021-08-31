#version 430 core

in vec3 position;
out vec3 v_position;

uniform mat4 view;
uniform mat4 model;
uniform mat4 perspective;
uniform mat4 rotate;

void main()
{
    mat4 modelview = view * model * rotate;
    gl_Position = perspective * modelview * vec4(position, 1.0f);
    v_position = gl_Position.xyz / gl_Position.w;
}