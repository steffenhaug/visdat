#version 430 core

in vec3 position;
out vec2 v_position;

void main()
{
    v_position = position.xy;
    gl_Position = vec4(-position.x, -position.y, position.z, 1.0f);
}
