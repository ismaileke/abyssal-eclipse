#version 330 core

out vec4 Color;

in vec2 textureCoordinate;

uniform sampler2D custom_texture;

void main()
{
    Color = texture(custom_texture, textureCoordinate);
}