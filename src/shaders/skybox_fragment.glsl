#version 330 core

out vec4 Color;

in vec3 textureCoordinate;

uniform samplerCube ourTexture;

void main() {
    Color = texture(ourTexture, textureCoordinate);
}