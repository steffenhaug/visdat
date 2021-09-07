#version 430 core

in vec3 v_position;
out vec4 color;

float line_1(float x) { return -1.6 * x + 0.5; }
float line_2(float x) { return  1.6 * x + 0.5; }
float line_3(float x) { return  0.0 * x - 0.35; }

vec3 tricol = vec3(0.2, 0.4, 0.7);

void main()
{
    color = vec4(0.5, 0.1, 0.9, 1.0);
    // float draw = float(
    //     line_1(v_position.x) >= v_position.y
    //     && line_2(v_position.x) >= v_position.y
    //     && line_3(v_position.x) <= v_position.y
    // );
    // color = vec4(
    //     max(draw * tricol.x, float(!bool(draw)) * -(-2+v_position.x + v_position.y) / 2),
    //     max(draw * tricol.y, float(!bool(draw)) * v_position.x),
    //     max(draw * tricol.z, float(!bool(draw)) * v_position.y),
    //     1.0f
    // );
}