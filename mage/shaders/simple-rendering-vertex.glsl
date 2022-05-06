#version 410 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoord;

layout (std140) uniform Matrices {
	mat4 view;
	mat4 projection;
};
uniform mat4 model;

out vec2 TexCoord;

void main()
{
	/*
	FragPos = vec3(model * vec4(aPos, 1.0))
	Normal = transpose(inverse(mat3(model))) * aNormal;
	*/
	gl_Position = projection * view *  model * vec4(aPos, 1.0);
	TexCoord = aTexCoord;
}
