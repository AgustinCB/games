#version 410 core
out vec4 FragColor;

in vec2 TexCoord;

#include "material.glsl"
uniform Material material;

void main()
{
    FragColor = texture(material.diffuse, TexCoord);
}
