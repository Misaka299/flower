const vec2 verts[4] = vec2[4](
vec2(1.0f, 1.0f),
vec2(1.0f, -1.0f),
vec2(-1.0f, -1.0f),
vec2(-1.0f, 1.0f)
);
const vec2 coords[4] = vec2[4](
vec2(1.0, 0.0),
vec2(1.0, 1.0),
vec2(0.0, 1.0),
vec2(0.0, 0.0)
);


in vec4 color;

out vec2 vert;
out vec2 coord;

void main() {
    vert = verts[gl_VertexID];
    coord = coords[gl_VertexID];
    gl_Position = vec4(vert, 0.0, 1.0);
}


//const vec2 verts[4] = vec2[4](
//    vec2(0.0f, 0.0f),
//    vec2(0.0f, 1.0f),
//    vec2(1.0f, 1.0f),
//    vec2(1.0f, 0.0f)
//);
//out vec2 vert;
//void main() {
//    vert = verts[gl_VertexID];
//    gl_Position = vec4(0.0, 0.0, 1.0, 1.0);
//}