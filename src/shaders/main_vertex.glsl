#version 330 core
layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 inTextureCoordinate;

out vec2 textureCoordinate;

uniform mat4 u_matrix_projection;
uniform mat4 u_matrix_camera;
uniform mat4 u_matrix_transform;


void main()
{
    textureCoordinate = inTextureCoordinate;
    gl_Position = u_matrix_projection * u_matrix_camera * u_matrix_transform * vec4(Position, 1.0);
}