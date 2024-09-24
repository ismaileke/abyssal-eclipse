#version 330 core
layout (location=0) in vec3 inPosition;
layout (location=1) in vec2 inTextureCoordinate;

uniform mat4 u_matrix_projection;
uniform mat4 u_matrix_camera;
uniform mat4 u_matrix_transform;

out vec3 textureCoordinate;

void main()
{
    textureCoordinate = inPosition;
    gl_Position = u_matrix_projection * u_matrix_camera * u_matrix_transform * vec4(inPosition, 1.0);
}